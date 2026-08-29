//! Tarjan's strongly connected components (SCC) algorithm.
//!
//! Graph-agnostic: nodes are `0..adj.len()`, edges are `adj[v]`. Callers in
//! [`super::storage`] map message types onto those indices.

use ::std::cmp::min;

/// Tarjan (1972): find strongly connected components with one DFS.
///
/// # Input
///
/// Directed graph as an adjacency list. Nodes are `0..adj.len()`.
/// `adj[v]` lists the successors of `v` (outgoing edges). The lists are
/// borrowed only; neighbor rows are not cloned during the walk.
///
/// # Output
///
/// `scc_id` of length `adj.len()`. `scc_id[v]` is an opaque component id;
/// only equality matters (`same_scc` iff `scc_id[a] == scc_id[b]`).
/// The numeric values themselves are not stable across graphs or even
/// equivalent rewrites of the same graph.
///
/// # Example
///
/// Graph `0 ⇄ 1 → 2`:
///
/// ```text
/// adj = [ [1], [0, 2], [] ]
/// ```
///
/// `0` and `1` share an id (they reach each other). `2` gets a different id
/// (reachable from the cycle, but not the other way). One possible result is
/// `[1, 1, 0]`.
///
/// A chain `0 → 1 → 2` (`[[1], [2], []]`) yields three distinct ids.
///
/// # Algorithm
///
/// DFS from every unvisited node. Nodes on the current search path sit on a
/// stack. `indices[v]` is the DFS discovery time. `lowlink[v]` is the earliest
/// discovery time reachable from `v` without leaving the stack (tree edges +
/// back edges to ancestors). When `lowlink[v] == indices[v]`, `v` is the root
/// of an SCC: pop the stack until `v` and give those nodes the same id. Cross
/// edges to already-popped nodes are ignored (they belong to a finished
/// component).
pub(super) fn tarjan(adj: &[Vec<usize>]) -> Vec<usize> {
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

/// Mutable DFS state for [`tarjan`].
struct Tarjan<'a> {
    /// Adjacency list: `adj[v]` = successor indices.
    adj: &'a [Vec<usize>],
    /// Next discovery time (`indices` value) to hand out.
    dfs_index: usize,
    /// Next SCC id to assign when a component is popped.
    next_scc: usize,
    /// Nodes on the current DFS path that are not yet assigned to an SCC.
    stack: Vec<usize>,
    /// Discovery time, or `None` if `v` has not been visited.
    indices: Vec<Option<usize>>,
    /// Smallest discovery time reachable from `v` via stack nodes.
    lowlink: Vec<usize>,
    /// Whether `v` is still on [`Self::stack`] (back-edge vs finished SCC).
    on_stack: Vec<bool>,
    /// Output: component id per node.
    scc_id: Vec<usize>,
}

impl Tarjan<'_> {
    /// Visit `v` and recurse into unvisited children (Tarjan's `strongconnect`).
    fn strongconnect(&mut self, v: usize) {
        self.indices[v] = Some(self.dfs_index);
        self.lowlink[v] = self.dfs_index;
        self.dfs_index += 1;
        self.stack.push(v);
        self.on_stack[v] = true;
        // Copy the slice header so the iterator borrows `adj`, not `self`.
        let adj = self.adj;
        for &w in &adj[v] {
            if self.indices[w].is_none() {
                // Tree edge: recurse, then `v` can reach whatever `w` can.
                self.strongconnect(w);
                self.lowlink[v] = min(self.lowlink[v], self.lowlink[w]);
            } else if self.on_stack[w] {
                // Back edge to an ancestor still on the stack → same SCC.
                self.lowlink[v] = min(self.lowlink[v], self.indices[w].unwrap());
            }
            // Else `w` is already assigned to a finished SCC: ignore.
        }
        // `v` is an SCC root: no ancestor is reachable from here.
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

#[cfg(test)]
mod tests {
    use super::tarjan;
    use ::std::collections::HashSet;

    fn same_component(ids: &[usize], nodes: &[usize]) -> bool {
        let first = ids[nodes[0]];
        nodes.iter().all(|&n| ids[n] == first)
    }

    /// `ids[a] != ids[b]` for every pair (each node is its own SCC).
    fn all_singleton(ids: &[usize]) -> bool {
        let mut seen = HashSet::new();
        ids.iter().all(|id| seen.insert(*id))
    }

    #[test]
    fn empty_and_singletons() {
        assert!(tarjan(&[]).is_empty());
        assert_eq!(tarjan(&[vec![]]), vec![0]);
        assert!(all_singleton(&tarjan(&[vec![], vec![], vec![]])));
    }

    #[test]
    fn self_loop_is_one_component() {
        // Node 0 → 0. Still a single SCC (the node itself).
        let ids = tarjan(&[vec![0]]);
        assert_eq!(ids.len(), 1);
    }

    #[test]
    fn dag_chain_has_distinct_components() {
        // 0 → 1 → 2, no back edges.
        let ids = tarjan(&[vec![1], vec![2], vec![]]);
        assert!(all_singleton(&ids));
    }

    #[test]
    fn mutual_edge_is_one_component() {
        // 0 ↔ 1.
        let ids = tarjan(&[vec![1], vec![0]]);
        assert!(same_component(&ids, &[0, 1]));
    }

    #[test]
    fn three_cycle() {
        // 0 → 1 → 2 → 0.
        let ids = tarjan(&[vec![1], vec![2], vec![0]]);
        assert!(same_component(&ids, &[0, 1, 2]));
    }

    #[test]
    fn two_separate_cycles() {
        // 0 ↔ 1,  2 ↔ 3.
        let ids = tarjan(&[vec![1], vec![0], vec![3], vec![2]]);
        assert!(same_component(&ids, &[0, 1]));
        assert!(same_component(&ids, &[2, 3]));
        assert_ne!(ids[0], ids[2]);
    }

    #[test]
    fn cycle_with_exit_to_sink() {
        // 0 ↔ 1 → 2. {0,1} one SCC, 2 another (cross edge to a distinct node).
        let ids = tarjan(&[vec![1], vec![0, 2], vec![]]);
        assert!(same_component(&ids, &[0, 1]));
        assert_ne!(ids[0], ids[2]);
    }

    #[test]
    fn diamond_dag() {
        // 0 → 1, 0 → 2, 1 → 3, 2 → 3. No cycles.
        let ids = tarjan(&[vec![1, 2], vec![3], vec![3], vec![]]);
        assert!(all_singleton(&ids));
    }
}
