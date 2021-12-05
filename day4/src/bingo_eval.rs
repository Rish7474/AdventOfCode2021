use std::collections::HashMap;
use std::collections::HashSet;

const BOARD_SIZE : u32 = 5;

pub struct BingoBoard {
    id: i32,
    row_markings: Vec<u32>,
    col_markings: Vec<u32>,
    unmarked_sum: u32,
    board_map: HashMap<u32, (u32, u32)>
}

pub fn init_bingo_attributes(bingo_data: &Vec<&str>) -> (Vec<u32>, Vec<BingoBoard>) {
    let bingo_moves: Vec<u32> = bingo_data[0].split(",").map(|s| s.parse::<u32>().unwrap()).collect();
    let mut bingo_boards: Vec<BingoBoard> = Vec::new();
    let mut board_id: i32 = 0;

    for board_encoding in (&bingo_data[1..]).chunks(BOARD_SIZE.try_into().unwrap()) {
        let mut bingo_board = BingoBoard {
            id: board_id,
            row_markings: vec![0; BOARD_SIZE.try_into().unwrap()],
            col_markings: vec![0; BOARD_SIZE.try_into().unwrap()],
            unmarked_sum: 0,
            board_map: HashMap::new()
        };

        for r in 0..BOARD_SIZE {
            let row: Vec<u32> = board_encoding[r as usize].split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
            for c in 0..BOARD_SIZE {
                bingo_board.unmarked_sum += row[c as usize];
                bingo_board.board_map.insert(
                    row[c as usize],
                    (r, c),
                );
            }
        }
        board_id += 1;
        bingo_boards.push(bingo_board);
    }

    return (bingo_moves, bingo_boards)
}

pub fn evaluate_bingo_match(bingo_data: &Vec<&str>, ith_win: i32) -> u32 {
    assert_ne!(ith_win, 0);
    let (bingo_moves, mut bingo_boards) = init_bingo_attributes(&bingo_data);
    let num_boards: usize = bingo_boards.len();
    assert!(ith_win <= num_boards as i32);
    let mut winning_boards: HashSet<i32> = HashSet::with_capacity(num_boards);

    for bingo_move in &bingo_moves {
        for bingo_board in & mut bingo_boards {
            match bingo_board.board_map.get(bingo_move) {
                Some(&(row, col)) => {
                    bingo_board.unmarked_sum -= bingo_move;
                    bingo_board.row_markings[row as usize] += 1;
                    bingo_board.col_markings[col as usize] += 1;

                    if bingo_board.row_markings[row as usize] == BOARD_SIZE || bingo_board.col_markings[col as usize] == BOARD_SIZE {
                        winning_boards.insert(bingo_board.id);
                        if winning_boards.len() as i32 == ith_win || (ith_win == -1 && winning_boards.len() as i32 == num_boards as i32) {
                            return bingo_move * bingo_board.unmarked_sum;
                        }
                    }
                },
                _ => {}
            }
        }
    }
    0
}