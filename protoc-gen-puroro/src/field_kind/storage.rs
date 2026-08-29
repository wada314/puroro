//! FileSet-wide nested-message storage (boxed vs inlined).

use super::tarjan::tarjan;
use crate::descriptor::MessageLayout;
use crate::descriptor::ProtoFqn;
use crate::resolved::{Field, FieldOccurrence, FileSet, Message, SingularPresence, TypeRef};
use ::std::collections::{HashMap, HashSet};

/// Per-field nested-message storage chosen for this [`FileSet`].
///
/// Missing keys are boxed (repeated / map / ineligible / default).
#[derive(Debug, Default)]
pub struct MessageStoragePlan {
    /// Owner FQN → field numbers stored inline in the parent struct.
    inline: HashMap<ProtoFqn, HashSet<i32>>,
}

impl MessageStoragePlan {
    /// Whether `owner`'s singular field `field_number` is inlined.
    pub fn is_inline(&self, owner: &Message<'_>, field_number: i32) -> bool {
        self.inline
            .get(owner.fqn())
            .is_some_and(|fields| fields.contains(&field_number))
    }
}

/// Decide boxed vs inlined storage for every singular message field
/// (including oneof variants).
///
/// Same-SCC edges (self / mutual recursion) always box. `(puroro.message_layout)
/// = BOXED` is always honored; `INLINE` is ignored when illegal. Unspecified
/// uses the auto-inline heuristic (1..=4 Copy scalar / enum fields on the child).
pub fn plan_message_storage<'a>(file_set: &FileSet<'a>) -> MessageStoragePlan {
    let messages = collect_messages(file_set);
    let scc = scc_ids(&messages);
    let mut plan = MessageStoragePlan::default();
    for (idx, owner) in messages.iter().enumerate() {
        for field in owner.fields() {
            if !is_singular_message(field) {
                continue;
            }
            let TypeRef::Message(child) = field.type_ref() else {
                continue;
            };
            if child.is_map_entry() {
                continue;
            }
            let Some(child_idx) = index_of(&messages, child) else {
                continue;
            };
            let same_scc = scc[idx] == scc[child_idx];
            if decide_inline(field, child, same_scc) {
                plan.inline
                    .entry(owner.fqn().clone())
                    .or_default()
                    .insert(field.number());
            }
        }
    }
    plan
}

fn collect_messages<'a>(file_set: &FileSet<'a>) -> Vec<&'a Message<'a>> {
    let mut out = Vec::new();
    fn walk<'a>(m: &'a Message<'a>, out: &mut Vec<&'a Message<'a>>) {
        if !m.is_map_entry() {
            out.push(m);
        }
        for nested in m.nested_messages() {
            walk(nested, out);
        }
    }
    for file in file_set.files() {
        for m in file.messages() {
            walk(m, &mut out);
        }
    }
    out
}

fn index_of<'a>(messages: &[&'a Message<'a>], target: &'a Message<'a>) -> Option<usize> {
    messages.iter().position(|m| m.fqn() == target.fqn())
}

fn is_singular_message(field: &Field<'_>) -> bool {
    matches!(field.occurrence(), FieldOccurrence::Singular(_))
        && matches!(field.type_ref(), TypeRef::Message(_))
}

fn decide_inline(field: &Field<'_>, child: &Message<'_>, same_scc: bool) -> bool {
    if same_scc {
        return false;
    }
    match field.message_layout() {
        Some(MessageLayout::Boxed) => false,
        Some(MessageLayout::Inline) => true,
        Some(MessageLayout::Unspecified) | None => auto_inline_child(child),
    }
}

/// Tiny children: 1..=4 non-repeated Copy scalars / enums, no string / bytes /
/// message / repeated / map / oneof members.
fn auto_inline_child(child: &Message<'_>) -> bool {
    if child.is_map_entry() {
        return false;
    }
    let mut n = 0usize;
    for field in child.fields() {
        if !is_copy_scalar_or_enum(field) {
            return false;
        }
        n += 1;
        if n > 4 {
            return false;
        }
    }
    (1..=4).contains(&n)
}

fn is_copy_scalar_or_enum(field: &Field<'_>) -> bool {
    match field.occurrence() {
        FieldOccurrence::Singular(SingularPresence::Oneof)
        | FieldOccurrence::Repeated(_)
        | FieldOccurrence::Map => false,
        FieldOccurrence::Singular(_) => match field.type_ref() {
            TypeRef::String | TypeRef::Bytes | TypeRef::Message(_) => false,
            TypeRef::Double
            | TypeRef::Float
            | TypeRef::Int64
            | TypeRef::UInt64
            | TypeRef::Int32
            | TypeRef::Fixed64
            | TypeRef::Fixed32
            | TypeRef::Bool
            | TypeRef::UInt32
            | TypeRef::SFixed32
            | TypeRef::SFixed64
            | TypeRef::SInt32
            | TypeRef::SInt64
            | TypeRef::Enum(_) => true,
        },
    }
}

/// Assign a strongly-connected-component (SCC) id to each message.
///
/// The graph is: node = message type, edge `Owner → Child` for each singular
/// field (including oneof variants) whose type is `Child`. Inlining `Child`
/// into `Owner` embeds `Child`'s struct in `Owner` (or in a oneof variant).
/// That is only possible when there is **no cycle** through that edge
/// (`Owner` cannot reach itself by following nested-message fields). Two types
/// are in the same SCC iff each can reach the other (`A → B → A`, or a longer
/// cycle). A self-field `Nest.child: Nest` is the same node as parent and
/// child, so `scc[i] == scc[i]` and we refuse to inline.
///
/// Isolated types (no message edges) each get their own component.
///
/// Implemented with Tarjan's SCC algorithm (one DFS; see [`super::tarjan`]).
fn scc_ids(messages: &[&Message<'_>]) -> Vec<usize> {
    let n = messages.len();
    let mut index_by_fqn = HashMap::with_capacity(n);
    for (i, m) in messages.iter().enumerate() {
        index_by_fqn.insert(m.fqn(), i);
    }
    let mut adj = vec![Vec::new(); n];
    for (i, owner) in messages.iter().enumerate() {
        for field in owner.fields() {
            if !is_singular_message(field) {
                continue;
            }
            let TypeRef::Message(child) = field.type_ref() else {
                continue;
            };
            if let Some(&j) = index_by_fqn.get(child.fqn()) {
                adj[i].push(j);
            }
        }
    }
    tarjan(&adj)
}

#[cfg(test)]
mod tests {
    use super::{collect_messages, index_of, plan_message_storage, scc_ids};
    use crate::descriptor::test_helpers as desc;
    use crate::descriptor::{
        FieldDesc, FieldType, MessageDesc, MessageLayout, ProtoFile, ProtoFqn,
    };
    use crate::resolved::{Arena, FileSet, Message, resolve};
    use ::std::collections::HashSet;

    fn proto3_file(messages: Vec<MessageDesc>) -> ProtoFile {
        ProtoFile {
            messages,
            ..desc::proto_file("t.proto", "example")
        }
    }

    fn msg_field(name: &str, number: i32, type_name: &str) -> FieldDesc {
        FieldDesc {
            type_name: Some(ProtoFqn::parse(type_name)),
            ..desc::field(name, number, FieldType::Message)
        }
    }

    fn force_inline(name: &str, number: i32, type_name: &str) -> FieldDesc {
        FieldDesc {
            message_layout: Some(MessageLayout::Inline),
            ..msg_field(name, number, type_name)
        }
    }

    fn message<'a>(set: &'a FileSet<'a>, name: &str) -> &'a Message<'a> {
        set.files()
            .flat_map(|f| f.messages())
            .find(|m| m.name() == name)
            .unwrap_or_else(|| panic!("missing message {name}"))
    }

    fn same_component(ids: &[usize], nodes: &[usize]) -> bool {
        let first = ids[nodes[0]];
        nodes.iter().all(|&n| ids[n] == first)
    }

    /// `ids[a] != ids[b]` for every pair (each node is its own SCC).
    fn all_singleton(ids: &[usize]) -> bool {
        let mut seen = HashSet::new();
        ids.iter().all(|id| seen.insert(*id))
    }

    fn scc_of<'a>(set: &'a FileSet<'a>) -> (Vec<&'a Message<'a>>, Vec<usize>) {
        let messages = collect_messages(set);
        let ids = scc_ids(&messages);
        (messages, ids)
    }

    fn msg_idx<'a>(messages: &[&'a Message<'a>], name: &str) -> usize {
        index_of(messages, message_in(messages, name)).expect("index")
    }

    fn message_in<'a>(messages: &[&'a Message<'a>], name: &str) -> &'a Message<'a> {
        messages
            .iter()
            .copied()
            .find(|m| m.name() == name)
            .unwrap_or_else(|| panic!("missing {name}"))
    }

    #[test]
    fn scc_self_reference_same_component_as_itself() {
        let arena = Arena::new();
        let files = [proto3_file(vec![MessageDesc {
            fields: vec![force_inline("child", 1, ".example.Nest")],
            ..desc::message("Nest")
        }])];
        let set = resolve(&arena, &files).unwrap();
        let (messages, ids) = scc_of(&set);
        let i = msg_idx(&messages, "Nest");
        assert_eq!(ids[i], ids[i]);
        let storage = plan_message_storage(&set);
        assert!(!storage.is_inline(message(&set, "Nest"), 1));
    }

    #[test]
    fn scc_mutual_pair_shares_component() {
        let arena = Arena::new();
        let files = [proto3_file(vec![
            MessageDesc {
                fields: vec![force_inline("b", 1, ".example.B")],
                ..desc::message("A")
            },
            MessageDesc {
                fields: vec![force_inline("a", 1, ".example.A")],
                ..desc::message("B")
            },
        ])];
        let set = resolve(&arena, &files).unwrap();
        let (messages, ids) = scc_of(&set);
        let a = msg_idx(&messages, "A");
        let b = msg_idx(&messages, "B");
        assert_eq!(ids[a], ids[b]);
        let storage = plan_message_storage(&set);
        assert!(!storage.is_inline(message(&set, "A"), 1));
        assert!(!storage.is_inline(message(&set, "B"), 1));
    }

    #[test]
    fn scc_three_cycle_shares_component() {
        let arena = Arena::new();
        let files = [proto3_file(vec![
            MessageDesc {
                fields: vec![force_inline("b", 1, ".example.B")],
                ..desc::message("A")
            },
            MessageDesc {
                fields: vec![force_inline("c", 1, ".example.C")],
                ..desc::message("B")
            },
            MessageDesc {
                fields: vec![force_inline("a", 1, ".example.A")],
                ..desc::message("C")
            },
        ])];
        let set = resolve(&arena, &files).unwrap();
        let (messages, ids) = scc_of(&set);
        let a = msg_idx(&messages, "A");
        let b = msg_idx(&messages, "B");
        let c = msg_idx(&messages, "C");
        assert!(same_component(&ids, &[a, b, c]));
        let storage = plan_message_storage(&set);
        assert!(!storage.is_inline(message(&set, "A"), 1));
        assert!(!storage.is_inline(message(&set, "B"), 1));
        assert!(!storage.is_inline(message(&set, "C"), 1));
    }

    #[test]
    fn scc_dag_chain_distinct_so_force_inline_succeeds() {
        // Leaf has no message fields; INLINE on the chain is legal (not same SCC).
        let arena = Arena::new();
        let files = [proto3_file(vec![
            MessageDesc {
                fields: vec![force_inline("mid", 1, ".example.Mid")],
                ..desc::message("Top")
            },
            MessageDesc {
                fields: vec![force_inline("leaf", 1, ".example.Leaf")],
                ..desc::message("Mid")
            },
            desc::message("Leaf"),
        ])];
        let set = resolve(&arena, &files).unwrap();
        let (messages, ids) = scc_of(&set);
        let top = msg_idx(&messages, "Top");
        let mid = msg_idx(&messages, "Mid");
        let leaf = msg_idx(&messages, "Leaf");
        assert!(all_singleton(&[ids[top], ids[mid], ids[leaf]]));
        let storage = plan_message_storage(&set);
        assert!(storage.is_inline(message(&set, "Top"), 1));
        assert!(storage.is_inline(message(&set, "Mid"), 1));
    }

    #[test]
    fn scc_includes_oneof_edges_and_can_inline() {
        // Oneof A { B b } is a graph edge. No back-edge → distinct SCCs, INLINE
        // is legal (B has no fields, so the hint is what enables inline).
        let arena = Arena::new();
        let files = [proto3_file(vec![
            MessageDesc {
                fields: vec![FieldDesc {
                    oneof_index: Some(0),
                    message_layout: Some(MessageLayout::Inline),
                    ..msg_field("b", 1, ".example.B")
                }],
                oneofs: vec![desc::oneof("choice")],
                ..desc::message("A")
            },
            desc::message("B"),
        ])];
        let set = resolve(&arena, &files).unwrap();
        let (messages, ids) = scc_of(&set);
        let a = msg_idx(&messages, "A");
        let b = msg_idx(&messages, "B");
        assert_ne!(ids[a], ids[b]);
        let storage = plan_message_storage(&set);
        assert!(storage.is_inline(message(&set, "A"), 1));
    }

    #[test]
    fn scc_oneof_mutual_pair_shares_component() {
        // A.oneof { B b } and B.a: A stay boxed (same SCC).
        let arena = Arena::new();
        let files = [proto3_file(vec![
            MessageDesc {
                fields: vec![FieldDesc {
                    oneof_index: Some(0),
                    message_layout: Some(MessageLayout::Inline),
                    ..msg_field("b", 1, ".example.B")
                }],
                oneofs: vec![desc::oneof("choice")],
                ..desc::message("A")
            },
            MessageDesc {
                fields: vec![force_inline("a", 1, ".example.A")],
                ..desc::message("B")
            },
        ])];
        let set = resolve(&arena, &files).unwrap();
        let (messages, ids) = scc_of(&set);
        let a = msg_idx(&messages, "A");
        let b = msg_idx(&messages, "B");
        assert_eq!(ids[a], ids[b]);
        let storage = plan_message_storage(&set);
        assert!(!storage.is_inline(message(&set, "A"), 1));
        assert!(!storage.is_inline(message(&set, "B"), 1));
    }
}
