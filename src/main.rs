use std::io;
use tic_tac_toe::{Board, CellIndex, Symbol, TurnResult};

const EXIT: &str = "exit";

const X: &str = "x";
const O: &str = "o";

const Y: &str = "y";
const N: &str = "n";

fn main() {
    println!("Welcome to tic-tac-toe.");

    loop {
        let starting_symbol = select_starting_player();

        if starting_symbol == None {
            break;
        }

        if play_game(starting_symbol.unwrap()) {
            break;
        }

        println!("Play again? Y/N");

        if check_final_exit() {
            break;
        }
    }
}

fn select_starting_player() -> Option<Symbol> {
    loop {
        println!("Who will play first? Type X or O");

        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line!");

        let line = line.trim().to_lowercase();
        match line.as_str() {
            X => break Some(Symbol::X),
            O => break Some(Symbol::O),
            EXIT => break None,
            _ => println!("I couldn't understand that. Type \"{EXIT}\" to quit."),
        }
    }
}

fn play_game(starting_symbol: Symbol) -> bool {
    let mut board: Board = Board::new(starting_symbol);
    loop {
        let current_player = board.current_player();
        let board_text = board.as_text();

        print!("{board_text}");
        println!("Player {current_player} it's your turn.\nType your move in COLUMN ROW format.\nType \"{EXIT}\" to quit");

        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim().to_lowercase();

        if line == EXIT {
            break true;
        }

        let indices: Vec<u8> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<u8>().ok())
            .collect();

        if indices.len() == 2 {
            let turn_result = board.process_turn(CellIndex(indices[0], indices[1]));
            match turn_result {
                TurnResult::Continue => continue,
                TurnResult::Draw => {
                    println!("It's a draw!");
                    break false;
                }
                TurnResult::Winner(symbol) => {
                    println!("Player {} won!", symbol);
                    let board_output = board.as_text();
                    print!("{board_output}");
                    break false;
                }
                TurnResult::InvalidMove => {
                    println!("Space already occupied. Please try again.");
                    continue;
                }
            }
        } else {
            println!("Invalid input format!\nPlease try again.");
        }
    }
}

fn check_final_exit() -> bool {
    loop {
        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim().to_lowercase();
        match line.as_str() {
            Y => break false,
            N => break true,
            _ => println!("Invalid input format, please try again."),
        }
    }
}
