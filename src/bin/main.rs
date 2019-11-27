use std::collections::HashMap;
use std::env::args;

use num::traits::{One, Zero};

use backgammon_last_stuff::{utils::DICE_DATA, BigURatio, Board, ProbDist};
use std::io::BufRead;

type DB = HashMap<Board, BigURatio>;

fn search(db: &mut DB, board: &Board) -> BigURatio {
    if let Some(mean) = db.get(board) {
        return mean.clone();
    }
    let mut dist = ProbDist::new();
    for (dices, n) in DICE_DATA.iter() {
        let mut best = BigURatio::new_from_u32(4_294_967_295, 1);
        for next in board.list_moves(dices) {
            let s = search(db, &next);
            if s < best {
                best = s;
            }
        }
        dist.append(best + BigURatio::one(), BigURatio::new_from_u32(*n, 36));
    }
    let m = dist.mean();
    db.insert(board.clone(), m.clone());
    m
}

fn cell_loop(db: &mut DB, depth: usize, i: usize, cell: [u8; 6]) {
    if depth == 0 {
        search(db, &Board::new(cell));
    } else {
        for i in i..6 {
            let mut cell = cell;
            cell[i] += 1;
            cell_loop(db, depth - 1, i, cell);
        }
    }
}

fn read_db(name: &str) -> std::io::Result<DB> {
    let re = regex::Regex::new(r#"^\[(\d+),(\d+),(\d+),(\d+),(\d+),(\d+)\]$"#).unwrap();
    let mut db = HashMap::new();
    for line in std::io::BufReader::new(std::fs::File::open(name)?).lines() {
        let line = line?;
        println!("{}", line);
        let mut words = line.split_ascii_whitespace();
        let board = {
            let m = re.captures(words.next().unwrap()).unwrap();
            let mut cell = m
                .iter()
                .skip(1)
                .map(|s| s.unwrap().as_str().parse().unwrap());
            Board::new([
                cell.next().unwrap(),
                cell.next().unwrap(),
                cell.next().unwrap(),
                cell.next().unwrap(),
                cell.next().unwrap(),
                cell.next().unwrap(),
            ])
        };
        let count = {
            let mut s = words.next().unwrap().split('/');
            BigURatio::new(
                s.next().unwrap().parse().unwrap(),
                s.next()
                    .map(|s| s.parse().unwrap())
                    .unwrap_or_else(One::one),
            )
        };
        db.insert(board, count);
    }
    Ok(db)
}

fn main() {
    let mut args = args();
    let depth = args.nth(1).unwrap().parse().unwrap();
    let mut db = match args.next() {
        None => {
            let mut db = HashMap::new();
            db.insert(Board::new([0; 6]), BigURatio::zero());
            db
        }
        Some(name) => read_db(&name).unwrap(),
    };
    cell_loop(&mut db, depth, 0, [0; 6]);
    for (board, mean) in db.iter() {
        println!("{} {}", board, mean);
    }
}
