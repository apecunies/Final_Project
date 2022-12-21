use std::io;

fn main() {
    let mut board = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']];
    let mut current_player = 'X';

    loop {
        println!("Current board:");
        print_board(&board);

        println!("Player {}'s turn. Enter row and column:", current_player);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let coords: Vec<&str> = input.trim().split(" ").collect();
        let row: usize = coords[0].parse().expect("Not a number");
        let col: usize = coords[1].parse().expect("Not a number");

        if row > 2 || col > 2 {
            println!("Invalid move!");
            continue;
        }

        if board[row][col] != ' ' {
            println!("That space is already occupied!");
            continue;
        }

        board[row][col] = current_player;

        if check_win(&board, current_player) {
            println!("Player {} wins!", current_player);
            break;
        }

        if check_draw(&board) {
            println!("It's a draw!");
            break;
        }

        if current_player == 'X' {
            current_player = 'O';
        } else {
            current_player = 'X';
        }
    }
}

fn check_win(board: &[[char; 3]; 3], player: char) -> bool {
    /// Check rows
    for row in board {
        if row[0] == player && row[1] == player && row[2] == player {
            return true;
        }
    }

    /// Check columns
    for col in 0..3 {
        if board[0][col] == player && board[1][col] == player && board[2][col] == player {
            return true;
        }
    }

    /// Check diagonals
    if board[0][0] == player && board[1][1] == player && board[2][2] == player {
        return true;
    }
    if board[2][0] == player && board[1][1] == player && board[0][2] == player {
        return true;
    }

    false
}

fn check_draw(board: &[[char; 3]; 3]) -> bool {
    for row in board {
        for col in row {
            if *col == ' ' {
                return false;
            }
        }
    }
    true
}

fn print_board(board: &[[char; 3]; 3]) {
    println!("  0 1 2");
    for (i, row) in board.iter().enumerate() {
        print!("{} ", i);
        for col in row {
            print!("{} ", col);
        }
        println!();
    }
}
