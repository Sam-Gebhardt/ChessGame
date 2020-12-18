use std::io;
use std::io::Write;
use char;
use crate::pieces::piece_type;
use crate::pieces::Moves;


fn alpha(n: i8) -> char {
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
        self.b[0][3] = -6;
        self.b[0][4] = -5;

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

        if self.in_check(src, dest) != 0 {
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

    fn in_check(&mut self, src: [i8; 2], dest: [i8; 2]) -> i8 {
        // See if a move cause a check to happen

        self.move_piece(src, dest);

        for i in 0..8 {
            for j in 0..8 {
                let piece: Box<dyn Moves> = piece_type(self.get_piece(i, j), [i, j]);

                if piece.get_key() > 0 {
                    if piece.move_set(&self).contains(&self.black) {
                        self.move_piece(dest, src);
                        return -6; //black is in check
                    }
                } else {
                    if piece.move_set(&self).contains(&self.white) {
                        self.move_piece(dest, src);
                        return 6; // white is in check
                    }
                }
            }
        }
        self.move_piece(dest, src);
        return 0; //not in check
    }

    pub fn check_mate(&mut self) -> bool {
        // King is currently in check and can't move

        let check: i8 = self.in_check([0, 0], [0, 0]);
        let pos: [i8; 2];
        let piece: Box<dyn Moves>;
        let color: i8;

        if check == -6 { 
            pos = self.black;
            piece = piece_type(self.get_piece(pos[0], pos[1]), pos);
            color = -6;

        } else if check == 6 {
            pos = self.white;
            piece = piece_type(self.get_piece(pos[0], pos[1]), pos);
            color = 6;

        } else {
            return false
        }

        let moves: Vec<[i8; 2]> = piece.move_set(&self);
        if moves.len() == 0 {
            return true;
        }

        for i in 0..moves.len() {
            if self.in_check(pos, moves[i]) == color {
                return true;
            }
        }
        return false;
    }

    pub fn stalemate(&self) -> bool {
        // no legal moves, but not in check

        let mut piece: Box<dyn Moves>;
        let mut moves: Vec<[i8; 2]>;
        let mut black = true;
        let mut white = true;

        // loop through the board, if a piece has a move then there is no stalemate
        for i in 0..8 {
            for j in 0..8 {
                if black && self.get_piece(i, j) < 0 {
                    piece = piece_type(self.get_piece(i, j), [i, j]);
                    moves = piece.move_set(&self);

                    if moves.len() != 0 { 
                        black = false;
                    }
                } else if white && self.get_piece(i, j) > 0 {
                    piece = piece_type(self.get_piece(i, j), [i, j]);
                    moves = piece.move_set(&self);

                    if moves.len() != 0 { 
                        white = false;
                    }
                }
            }
        }
        
        return black || white;
    }
}
