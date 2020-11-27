/*
This file holds the classes of each piece and their respective moves
*/


pub struct Board {
    pub b: [[i32; 8]; 8]
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
        for i in 0..8 {
            self.b[0][i] = start;
            self.b[7][i] = start * -1;
            start += 1;
            if start == 6 {
                start -= 2;
            }
        }
        // King and queen should be on opposite sides
        self.b[7][3] = -6;
        self.b[7][4] = -5;
    }

    pub fn get_piece(&self, one: i32, two: i32) -> i32 {
        // Get a piece from the board at b[one][two]

        // Cast to correct type for indexing
        let one_usize: usize = one as usize;
        let two_usize: usize = two as usize;

        return self.b[one_usize][two_usize];

    }
    pub fn bound_check(&self, moves: Vec<[i32; 2]>) -> Vec<[i32; 2]> {
        // Checks two things:
        // 1: is move on the board ( 0 =< x/y <= 7)
        // 2: is their a friendly piece at the position

        let mut valid: Vec<[i32; 2]> = Vec::new();
        for i in 0..moves.len() {
            if (8 > moves[i][0] && moves[i][0] > -1) && (8 > moves[i][1] && moves[i][1] > -1) {
                valid.push(moves[i]);
            }
        }

        let mut empty: Vec<[i32; 2]> = Vec::new();
        for i in 0..valid.len() {
            if self.b[valid[i][0] as usize][valid[i][1] as usize] == 0 {
                empty.push(valid[i]);
            } 
        }
        return valid;
    }
}


pub struct Pawn {
    pub pos: [i32; 2],
    pub key: i32
}

struct Tower {
    pos: [i32; 2],
    key: i32
}

// struct Knight {
//     pos: [i32; 2],
//     key: i32
// }

// struct Bishop {
//     pos: [i32; 2],
//     key: i32
// }

// struct King {
//     pos: [i32; 2],
//     key: i32
// }

// pub struct Queen {
//     pos: [i32; 2],
//     pub key: i32
// }

fn sign_checker(one: i32, two: i32) -> bool {
    // return true if the numbers are the same sign
    // else return false

    if one > 0 && two > 0 {
        return true;
    } else if one < 0 && two < 0 {
        return true;
    } return false;
}

pub trait Moves {
    // Empty methods to overwrite by each piece
    fn move_set(&self, board: &Board) -> Vec<[i32; 2]> {
        let val: Vec<[i32; 2]> = Vec::new();
        return val;
    }

    fn open_moves(&mut self, board: Board) {
        return;
    }
}


impl Moves for Pawn {
    // has 4 possible moves:
    // +2 (Special case), +1, +1-1, +1+1

    fn move_set(&self, board: &Board) -> Vec<[i32; 2]> {
        let mut direction = 1; 

        if self.key == -1 {
            direction = -1;
        }
        // A vector that holds all possible moves for a Pawn at pos [x][y]
        let mut all_moves: Vec<[i32; 2]> = Vec::new();

        // Check if Pawn can move 2 spaces
        if (self.pos[1] == 6 && self.key == -1) || self.pos[1] == 1 {
            all_moves.push([self.pos[0] + direction * 2, self.pos[1]]);
        }

        // Move forward 1 
        all_moves.push([self.pos[0] + 1 * direction, self.pos[1]]);

        // Check if an oppenent is in the diagonal

        // Pawn has attack seperate from regular move, so i'll do bound 
        // checking within the function
        let mut valid: Vec<[i32; 2]> = Vec::new();
        if (self.pos[1] + 1) != 8 {
            valid.push([self.pos[0] + 1 * direction, self.pos[1] + 1]);
        } if (self.pos[1] - 1) != -1 {
            valid.push([self.pos[0] + 1 * direction, self.pos[1] - 1]);
        }

        // Now that we have the valid moves, check if they are legal
        for i in 0..valid.len() {
            let diagonal: i32 = board.get_piece(valid[i][0], valid[i][1]);
            if sign_checker(self.key, diagonal) {
                all_moves.push(valid[i]);
            }
        }
        return all_moves;
    }
}

impl Moves for Tower {
    fn open_moves(&mut self, board: Board){
        if self.key == -1 {
            println!("Yes");
        }

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