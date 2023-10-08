use fifteen_game::game_logic::*;

// Test board initialization with correct number in the right position
#[test]
fn test_initialize_board() {
    let board = initialize_board();
    let mut expected_num = 1;
    
    for i in 0..SIZE {
        for j in 0..SIZE {
            if i == SIZE - 1 && j == SIZE - 1 {
                assert_eq!(board[i][j], 0);
            } else {
                assert_eq!(board[i][j], expected_num);
                expected_num += 1;
            }
        }
    }
}

// Check the is_solved function return true on solved board
#[test]
fn test_is_solved_on_solved_board() {
    let solved_board = initialize_board();
    assert_eq!(is_solved(&solved_board), true);
}

// Check the is_solved function return false on unsolved board
#[test]
fn test_is_solved_on_unsolved_board() {
    let mut unsolved_board = initialize_board();
    unsolved_board[0][0] = 99;
    assert_eq!(is_solved(&unsolved_board), false);
}

// Check if find empty returns the correct square on initialized board
#[test]
fn test_find_empty_on_empty_last() {
    let board = initialize_board();
    assert_eq!(find_empty(&board), Some((SIZE - 1, SIZE - 1)));
}

// Check if find empty returns the correct square for random square
#[test]
fn test_find_empty_on_custom_position() {
    let mut board = initialize_board();
    board[SIZE-1][SIZE-1] = 15;
    board[SIZE-2][SIZE-2] = 0;
    assert_eq!(find_empty(&board), Some((SIZE-2, SIZE-2)));
}
