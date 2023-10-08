mod game_logic;
use game_logic::*;

// Main function to run the 15 game
pub fn main() {
    // Game Introduction
    let intro = r#"
    __  ______   ______                                                                       
   / / / ____/  / ____/____ _ ____ ___   ___                                                  
  / / /___ \   / / __ / __ `// __ `__ \ / _ \                                                 
 / / ____/ /  / /_/ // /_/ // / / / / //  __/                                                 
/_/ /_____/   \____/ \__,_//_/ /_/ /_/ \___/                                                  
                                                                                             
    __              ___            _          _    __       __          __                 _ 
   / /_   __  __   /   |   ____   (_)_____   | |  / /___   / /_ __  __ / /__ __  __ _____ (_)
  / __ \ / / / /  / /| |  / __ \ / // ___/   | | / // _ \ / __// / / // //_// / / // ___// / 
 / /_/ // /_/ /  / ___ | / / / // // /       | |/ //  __// /_ / /_/ // ,<  / /_/ // /   / /  
/_.___/ \__, /  /_/  |_|/_/ /_//_//_/        |___/ \___/ \__/ \__,_//_/|_| \__,_//_/   /_/   
       /____/ 
    "#;
    println!("{}", intro);

    let mut board = initialize_board();
    shuffle(&mut board, 100);
    let mut total_moves = 0;

    while !is_solved(&board) {
        print_board(&board);
        board = player_move(board);
        total_moves += 1;
        println!("Total Moves: {}", total_moves);
    }
    
    println!("Congrats! You solved the puzzle in {} moves!", total_moves);
}