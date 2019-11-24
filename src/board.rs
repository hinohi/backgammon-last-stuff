use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Board {
    cell: [u8; 7],
}

impl Board {
    pub const fn new(cell: [u8; 6]) -> Self {
        let c = [0, cell[0], cell[1], cell[2], cell[3], cell[4], cell[5]];
        Board { cell: c }
    }

    pub fn list_moves(&self, dices: &[usize]) -> HashSet<Board> {
        let mut ret = HashSet::new();
        let mut states = vec![(self.clone(), Vec::from(dices))];
        let mut saturated = Vec::new();
        while !states.is_empty() {
            let mut next_states = Vec::new();
            for (board, dices) in states {
                let mut moved = false;
                for (i, &dice) in dices.iter().enumerate() {
                    for j in dice..7 {
                        if board.cell[j] > 0 {
                            let mut board = board.clone();
                            board.cell[j] -= 1;
                            if dice < j {
                                board.cell[j - dice] += 1;
                            }
                            if dices.len() == 1 {
                                ret.insert(board);
                            } else {
                                let mut dices = dices.clone();
                                dices.swap_remove(i);
                                next_states.push((board, dices));
                            }
                            moved = true;
                        }
                    }
                }
                if !moved {
                    saturated.push((board, dices.len()));
                }
            }
            states = next_states;
        }
        for (board, dices) in saturated {
            let mut states = vec![board];
            for _ in 0..dices {
                let mut next_states = Vec::new();
                for board in states {
                    let mut moved = false;
                    for j in 1..7 {
                        if board.cell[j] > 0 {
                            let mut board = board.clone();
                            board.cell[j] -= 1;
                            next_states.push(board);
                            moved = true;
                        }
                    }
                    if !moved {
                        ret.insert(board);
                    }
                }
                states = next_states;
            }
            for board in states {
                ret.insert(board);
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_set(boards: &[Board]) -> HashSet<Board> {
        let mut set = HashSet::with_capacity(boards.len());
        for b in boards {
            set.insert(b.clone());
        }
        set
    }

    #[test]
    fn list_moves_1() {
        let board = Board::new([0, 0, 0, 1, 0, 0]);
        assert_eq!(
            board.list_moves(&[1]),
            as_set(&[Board::new([0, 0, 1, 0, 0, 0])]),
        );
        assert_eq!(
            board.list_moves(&[4]),
            as_set(&[Board::new([0, 0, 0, 0, 0, 0])]),
        );
        assert_eq!(
            board.list_moves(&[5]),
            as_set(&[Board::new([0, 0, 0, 0, 0, 0])]),
        );
        assert_eq!(
            board.list_moves(&[1, 2]),
            as_set(&[Board::new([1, 0, 0, 0, 0, 0])]),
        );
    }

    #[test]
    fn list_moves_2() {
        let board = Board::new([0, 0, 1, 1, 0, 0]);
        assert_eq!(
            board.list_moves(&[1]),
            as_set(&[
                Board::new([0, 1, 0, 1, 0, 0]),
                Board::new([0, 0, 2, 0, 0, 0]),
            ]),
        );
        assert_eq!(
            board.list_moves(&[5]),
            as_set(&[
                Board::new([0, 0, 1, 0, 0, 0]),
                Board::new([0, 0, 0, 1, 0, 0]),
            ]),
        );
        assert_eq!(
            board.list_moves(&[1, 3]),
            as_set(&[
                Board::new([1, 1, 0, 0, 0, 0]),
                Board::new([0, 0, 1, 0, 0, 0]),
            ]),
        );
        assert_eq!(
            board.list_moves(&[4, 3]),
            as_set(&[
                Board::new([0, 0, 0, 0, 0, 0]),
                Board::new([1, 0, 0, 0, 0, 0]),
                Board::new([0, 0, 1, 0, 0, 0]),
            ]),
        );
    }

    #[test]
    fn list_moves_3() {
        let board = Board::new([0, 0, 0, 0, 1, 3]);
        assert_eq!(
            board.list_moves(&[6, 6, 6, 6]),
            as_set(&[Board::new([0, 0, 0, 0, 0, 0])]),
        );
        assert_eq!(
            board.list_moves(&[5, 5, 5, 5]),
            as_set(&[Board::new([3, 0, 0, 0, 0, 0])]),
        );
        assert_eq!(
            board.list_moves(&[3, 3, 3, 3]),
            as_set(&[
                Board::new([0, 0, 0, 0, 1, 1]),
                Board::new([0, 0, 2, 0, 1, 0]),
                Board::new([0, 1, 1, 0, 0, 1]),
                Board::new([0, 1, 3, 0, 0, 0]),
            ]),
        );
    }
}
