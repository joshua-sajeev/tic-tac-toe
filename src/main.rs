use std::{fmt,io,cmp};
#[derive(Debug,PartialEq,Clone,Copy)]
enum Mark{
    X,
    O,
    B,
}
#[derive(Debug)]
struct Players {
    player: Mark,
    opponent: Mark,
    active_player: Mark,
}

impl Players{
    fn toggle(&mut self) {
        self.active_player = match self.active_player {
            Mark::O => Mark::X,
            Mark::X => Mark::O,
            Mark::B => Mark::B,
        };
    }
}
impl fmt::Display for Players  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.player {
            Mark::X => write!(f, "You are X and Digambaran is O"),
            Mark::O => write!(f, "You are O and Digambaran is X"),
            Mark::B => write!(f, "-"),
        }
    }
}
impl fmt::Display for Mark  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mark::X => write!(f, "X"),
            Mark::O => write!(f, "O"),
            Mark::B => write!(f, "-"),
        }
    }
}
#[derive(Debug)]
struct TicTacToe {
    board: Vec<Vec<Mark>>,
}
impl TicTacToe {
    fn new() -> Self {
        TicTacToe {
            board: vec![vec![Mark::B; 3]; 3],
        }
    }

    fn print_board(&self) {

        println!("+---+---+---+");
        println!("| {} | {} | {} |",self.board[0][0],self.board[0][1],self.board[0][2]);
        println!("+---+---+---+");
        println!("| {} | {} | {} |",self.board[1][0],self.board[1][1],self.board[1][2]);
        println!("+---+---+---+");
        println!("| {} | {} | {} |",self.board[2][0],self.board[2][1],self.board[2][2]);
        println!("+---+---+---+");
        println!();
    }
}
fn main() {
    println!("\nDigambaran is  a tic-tac-toe app developed in rust.Enjoy your Game!!");
    let mut p = Players {
        player: Mark::X,
        opponent: Mark::O,
        active_player: Mark::X,
    };
    println!("By Default you are 'O',\nDo you want to change;\ny/n(default y);");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("failed to read from stdin");

    let choice = choice.chars().next();
    let change = match choice{
        Some('y') => true,
        Some('n') => false,
        _  => {
             println!("\nChoosing default value!\n");
             true
        }
    };

    if change{
        p.player = Mark::O;
        p.opponent = Mark::X;
        p.active_player = Mark::X;
    }
    println!("{p}");
    println!("+---+---+---+");
    println!("| 1 | 2 | 3 |");
    println!("+---+---+---+");
    println!("| 4 | 5 | 6 |");
    println!("+---+---+---+");
    println!("| 7 | 8 | 9 |");
    println!("+---+---+---+");
    println!("Assume the numbers are the corresponding positions/n Press 0 to print this!");

    let mut occupied:Vec<u8> = vec![];
    let mut win = false;
    let mut count = 0;
    let mut tic_tac_toe = TicTacToe::new();
    while !win {
        println!("Active Player : {}",p.active_player);
        let  (mut i,mut j):(usize,usize) = (3,3);
        if p.active_player == p.player{
            (i,j) = find_best_move(&mut occupied,&mut tic_tac_toe.board,&p);
        }else{
            tic_tac_toe.print_board();  
            let a = read_input(&mut occupied,&tic_tac_toe);
            (i,j) = find_position(a);
        }
        count += 1;
        if i != 3 && j != 3 {
            tic_tac_toe.board[i][j] = p.active_player;
        }
        tic_tac_toe.print_board();
        win = winner(&tic_tac_toe.board);
        p.toggle();
        if count == 9 && !win {
            println!("It's a draw!");
            break;
        }
    }
}

fn read_input(pos: &mut Vec<u8>,ttt: &TicTacToe) -> u8 {
    loop {
        println!("Enter your position");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read from stdin");

        let input = input.trim();

        match input.parse::<u8>() {
            Ok(0) => {
                println!("+---+---+---+");
                println!("| 1 | 2 | 3 |");
                println!("+---+---+---+");
                println!("| 4 | 5 | 6 |");
                println!("+---+---+---+");
                println!("| 7 | 8 | 9 |");
                println!("+---+---+---+");
                println!("Assume the numbers are the corresponding positions");
                ttt.print_board();
            }
            Ok(res) => {
                if (1..=9).contains(&res) {
                    if !pos.contains(&res) {
                        pos.push(res);
                        return res;
                    } else {
                        println!("Position already occupied. Please choose another position.");
                    }
                } 
                else {
                    println!("Not a valid position. Please enter a number between 1 and 9.");
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }
}
fn minimax(board: &Vec<Vec<Mark>>, depth: u8, is_max: bool, p: &Players) -> i32 {
    let score = evaluate(board, p.player, p.opponent);

    if score == 10 {
        return score ;
    }

    if score == -10 {
        return score ;
    }

    if !is_moves_left(board) {
        return 0;
    }

    if is_max {
        let mut best = -1000;
        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] == Mark::B {
                    let mut new_board = board.clone();
                    new_board[i][j] = p.player;
                    best = cmp::max(best, minimax(&new_board, depth + 1, !is_max, p));
                }
            }
        }
        best
    } else {
        let mut best = 1000;
        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] == Mark::B {
                    let mut new_board = board.clone();
                    new_board[i][j] = p.opponent;
                    best = cmp::min(best, minimax(&new_board, depth + 1, !is_max, p));
                }
            }
        }
        best
    }
}

fn is_moves_left(board: &Vec<Vec<Mark>>) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] == Mark::B {
                return true;
            }
        }
    }
    false
}

fn evaluate(board: &Vec<Vec<Mark>>, player: Mark, opponent: Mark) -> i32 {
    for row in 0..3 {
        if board[row][0] == board[row][1] && board[row][1] == board[row][2] {
            if board[row][0] == player {
                return 10;
            } else if board[row][0] == opponent {
                return -10;
            }
        }
    }

    for col in 0..3 {
        if board[0][col] == board[1][col] && board[1][col] == board[2][col] {
            if board[0][col] == player {
                return 10;
            } else if board[0][col] == opponent {
                return -10;
            }
        }
    }

    if board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        if board[0][0] == player {
            return 10;
        } else if board[0][0] == opponent {
            return -10;
        }
    }

    if board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        if board[0][2] == player {
            return 10;
        } else if board[0][2] == opponent {
            return -10;
        }
    }

    0
}

fn find_best_move(pos: &mut Vec<u8>,board: &mut Vec<Vec<Mark>>,p : &Players)->(usize,usize){
    let mut best_val = -1000;
    let mut best_mov = (3,3);
    for i in 0..3{
        for j in 0..3{
            if board[i][j] == Mark::B{
                board[i][j] = p.player;
                let mov_val = minimax(board, 0, false,&p);
                board[i][j] = Mark::B; 
                if mov_val > best_val{
                    best_mov = (i, j);
                    best_val = mov_val;
                }
            }
        }
    }
    let position = match best_mov {
        (0, 0) => 1,
        (0, 1) => 2,
        (0, 2)=> 3,
        (1, 0)=> 4,
        (1, 1)=> 5,
        (1, 2)=> 6,
        (2, 0)=> 7,
        (2, 1)=> 8,
        (2, 2)=> 9,
        _ =>  0,
    };
    pos.push(position);
    best_mov
}
fn find_position(pos: u8)->(usize,usize) {
    let (i, j) = match pos {
        1 => (0, 0),
        2 => (0, 1),
        3 => (0, 2),
        4 => (1, 0),
        5 => (1, 1),
        6 => (1, 2),
        7 => (2, 0),
        8 => (2, 1),
        9 => (2, 2),
        _ => (3, 3),
    };

    (i,j)
}

fn winner(board: &Vec<Vec<Mark>>) -> bool {
    // Check for a winner in row
    for i in 0..3{
        if board[i][1] != Mark::B{
            if (board[i][1] == board[i][0]) &&  (board[i][1] == board[i][2]){
                println!("{} is the winner",board[i][1]);
                return true;
            }
        }
    }

    //Check for a winner in a col 
    for i in 0..3{
        if board[1][i] != Mark::B{
            if (board[1][i] == board[0][i]) &&  (board[1][i] == board[2][i]){
                println!("{} is the winner",board[1][i]);
                return true;
            }
        }
    }

    //Check for a winner diaganally 
    for i in 0..3{
        if board[1][1] != Mark::B{
            if (board[0][0] == board[1][1]) &&  (board[1][1] == board[2][2]){
                println!("{} is the winner",board[1][1]);
                return true;
            }else if board[0][2] == board[1][1] && (board[1][1] == board[2][0]){
                println!("{} is the winner",board[1][1]);
                return true;
            }
        }
    }
    false
}

