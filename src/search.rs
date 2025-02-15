use crate::board::*;
use crate::heuristics::*;
use crate::min_heap::*;
use std::collections::*;
use std::ops::Not;
use std::time::Duration;

/// Statistics of the search, used to evaluate the performance of the search algorithms.
/// Feel free to add more fields to this struct if you need them.
pub struct Stats {
    /// Numbers of states expanded during search
    pub expanded: usize,
    /// Total runtime spend in the search.
    ///
    /// ```rust
    /// let start_time: Instant = std::time::Instant::now();
    /// // do something
    /// let runtime: Duration = start_time.elapsed();
    /// ```
    pub runtime: Duration,
}

impl Stats {
    /// Creates a new `Stats` instance with the given expanded states count and runtime.
    pub fn new(expanded: usize, runtime: Duration) -> Stats {
        Stats { expanded, runtime }
    }
}

pub fn search(init_state: Board, heuristic: &Heuristic) -> (Option<Vec<Direction>>, Stats) {
    let start = std::time::Instant::now();
    // MinHeap provide allows to store the states to explore, with associated priority
    let mut heap: MinHeap<Board> = MinHeap::new();
    // the standard library provides a HashMap, that can be used to store the cost or other things
    let mut costs: HashMap<Board, u32> = HashMap::new();

    let mut parent_action: HashMap<Board, (Board, Direction)> = HashMap::new();

    let mut expanded: HashSet<Board> = HashSet::new();

    let mut path: HashSet<Board> = HashSet::new();
    let mut directions: Vec<Direction> = Vec::new();

    costs.insert(init_state, 0);
    heap.insert(init_state, 0 + heuristic.estimate(&init_state));

    while !heap.is_empty() {
        let mut s = heap.pop().expect("No node in the heap");

        if expanded.contains(&s) {
            continue;
        }

        if s == Board::GOAL {
            let mut find: bool = false;
            let mut parent: (Board, Direction);
            while !find {
                match parent_action.get(&s) {
                    Some(x) => {
                        parent = *x;
                        path.insert(parent.0);
                        directions.push(parent.1);
                        s = parent.0;
                        if parent.0 == init_state {
                            find = true;
                        }
                    }
                    None => find = true,
                }
            }
        }

        for action in DIRECTIONS {
            let sbis = match s.apply(action) {
                Some(board) => board,
                None => continue,
            };

            let current_cost = costs.get(&s).expect("Cannot find the cost") + 1;

            let found_better_path = match costs.get(&sbis) {
                Some(previous_cost) => current_cost < *previous_cost,
                None => true,
            };

            if found_better_path {
                costs.insert(sbis, current_cost);
                parent_action.insert(sbis, (s, action));
                heap.insert(sbis, current_cost+ heuristic.estimate(&sbis));
            }
        }
        expanded.insert(s);
    }

    directions.reverse();

    // here is an example to measure the runtime and returns the statistics
    let runtime = start.elapsed();
    // example to construct a Stats instance
    let stats = Stats::new(0, runtime);
    // return the results and associated stats
    (Some(directions), stats)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_search() {
        use super::*;

        // validates that search oes return the optimal plan on the first 20 isntances
        for (expected_cost, init) in &INSTANCES[0..20] {
            let (path, stats) = search(*init, &Heuristic::Blind);
            let path = path.expect("no plan");
            assert!(init.is_valid_plan(&path));
            assert_eq!(path.len(), *expected_cost as usize);
        }
    }
}
