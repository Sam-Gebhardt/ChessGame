/*
This file holds the classes of each piece and their respective moves
*/
use crate::board::Board;

pub fn piece_type(key: i8, pos: [i8; 2]) -> Box<dyn Moves>{
    let abs_key = key.abs();

    // Boxing makes all struct have same size
    let t: Box<dyn Moves> = match abs_key {
        1 => Box::new(Pawn{pos: pos, key: key}),
        2 => Box::new(Tower{pos: pos, key: key}),
        3 => Box::new(Knight{pos: pos, key: key}),
        4 => Box::new(Bishop{pos: pos, key: key}),
        5 => Box::new(King{pos: pos, key: key}),
        6 => Box::new(Queen{pos: pos, key: key}),
        _ => Box::new(Pawn{pos: pos, key: key}),
    };
    return t;
}

struct Pawn {
    pos: [i8; 2],
    key: i8
}

struct Tower {
    pos: [i8; 2],
    key: i8
}

struct Knight {
    pos: [i8; 2],
    key: i8
}

struct Bishop {
    pos: [i8; 2],
    key: i8
}

struct King {
    pos: [i8; 2],
    key: i8
}

struct Queen {
    pos: [i8; 2],
    key: i8
}

pub fn sign_checker(one: i8, two: i8) -> bool {
    // return true if the numbers are the same sign

    if one > 0 && two > 0 {
        return true;

    } else if one < 0 && two < 0 {
        return true;

    } return false;
}

fn tb_helper(key: i8, pos: [i8; 3], flags: &mut [bool; 4], moves: &mut Vec<[i8; 2]>, board: &Board) {
    // Helper func to get moves for tower and bishop
    // Checks if there is a piece in spot, and if that piece is friendly

    if board.get_piece(pos[1], pos[0]) != 0 {
        flags[pos[2] as usize] = false;

        if !sign_checker(key, board.get_piece(pos[1], pos[0])) {
            moves.push([pos[1], pos[0]]);
        }
    } else {
        moves.push([pos[1], pos[0]]);
    }
}

pub trait Moves {

    // Empty methods to overwrite by each piece
    fn move_set(&self, _board: &Board) -> Vec<[i8; 2]> {
        let val: Vec<[i8; 2]> = Vec::new();
        return val;
    }

    fn get_key(&self) -> i8 {
        return 0;
    }

    fn get_pos(&self) -> [i8; 2] {
        return [0, 0];
    }
}

impl Moves for Pawn {

    fn move_set(&self, board: &Board) -> Vec<[i8; 2]> {

        let direction = self.key * -1; 

        let mut moves: Vec<[i8; 2]> = Vec::new();
        let mut valid: Vec<[i8; 2]> = Vec::new();

        // Check if Pawn can move 2 spaces
        if (self.pos[0] == 6 && self.key == 1) || (self.pos[0] == 1 && self.key == -1) {
            valid.push([self.pos[0] + direction * 2, self.pos[1]]);
        }

        // Move forward 1 
        valid.push([self.pos[0] + direction, self.pos[1]]);

        // Does a piece occupy a position where the pawn would move?
        for i in 0..valid.len() {

            if board.get_piece(valid[i][0], valid[i][1]) == 0 {
                moves.push(valid[i]);
            }
        }
        valid.clear();

        // Check if an oppenent is in the diagonal
        if (self.pos[1] + 1) != 8 {
            valid.push([self.pos[0] + direction, self.pos[1] + 1]);
        }
        if (self.pos[1] - 1) != -1 {
            valid.push([self.pos[0] + direction, self.pos[1] - 1]);
        }

        // Now that we have the valid moves, check if they are legal
        for i in 0..valid.len() {
            let diagonal: i8 = board.get_piece(valid[i][0], valid[i][1]);

            // Can't use signal_checker fn because 0 is a special case for Pawns
            if !((self.key > 0 && diagonal >= 0) || (self.key < 0 && diagonal <= 0)) {
                moves.push(valid[i]);
            }
        }
        return moves;
    }

    fn get_key(&self) -> i8 {
        return self.key;
    }

    fn get_pos(&self) -> [i8; 2] {
        return self.pos;
    }
}

impl Moves for Tower {
    // Todo: Castling

    fn move_set(&self, board: &Board) -> Vec<[i8; 2]> {
        
        let mut moves: Vec<[i8; 2]> = Vec::new();

        // flag is set to false if piece is encoutered
        let mut flags: [bool; 4] = [true; 4];
        let x = self.pos[1];
        let y = self.pos[0];

        // Go vertical and horizontal and call helper func
        for i in 1..9 { 
            if x + i < 8 && flags[0] {

                tb_helper(self.key, [x + i, y, 0], &mut flags, &mut moves, board);

            } if x - i > -1 && flags[1] {

                tb_helper(self.key, [x - i, y, 1], &mut flags, &mut moves, board);

            } if y + i < 8 && flags[2] {

                tb_helper(self.key, [x, y + i, 2], &mut flags, &mut moves, board);

            } if y - i > -1 && flags[3] {
                
                tb_helper(self.key, [x, y - i, 3], &mut flags, &mut moves, board);

            }
        }
        return moves;
    }

    fn get_key(&self) -> i8 {
        return self.key;
    }

    fn get_pos(&self) -> [i8; 2] {
        return self.pos;
    }
}

impl Moves for Knight {

    fn move_set(&self, board: &Board) -> Vec<[i8; 2]> {

        let mut moves: Vec<[i8; 2]> = Vec::new();
        let mut legal_moves: Vec<[i8; 2]> = Vec::new();

        // All possible moves for knight
        let changes: [[i8; 2]; 8] = [[-2, 1], [-2, -1], [2, 1], [2, -1], 
                                     [1, 2], [-1, 2], [1, -2], [-1, -2]];
        let x = self.pos[1];
        let y = self.pos[0];

        for i in 0..8 {
            // Bound check
            if !(y + changes[i][0] > 7 || y + changes[i][0] < 0 ||
                 x + changes[i][1] > 7 || x + changes[i][1] < 0) {

                    legal_moves.push([y + changes[i][0], x + changes[i][1]])
            }
        }

        for i in 0..legal_moves.len() {
            if !sign_checker(self.key, board.get_piece(legal_moves[i][0], legal_moves[i][1])) {
                moves.push(legal_moves[i]);
            }
        }
        
        return moves;
    }

    fn get_key(&self) -> i8 {
        return self.key;
    }

    fn get_pos(&self) -> [i8; 2] {
        return self.pos;
    }
}

impl Moves for Bishop {

    fn move_set(&self, board: &Board) -> Vec<[i8; 2]>{
        // Can move in the diagonal until it reaches a piece or border

        let mut moves: Vec<[i8; 2]> = Vec::new();
        
        // flag is set to false if piece is encoutered
        let mut flags: [bool; 4] = [true; 4];
        let x = self.pos[1];
        let y = self.pos[0];

        // Spread out diagonal and call helper funct
        for i in 1..9 { 
            if (x + i < 8) && (y + i < 8) && flags[0] {

                tb_helper(self.key, [x + i, y + i, 0], &mut flags, &mut moves, board);

            } if (x + i < 8) && (y - i > -1) && flags[1] {
                
                tb_helper(self.key, [x + i, y - i, 1], &mut flags, &mut moves, board);

            } if (x - i > -1) && (y + i < 8) && flags[2] {

                tb_helper(self.key, [x - i, y + i, 2], &mut flags, &mut moves, board);

            } if (x - i > -1) && (y - i > -1) && flags[3] {

                tb_helper(self.key, [x - i, y - i, 3], &mut flags, &mut moves, board);

            }
            //todo: rewrite by multiplying [1,1], [-1, 1], [1, -1], [-1,-1] by i and adding
        }
        return moves;
    }

    fn get_key(&self) -> i8 {
        return self.key;
    }

    fn get_pos(&self) -> [i8; 2] {
        return self.pos;
    }
}

impl Moves for King {

    fn move_set(&self, board: &Board) -> Vec<[i8; 2]> {

        // All 8 possible moves for a King
        let changes: [[i8; 2]; 8] = [[1, 0], [1, -1], [0, -1], [-1, -1], 
                                     [-1, 0], [-1, 1], [0, 1], [1, 1]];
        
        let mut moves: Vec<[i8; 2]> = Vec::new();

        let x = self.pos[1];
        let y = self.pos[0];

        for i in 0..8 {

            // bound check
            if y + changes[i][0] > 7 || y + changes[i][0] < 0 ||
               x + changes[i][1] > 7 || x + changes[i][1] < 0 {
                
                continue;
            }

            // is there a friendly
            if !sign_checker(self.key, board.get_piece(y + changes[i][0], x + changes[i][1])) {
                moves.push([y + changes[i][0], x + changes[i][1]])
            }
        }
        return moves;
    }

    fn get_key(&self) -> i8 {
        return self.key;
    }

    fn get_pos(&self) -> [i8; 2] {
        return self.pos;
    }
}

impl Moves for Queen {

    fn move_set(&self, board: &Board) -> Vec<[i8; 2]> {

        // Call tower + bishop 
        let t: Tower = Tower{pos: self.pos, key: self.key};
        let b: Bishop = Bishop{pos: self.pos, key: self.key};

        let mut moves: Vec<[i8; 2]> = b.move_set(board);
        let mut t_moves: Vec<[i8; 2]> = t.move_set(board);

        moves.append(&mut t_moves);
        return moves;
    }

    fn get_key(&self) -> i8 {
        return self.key;
    }

    fn get_pos(&self) -> [i8; 2] {
        return self.pos;
    }
}

/*
TODO:
Castling
Upgrade pawns
Add fn to remove moves that put player in check
Maintain pos of each piece/color in board?
Random must determine if its in check
Move piece_type to board
Fix random by calling in_check in choose_move
*/

// *************************************************************************************
// Test cases for private functions/structs

