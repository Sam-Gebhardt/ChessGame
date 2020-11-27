/*
This file holds the classes of each piece and their respective moves
*/


pub struct Board {
    pub b: [[i8; 8]; 8]
}

impl Board {
    pub fn construct(&mut self) { //Fill the board with pieces
        // put the pawns on the board
        for i in 0..8 {
            self.b[1][i] = 1;
            self.b[6][i] = -1;
        }

        // put non pawns on the board
        let mut start = 2;
        let mut change = 1;
        for i in 0..8 {
            self.b[0][i] = start;
            self.b[7][i] = start * -1;
            start += change;
            if start == 7 {
                start -= 3;
                change = -1;
            }
        }
        // King and queen should be on opposite sides
        self.b[7][3] = -6;
        self.b[7][4] = -5;
    }

    pub fn get_piece(&self, one: i8, two: i8) -> i8 {
        // Get a piece from the board at b[one][two]

        // Cast to correct type for indexing
        let one_usize: usize = one as usize;
        let two_usize: usize = two as usize;

        return self.b[one_usize][two_usize];

    }
    pub fn bound_check(&self, moves: Vec<[i8; 2]>) -> Vec<[i8; 2]> {
        // Checks two things:
        // 1: is move on the board ( 0 =< x/y <= 7)
        // 2: is their a friendly piece at the position

        let mut valid: Vec<[i8; 2]> = Vec::new();
        for i in 0..moves.len() {
            if (8 > moves[i][0] && moves[i][0] > -1) && (8 > moves[i][1] && moves[i][1] > -1) {
                valid.push(moves[i]);
            }
        }

        let mut empty: Vec<[i8; 2]> = Vec::new();
        for i in 0..valid.len() {
            if self.b[valid[i][0] as usize][valid[i][1] as usize] == 0 {
                empty.push(valid[i]);
            } 
        }
        return valid;
    }
}


pub struct Pawn {
    pub pos: [i8; 2],
    pub key: i8
}

pub struct Tower {
    pub pos: [i8; 2],
    pub key: i8
}

// struct Knight {
//     pos: [i8; 2],
//     key: i8
// }

// struct Bishop {
//     pos: [i8; 2],
//     key: i8
// }

// struct King {
//     pos: [i8; 2],
//     key: i8
// }

// pub struct Queen {
//     pos: [i8; 2],
//     pub key: i8
// }

fn sign_checker(one: i8, two: i8) -> bool {
    // return true if the numbers are the same sign
    // else return false
    if two == 0 {
        return true;
    }
    if one > 0 && two > 0 {
        return true;
    } else if one < 0 && two < 0 {
        return true;
    } return false;
}

pub trait Moves {
    // Empty methods to overwrite by each piece
    fn move_set(&self, _board: &Board) -> Vec<[i8; 2]> {
        let val: Vec<[i8; 2]> = Vec::new();
        return val;
    }

    fn open_moves(&mut self, _board: Board) {
        return;
    }
}


impl Moves for Pawn {
    // has 4 possible moves:
    // +2 (Special case), +1, +1-1, +1+1

    fn move_set(&self, board: &Board) -> Vec<[i8; 2]> {
        let mut direction = 1; 

        if self.key == -1 {
            direction = -1;
        }
        // A vector that holds all possible moves for a Pawn at pos [x][y]
        let mut all_moves: Vec<[i8; 2]> = Vec::new();
        let mut valid: Vec<[i8; 2]> = Vec::new();

        // Check if Pawn can move 2 spaces
        if (self.pos[0] == 6 && self.key == -1) || (self.pos[0] == 1 && self.key == 1) {
            valid.push([self.pos[0] + direction * 2, self.pos[1]]);
        }

        // Edge case: Pawn reaches end of board causes array issue
        // shoudn't be an issue once upgrading pawns is implimented

        // Move forward 1 
        valid.push([self.pos[0] + 1 * direction, self.pos[1]]);

        // Does a piece occupy a position where the pawn would move?
        for i in 0..valid.len() {
            if board.get_piece(valid[i][0], valid[i][1]) == 0 {
                all_moves.push(valid[i]);
            }
        }
        valid.clear();

        // Check if an oppenent is in the diagonal
        // Pawn has attack seperate from regular move, so i'll do bound 
        // checking within the function
        if (self.pos[1] + 1) != 8 {
            valid.push([self.pos[0] + (1 * direction), self.pos[1] + 1]);
        } if (self.pos[1] - 1) != -1 {
            valid.push([self.pos[0] + (1 * direction), self.pos[1] - 1]);
        }

        // Now that we have the valid moves, check if they are legal
        for i in 0..valid.len() {
            let diagonal: i8 = board.get_piece(valid[i][0], valid[i][1]);
            println!("self: {}, dia: {}", self.key, diagonal);
            if !sign_checker(self.key, diagonal) {
                all_moves.push(valid[i]);
            }
        }
        return all_moves;
    }
}

impl Moves for Tower {
    // Can move NSEW, till it reaches a piece of border

    fn move_set(&self, board: &Board) -> Vec<[i8; 2]> {
        let mut moves: Vec<[i8; 2]> = Vec::new();
        // Using 4 vectors and appending makes the output in order instead of mangled
        let mut moves_0: Vec<[i8; 2]> = Vec::new();
        let mut moves_1: Vec<[i8; 2]> = Vec::new();
        let mut moves_2: Vec<[i8; 2]> = Vec::new();
        let mut moves_3: Vec<[i8; 2]> = Vec::new();
        // TODO: Condese into a single vec once debugging is done

        // flag is set to false if piece is encoutered
        let mut flags: [bool; 4] = [true; 4];
        let x = self.pos[1];
        let y = self.pos[0];

        // Check if possible move is inbounds and if move is not behind another piece
        // Then check if a piece occupys that space. If one is there set flag to false
        // and check if piece is enemy or friedly, otherwise space is open
        // Do check for each direction
        for i in 1..9 {
            if x + i < 8 && flags[0] {

                if board.b[y as usize][(x + i) as usize] != 0  {
                    flags[0] = false;
                    if !sign_checker(self.key, board.b[y as usize][(x + i) as usize]) {
                        moves_0.push([y, x + i]);
                    }
                } else {
                    moves_0.push([y, x + i]);
                }
            } if x - i > -1 && flags[1] {
                if board.b[y as usize][(x - i) as usize] != 0 {
                    flags[1] = false;
                    if !sign_checker(self.key, board.b[y as usize][(x - i) as usize]) {
                        moves_1.push([y, x - i]);
                    }
                } else {
                    moves_1.push([y, x - i]);
                }
            } if y + i < 8 && flags[2] {
                if board.b[(y + i) as usize][x as usize] != 0 {
                    flags[2] = false;
                    if !sign_checker(self.key, board.b[(y + i) as usize][x as usize]) {
                        moves_2.push([y + i, x]);
                    }
                } else {
                    moves_2.push([y + i, x]);
                }
            } if y - i > -1 && flags[3] {
                if board.b[(y - i) as usize][x as usize] != 0 {
                    flags[3] = false;
                    if !sign_checker(self.key, board.b[(y - i) as usize][x as usize]) {
                        moves_3.push([y - i, x]);
                    }
                } else {
                    moves_3.push([y - i, x]);
                }
            }
        }
        
        moves.append(&mut moves_0);
        moves.append(&mut moves_1);
        moves.append(&mut moves_2);
        moves.append(&mut moves_3);

        return moves;

    }
}

// impl Moves for Knight {
//     fn open_moves(&mut self, board: Board) {
//         if self.key == -1 {
//             println!("Yes");
//         }

//     }
// }

// impl Moves for Bishop {
//     fn open_moves(&mut self, board: Board) {
//         if self.key == -1 {
//             println!("Yes");
//         }

//     }
// }

// impl Moves for King {
//     fn open_moves(&mut self, board: Board) {
//         if self.key == -1 {
//             println!("Yes");
//         }
//     }
// }

// impl Moves for Queen {
//     fn open_moves(&mut self, board: Board) {
//         if self.key == -1 {
//             println!("Yes");
//         }

//     }
// }


/* 
Cardinal Directions for movement


Each piece has a move_set and a key. The moveset is a Vector of strigs that holds the
instructions for moving each piece. For example, Pawn is ["f"]
It also holds a unique key that is representative of each piece of the board.
*/