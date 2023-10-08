use std::io;
use colored::Colorize;

pub const SIZE: usize = 4;
pub type Board = [[u32; SIZE]; SIZE];

// Initialize the board with numbers from 1 to 15 and an empty space
pub fn initialize_board() -> Board {
    let mut board = [[0; SIZE]; SIZE];
    let mut num = 1;

    for i in 0..SIZE {
        for j in 0..SIZE {
            board[i][j] = num;
            num += 1;
        }
    }
    board[SIZE - 1][SIZE - 1] = 0;
    board
}

// Board Shuffle logic to start the game
pub fn shuffle(board: &mut Board, moves: usize) {
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut empty = (SIZE - 1, SIZE - 1);

    for _ in 0..moves {
        let (ex, ey) = empty;
        let available_moves: Vec<_> = dirs.iter()
            .filter_map(|&(dx, dy)| {
                let nx = (ex as isize + dx) as usize;
                let ny = (ey as isize + dy) as usize;
                if nx < SIZE && ny < SIZE {
                    Some((nx, ny))
                } else {
                    None
                }
            })
            .collect();

        let &(nx, ny) = available_moves.get(rand::random::<usize>() % available_moves.len()).unwrap();
        board[ex][ey] = board[nx][ny];
        board[nx][ny] = 0;
        empty = (nx, ny);
    }
}

// Check if the board is solved
pub fn is_solved(board: &Board) -> bool {
    let mut expected = 1;
    for i in 0..SIZE {
        for j in 0..SIZE {
            if board[i][j] != (expected % (SIZE * SIZE) as u32) {
                return false;
            }
            expected += 1;
        }
    }
    true
}

// Display the board on the console
pub fn print_board(board: &Board) {
    for (i, row) in board.iter().enumerate().rev() {
        println!("+-----+-----+-----+-----+");

        let row_str: Vec<String> = row.iter().enumerate().map(|(j, &cell)| {
            let correct_position = (SIZE - 1 - i) * SIZE + j + 1;
            if cell == 0 {
                format!("     ")
            } else if cell == correct_position as u32 {
                // Green color for correct positions
                format!("{:^5}", cell.to_string().green())
            } else {
                format!("{:^5}", cell)
            }
        }).collect();
        println!("|{}|", row_str.join("|"));
    }

    println!("+-----+-----+-----+-----+");
}

// Accept player input and update board state
pub fn player_move(mut board: Board) -> Board {
    loop {
        println!("Enter Move (w, a, s, d): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Some((dx, dy)) = match input.trim() {
            "w" => Some((-1, 0)),
            "a" => Some((0, 1)),
            "s" => Some((1, 0)),
            "d" => Some((0, -1)),
            _ => None,
        } {
            if let Some((empty_x, empty_y)) = find_empty(&board) {
                let new_x = (empty_x as isize + dx) as usize;
                let new_y = (empty_y as isize + dy) as usize;

                if new_x < SIZE && new_y < SIZE {
                    board[empty_x][empty_y] = board[new_x][new_y];
                    board[new_x][new_y] = 0;
                    return board;
                }
            }
        }

        println!("Invalid move!");
    }
}

// Finds and returns the coordinate of the empty spot
pub fn find_empty(board: &Board) -> Option<(usize, usize)> {
    for i in 0..SIZE {
        for j in 0..SIZE {
            if board[i][j] == 0 {
                return Some((i, j));
            }
        }
    }
    None
}
