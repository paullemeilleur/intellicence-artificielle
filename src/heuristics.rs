use crate::board::*;

/// A heuristic function to estimate the cost of reaching the goal state from a given board.
///
/// ```rust
/// let board = Board::new([[8, 7, 3], [2, 0, 5], [1, 4, 6]]);
/// let h = Heuristic::Manhattan.estimate(&board);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Heuristic {
    /// The blind heuristic always returns 0.
    Blind,
    /// The Hamming heuristic, which counts the number of misplaced tiles.
    Hamming,
    /// The Manhattan heuristic, which computes the sum of the Manhattan distances of each tile to its goal position.
    Manhattan,
}

impl Heuristic {
    pub fn estimate(&self, board: &Board) -> u32 {
        match self {
            // blind heuristic always returns 0
            Heuristic::Blind => 0,
            Heuristic::Hamming => {
                let mut hamming = 0;
                for i in 0..3 {
                    for j in 0..3 {
                        if Board::GOAL.value_at(i, j) != board.value_at(i, j) {
                            if board.value_at(i, j) == 0 {
                                continue;
                            } else {
                                hamming = hamming + 1;
                            }
                        }
                    }
                }
                hamming
            }
            Heuristic::Manhattan => {
                let mut manhattan = 0;
                for i in 1..9 {
                    let (x, y) = board.position(i);
                    let (x_goal, y_goal) = Board::GOAL.position(i);
                    manhattan = manhattan
                        + (x_goal as i32 - x as i32).abs()
                        + (y_goal as i32 - y as i32).abs();
                }
                manhattan.try_into().unwrap() 
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_heuristic() {
        use super::*;
        let board = Board::new([[8, 7, 3], [2, 0, 5], [1, 4, 6]]);
        assert_eq!(Heuristic::Blind.estimate(&board), 0);
        assert_eq!(Heuristic::Hamming.estimate(&board), 7);
        assert_eq!(Heuristic::Manhattan.estimate(&board), 14);
    }
}
