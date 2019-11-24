use std::collections::HashMap;

use num::traits::{One, Zero};

use backgammon_last_stuff::{utils::DICE_DATA, BigURatio, Board, ProbDist};

type Prob = ProbDist<BigURatio, BigURatio>;
type DB = HashMap<Board, Prob>;

fn search(db: &mut DB, board: &Board) -> BigURatio {
    if let Some(dist) = db.get(board) {
        return dist.mean();
    }
    let mut dist = ProbDist::new();
    for (dices, n) in DICE_DATA.iter() {
        let mut best = BigURatio::new_from_u32(4294967295, 1);
        for next in board.list_moves(dices) {
            let s = search(db, &next);
            if s < best {
                best = s;
            }
        }
        dist.append(best + BigURatio::one(), BigURatio::new_from_u32(*n, 36));
    }
    db.insert(board.clone(), dist);
    db.get(board).unwrap().mean()
}

struct CellInc {
    i: usize,
    cell: [u8; 6],
}

impl CellInc {
    fn new(i: usize, cell: [u8; 6]) -> CellInc {
        CellInc { i, cell }
    }
}

impl Iterator for CellInc {
    type Item = [u8; 6];
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= 6 {
            None
        } else {
            let mut cell = self.cell.clone();
            cell[self.i] += 1;
            self.i += 1;
            Some(cell)
        }
    }
}

fn main() {
    let mut db = HashMap::new();
    db.insert(Board::new([0; 6]), {
        let mut dist = ProbDist::new();
        dist.append(BigURatio::zero(), BigURatio::new_from_u32(1, 1));
        dist
    });
    for (i, cell) in CellInc::new(0, [0; 6]).enumerate() {
        search(&mut db, &Board::new(cell.clone()));
        for cell in CellInc::new(i, cell) {
            search(&mut db, &Board::new(cell.clone()));
            for cell in CellInc::new(i, cell) {
                search(&mut db, &Board::new(cell.clone()));
                for cell in CellInc::new(i, cell) {
                    search(&mut db, &Board::new(cell.clone()));
                    for cell in CellInc::new(i, cell) {
                        search(&mut db, &Board::new(cell.clone()));
                        for cell in CellInc::new(i, cell) {
                            search(&mut db, &Board::new(cell.clone()));
                            for cell in CellInc::new(i, cell) {
                                search(&mut db, &Board::new(cell.clone()));
                            }
                        }
                    }
                }
            }
        }
    }
    for (board, dist) in db.iter() {
        println!("{} {} {}", board, board.pips(), dist.mean());
    }
}
