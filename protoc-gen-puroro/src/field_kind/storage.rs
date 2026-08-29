//! FileSet-wide nested-message storage (boxed vs inlined).

use crate::descriptor::MessageLayout;
use crate::descriptor::ProtoFqn;
use crate::resolved::{Field, FieldOccurrence, FileSet, Message, SingularPresence, TypeRef};
use ::std::cmp::min;
use ::std::collections::{HashMap, HashSet};

/// Per-field nested-message storage chosen for this [`FileSet`].
///
/// Missing keys are boxed (oneof / repeated / map / ineligible / default).
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

/// Decide boxed vs inlined storage for every singular non-oneof message field.
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
            if !is_singular_non_oneof_message(field) {
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

fn is_singular_non_oneof_message(field: &Field<'_>) -> bool {
    matches!(
        field.occurrence(),
        FieldOccurrence::Singular(presence) if !matches!(presence, SingularPresence::Oneof)
    ) && matches!(field.type_ref(), TypeRef::Message(_))
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

/// Tarjan SCC ids, one per `messages` index. Isolated nodes (including a
/// self-referential message, whose parent and child share an index) get distinct
/// component ids except when a cycle joins them.
fn scc_ids(messages: &[&Message<'_>]) -> Vec<usize> {
    let n = messages.len();
    let mut index_by_fqn = HashMap::with_capacity(n);
    for (i, m) in messages.iter().enumerate() {
        index_by_fqn.insert(m.fqn(), i);
    }
    let mut adj = vec![Vec::new(); n];
    for (i, owner) in messages.iter().enumerate() {
        for field in owner.fields() {
            if !is_singular_non_oneof_message(field) {
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

fn tarjan(adj: &[Vec<usize>]) -> Vec<usize> {
    let n = adj.len();
    let mut state = Tarjan {
        adj,
        dfs_index: 0,
        next_scc: 0,
        stack: Vec::new(),
        indices: vec![None; n],
        lowlink: vec![0usize; n],
        on_stack: vec![false; n],
        scc_id: vec![0usize; n],
    };
    for v in 0..n {
        if state.indices[v].is_none() {
            state.strongconnect(v);
        }
    }
    state.scc_id
}

struct Tarjan<'a> {
    adj: &'a [Vec<usize>],
    dfs_index: usize,
    next_scc: usize,
    stack: Vec<usize>,
    indices: Vec<Option<usize>>,
    lowlink: Vec<usize>,
    on_stack: Vec<bool>,
    scc_id: Vec<usize>,
}

impl Tarjan<'_> {
    fn strongconnect(&mut self, v: usize) {
        self.indices[v] = Some(self.dfs_index);
        self.lowlink[v] = self.dfs_index;
        self.dfs_index += 1;
        self.stack.push(v);
        self.on_stack[v] = true;
        let neighbors = self.adj[v].clone();
        for w in neighbors {
            if self.indices[w].is_none() {
                self.strongconnect(w);
                self.lowlink[v] = min(self.lowlink[v], self.lowlink[w]);
            } else if self.on_stack[w] {
                self.lowlink[v] = min(self.lowlink[v], self.indices[w].unwrap());
            }
        }
        if self.lowlink[v] == self.indices[v].unwrap() {
            loop {
                let w = self.stack.pop().expect("SCC stack");
                self.on_stack[w] = false;
                self.scc_id[w] = self.next_scc;
                if w == v {
                    break;
                }
            }
            self.next_scc += 1;
        }
    }
}
