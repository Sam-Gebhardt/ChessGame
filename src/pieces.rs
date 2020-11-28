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
}


pub struct Pawn {
    pub pos: [i8; 2],
    pub key: i8
}

pub struct Tower {
    pub pos: [i8; 2],
    pub key: i8
}

pub struct Knight {
    pub pos: [i8; 2],
    pub key: i8
}

pub struct Bishop {
    pub pos: [i8; 2],
    pub key: i8
}

// struct King {
//     pos: [i8; 2],
//     key: i8
// }

pub struct Queen {
    pub pos: [i8; 2],
    pub key: i8
}

fn sign_checker(one: i8, two: i8) -> bool {
    // return true if the numbers are the same sign
    // else return false
    if two == 0 {
        return false;
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

            // Can't use signal_checker fn because 0 is a special case for Pawns
            if !((self.key > 0 && diagonal >= 0) || (self.key < 0 && diagonal <= 0)) {
                all_moves.push(valid[i]);
            }
        }
        return all_moves;
    }
}

impl Moves for Tower {
    // Can move NSEW, till it reaches a piece or border
    // Todo: Castling

    fn move_set(&self, board: &Board) -> Vec<[i8; 2]> {
        let mut moves: Vec<[i8; 2]> = Vec::new();
        // Using 4 vectors and appending makes the output in order instead of mangled
        let mut moves_0: Vec<[i8; 2]> = Vec::new();
        let mut moves_1: Vec<[i8; 2]> = Vec::new();
        let mut moves_2: Vec<[i8; 2]> = Vec::new();
        let mut moves_3: Vec<[i8; 2]> = Vec::new();
        // TODO: Condense into a single vec once debugging is done

        // flag is set to false if piece is encoutered
        let mut flags: [bool; 4] = [true; 4];
        let x = self.pos[1];
        let y = self.pos[0];

        // Check if possible move is inbounds and if move is not behind another piece
        // Then check if a piece occupys that space. If one is there set flag to false
        // and check if piece is enemy or friedly, otherwise space is open
        // Do check for each direction
        for i in 1..9 { //TODO: use board.get_piece()
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

impl Moves for Knight {
    fn move_set(&self, board: &Board) -> Vec<[i8; 2]> {
        // Can move in an L shape, 8 possible moves
        let mut moves: Vec<[i8; 2]> = Vec::new();
        let mut legal_moves: Vec<[i8; 2]> = Vec::new();
        let changes: [[i8; 2]; 8] = [[-2, 1], [-2, -1], [2, 1], [2, -1], 
                                     [1, 2], [-1, 2], [1, -2], [-1, -2]];
        let x = self.pos[1];
        let y = self.pos[0];

        for i in 0..8 {
            // Bound check
            if !(y + changes[i][0] > 7 || y + changes[i][0] < 0 
                || x + changes[i][1] > 7 || x + changes[i][1] < 0) {

                moves.push([y + changes[i][0], x + changes[i][1]])
            }
        }

        for i in 0..moves.len() {
            if !sign_checker(self.key, board.get_piece(moves[i][0], moves[i][1])) {
                legal_moves.push(moves[i]);
            }
        }
        
        return legal_moves;
    }
}

impl Moves for Bishop {
    fn move_set(&self, board: &Board) -> Vec<[i8; 2]>{
        // Can move in the diagonal until it reaches a piece or border

        let mut moves: Vec<[i8; 2]> = Vec::new();
        // Using 4 vectors and appending makes the output in order instead of mangled
        let mut moves_0: Vec<[i8; 2]> = Vec::new();
        let mut moves_1: Vec<[i8; 2]> = Vec::new();
        let mut moves_2: Vec<[i8; 2]> = Vec::new();
        let mut moves_3: Vec<[i8; 2]> = Vec::new();
        // TODO: Condense into a single vec once debugging is done

        // flag is set to false if piece is encoutered
        let mut flags: [bool; 4] = [true; 4];
        let x = self.pos[1];
        let y = self.pos[0];

        // Check if possible move is inbounds and if move is not behind another piece
        // Then check if a piece occupys that space. If one is there set flag to false
        // and check if piece is enemy or friedly, otherwise space is open
        // Do check for each direction
        for i in 1..9 { 
            if (x + i < 8) && (y + i < 8) && flags[0] {

                if board.get_piece(y + i, x + i) != 0  {
                    flags[0] = false;
                    if !sign_checker(self.key, board.get_piece(y + i, x + i)) {
                        moves_0.push([y + i, x + i]);
                    }
                } else {
                    moves_0.push([y + i, x + i]);
                }
            } if (x + i < 8) && (y - i > -1) && flags[1] {
                if board.get_piece(y - i, x + i) != 0 {
                    flags[1] = false;
                    if !sign_checker(self.key, board.get_piece(y - i, x + i)) {
                        moves_1.push([y - i, x + i]);
                    }
                } else {
                    moves_1.push([y - i, x + i]);
                }
            } if (x - i > -1) && (y + i < 8) && flags[2] {
                if board.get_piece(y + i, x - i) != 0 {
                    flags[2] = false;
                    if !sign_checker(self.key, board.get_piece(y + i, x - i)) {
                        moves_2.push([y + i, x - i]);
                    }
                } else {
                    moves_2.push([y + i, x - i]);
                }
            } if(x - i > -1) && (y - i > -1) && flags[3] {
                if board.get_piece(y - i, x - i) != 0 {
                    flags[3] = false;
                    if !sign_checker(self.key, board.get_piece(y - i, x - i)) {
                        moves_3.push([y - i, x - i]);
                    }
                } else {
                    moves_3.push([y - i, x - i]);
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

// impl Moves for King {
//     fn open_moves(&mut self, board: Board) {
//         if self.key == -1 {
//             println!("Yes");
//         }
//     }
// }

impl Moves for Queen {
    // Call tower + bishop 
    fn move_set(&self, board: &Board) -> Vec<[i8; 2]> {
        let t: Tower = Tower{pos: self.pos, key: self.key};
        let b: Bishop = Bishop{pos: self.pos, key: self.key};

        let mut b_moves: Vec<[i8; 2]> = b.move_set(board);
        let mut t_moves: Vec<[i8; 2]> = t.move_set(board);

        b_moves.append(&mut t_moves);

        return b_moves;
    }
}
