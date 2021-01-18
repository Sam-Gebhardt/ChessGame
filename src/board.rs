use std::io;
use std::io::Write;
use char;
use crate::pieces::piece_type;
use crate::pieces::Moves;
use crate::pieces::sign_checker;


pub fn alpha(n: i8) -> char {
    // Converts index to alpha char
    let a: char = match n {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => 'a',
    };
    return a;
}

fn numeric(n: char) -> i8 {
    let a: i8 = match n {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => 0,
    };
    return a as i8;
}

fn convert_user_input(input: String) -> [i8; 2] {
    let mut pos: [i8; 2] = [0, 0];

    pos[0] = numeric(input.chars().nth(0).unwrap()) as i8;
    pos[1] = input.chars().nth(1).unwrap().to_digit(10).unwrap() as i8;
    pos[1] = pos[1] - 1;

    return pos;
}

#[derive(Copy, Clone)]
pub struct Board {
    pub b: [[i8; 8]; 8],
    pub white: [i8; 2],
    pub black: [i8; 2]
}

impl Board {
    //Fill the board with pieces
    pub fn construct(&mut self) { 

        // put the pawns on the board
        for i in 0..8 {
            self.b[1][i] = -1;
            self.b[6][i] = 1;
        }

        // put non pawns on the board
        let mut start = 2;
        let mut change = 1;
        for i in 0..8 {
            self.b[0][i] = start * -1;
            self.b[7][i] = start;
            start += change;
            if start == 7 {
                start -= 3;
                change = -1;
            }
        }
        // King and queen should be on opposite sides
        self.b[7][3] = 6;
        self.b[7][4] = 5;

        self.black = [0, 3];
        self.white = [7, 4];
    }

    pub fn print_b(&self) {
        // print the board
        println!("");
        print!("   ");
        for i in 1..9 {
            print!("  {}", i);
        }
        io::stdout().flush().unwrap();
        println!("");
        for _i in 0..9 {
            print!("―――");
        }
        println!("");

        let n: usize = 8;
        for i in 0..n {
            for j in 0..n {
                
                if j == 0 {
                    print!("{} |", alpha(i as i8));
                    io::stdout().flush().unwrap();
                }
    
                if self.b[i][j] >= 0 {
                    io::stdout().flush().unwrap();

                    print!("  {}", self.b[i][j]);
    
                } else {
                    print!(" {}", self.b[i][j]);
                }
            }
            io::stdout().flush().unwrap();
            println!("");
        }
        println!("");
    }

    pub fn user_move(&mut self) -> bool{
        // Allows user to move pieces

        return self.get_input();
    }

    fn get_input(&mut self) -> bool {
        let mut from: String =  String::new();
        let mut to: String = String::new();

        print!("From: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut from)
            .expect("Enter a pos");

        print!("To:   ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut to)
            .expect("Enter a pos");
        
        if from == "q\n".to_string() {
            return false;
        }
        
        let parsed_from: [i8; 2] = convert_user_input(from);
        let parsed_to: [i8; 2] = convert_user_input(to);

        if self.get_piece(parsed_from[0], parsed_from[1]) < 0 {
            println!("Can't move enemy pieces. (You are white)\n");
            self.get_input();
        } else if !self.legal_move(parsed_from, parsed_to) {
            println!("Illegal move!\n");
            self.get_input();
        } else {
            self.move_piece(parsed_from, parsed_to);
        }
        return true;
    }

    fn legal_move(&mut self, src: [i8; 2], dest: [i8; 2]) -> bool {
        // is the inputed move legal
        
        if src[0] > 7 || src[0] < 0 || src[1] > 7 || src[1] < 0 {
            return false
        } else if dest[0] > 7 || dest[0] < 0 || dest[1] > 7 || dest[1] < 0 {
            return false
        }

        let key: i8 = self.b[src[0] as usize][src[1] as usize];
        let p: Box<dyn Moves> = piece_type(key, src);

        //passing in self as &Board
        let move_set: Vec<[i8; 2]> = p.move_set(&self);
        if !(move_set.contains(&dest)) {
            return false
        }

        if self.in_check(src, dest, 1) {
            println!("Move puts you in check.");
            return false;
        }

        return true;
    }

    fn set_king(&mut self, key: i8, pos: [i8; 2]) {
        if key > 0 {
            self.white = pos;
        }
        self.black = pos;
    }

    pub fn get_piece(&self, one: i8, two: i8) -> i8 {
        // Get a piece from the board at b[one][two]

        return self.b[one as usize][two as usize];
    }

    pub fn move_piece(&mut self, src: [i8; 2], dest: [i8; 2]) {
        // Move a piece from src to dest, set src to 0

        let key: i8 = self.get_piece(src[0], src[1]);

        if key == 5 || key == -5 {
            self.set_king(key, dest);
        }

        self.b[dest[0] as usize][dest[1] as usize] = key;
        self.b[src[0] as usize][src[1] as usize] = 0;
    }

    pub fn in_check(&mut self, src: [i8; 2], dest: [i8; 2], key: i8) -> bool {
        // See if a move cause a check to happen

        let mut board_copy = self.clone();
        board_copy.move_piece(src, dest);

        let king = if key > 0 {&board_copy.white} else {&board_copy.black};
        let mut piece: Box<dyn Moves>;
        let mut piece_key: i8;

        for i in 0..8 {
            for j in 0..8 {
                piece_key = board_copy.get_piece(i, j);

                if !sign_checker(key, piece_key) && piece_key != 0 {
                    piece = piece_type(piece_key, [i, j]);

                    if piece.move_set(&board_copy).contains(king) {
                        return true; 
                    }
                }  
            }
        }
        return false; 
    }

    pub fn check_mate(&mut self, key: i8) -> bool {
        // Test if key color is in checkmate
        // *Is dependent of check moves being filtered out in pieces*

        let helper = self.check_mate_helper();
        let check: bool = self.in_check(helper, helper, key);
        let pos: [i8; 2] = if key > 0 {self.white} else {self.black};

        let mut piece: Box<dyn Moves>;
        let mut moves: Vec<[i8; 2]>;

        if !check { 
            return false;
        }

        piece = piece_type(self.get_piece(pos[0], pos[1]), pos);
        if piece.move_set(&self).len() != 0 {
            return false;
        }

        // Must check if any piece can protect the king
        for i in 0..8 {
            for j in 0..8 {
                if sign_checker(self.get_piece(i, j), key) {
                    piece = piece_type(self.get_piece(i, j), [i, j]);
                    moves = piece.move_set(&self);

                    for a in 0..moves.len() {
                        if !self.in_check(piece.get_pos(), moves[a], key) {
                            return false;
                        }
                    }

                }
            }
        }

        return true; 
    }

    pub fn check_mate_helper(&self) -> [i8; 2] {
        // finds an empty space to see if current state is in check

        for i in 2..8 {
            for j in 0..8 {
                if self.get_piece(i, j) == 0 {
                    return [i, j];
                }
            }
        }
        // to make the compliler happy, but there must be an empty space on the board
        return [0, 0];
    }

    #[allow(dead_code)]
    pub fn stalemate(&self, key: i8) -> bool {
        // no legal moves, but not in check
        // key should be the sign of who's turn it is 

        let mut piece: Box<dyn Moves>;
        let mut moves: Vec<[i8; 2]>;

        // loop through the board, if a piece has a move then there is no stalemate
        for i in 0..8 {
            for j in 0..8 {
                if sign_checker(self.get_piece(i, j), key) {
                    piece = piece_type(self.get_piece(i, j), [i, j]);
                    moves = piece.move_set(&self);

                    if moves.len() != 0 { 
                        return false;
                    }
                } 
            }
        }
        
        return true;
    }
}
