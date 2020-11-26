/*
This file holds the classes of each piece and their respective moves
*/


pub struct Board {
    pub b: [[Box<dyn Moves>; 8]; 8]
}

impl Board {
    pub fn construct(&mut self) { //Fill the board with pieces
        // put the pawns on the board
        for i in 0..8 {
            let mut pawn: Pawn = Pawn {pos: [1, i], key: 1};
            let mut b_pawn: Pawn = Pawn {pos: [6, i], key: -1};

            // the compiler doesn't like i as an i32 for some reason
            let index: usize = i as usize;
            self.b[1][index] = Box::new(pawn);
            self.b[6][index] = Box::new(b_pawn);
        }

        // put non pawns on the board
        // TODO: 
        // Should try to loop or dynamically create the pieces like with the pawns

        // white non pawns
        let mut tower: Tower = Tower{pos: [0, 0], key: 2};
        let mut tower_2: Tower = Tower{pos: [0, 7], key: 2};

        let mut knight: Knight = Knight{pos: [0, 1], key: 3};
        let mut knight_2: Knight = Knight{pos: [0, 6], key: 3};

        let mut bishop: Bishop = Bishop{pos: [0, 2], key: 4};
        let mut bishop_2: Bishop = Bishop{pos: [0, 5], key: 4};

        let mut king: King = King{pos: [0, 3], key: 5};
        let mut queen: Queen = Queen{pos: [0, 4], key: 6};

        // black non pawns
        let mut b_tower: Tower = Tower{pos: [7, 0], key: -2};
        let mut b_tower_2: Tower = Tower{pos: [7, 7], key: -2};

        let mut b_knight: Knight = Knight{pos: [7, 1], key: -3};
        let mut b_knight_2: Knight = Knight{pos: [7, 6], key: -3};

        let mut b_bishop: Bishop = Bishop{pos: [7, 2], key: -4};
        let mut b_bishop_2: Bishop = Bishop{pos: [7, 5], key: -4};

        let mut b_king: King = King{pos: [7, 4], key: -5};
        let mut b_queen: Queen = Queen{pos: [7, 3], key: -6};

        // Again fix
        // let towers: [Tower; 4] = [tower, tower_2, b_tower, b_tower_2];
        // let knights: [Knight; 4] = [knight, knight_2, b_knight, b_knight_2];
        // let bishops: [Bishop; 4] = [bishop, bishop_2, b_bishop, b_bishop_2];

        // white
        self.b[0][0] = Box::new(tower);
        self.b[0][7] = Box::new(tower_2);
        self.b[0][1] = Box::new(knight);
        self.b[0][6] = Box::new(knight_2);
        self.b[0][2] = Box::new(bishop);
        self.b[0][5] = Box::new(bishop_2);
        self.b[0][3] = Box::new(king);
        self.b[0][4] = Box::new(queen);

        // black
        self.b[7][0] = Box::new(b_tower);
        self.b[7][7] = Box::new(b_tower_2);
        self.b[7][1] = Box::new(b_knight);
        self.b[7][6] = Box::new(b_knight_2);
        self.b[7][2] = Box::new(b_bishop);
        self.b[7][5] = Box::new(b_bishop_2);
        self.b[7][4] = Box::new(b_king);
        self.b[7][3] = Box::new(b_queen);

    }
}

pub struct Empty {
    pos: [i32; 2],
    key: i32
}

pub struct Pawn {
    pos: [i32; 2],
    pub key: i32
}

struct Tower {
    pos: [i32; 2],
    key: i32
}

struct Knight {
    pos: [i32; 2],
    key: i32
}

struct Bishop {
    pos: [i32; 2],
    key: i32
}

struct King {
    pos: [i32; 2],
    key: i32
}

pub struct Queen {
    pos: [i32; 2],
    pub key: i32
}

#[derive(Copy, Clone)]
pub trait Moves {
    // Empty methods to overwrite by each piece
    fn move_set(&self) -> Vec<[i32; 2]> {
        let val: Vec<[i32; 2]> = Vec::new();
        return val;
    }

    fn open_moves(&mut self, board: Board) {
        return;
    }

    fn get_key(&self) -> i32 {
        return 0;
    }
}

impl Moves for Empty {
    // Blank because we can just inherit from trait Moves
}

impl Moves for Pawn {
    // has 4 possible moves:
    // +2 (Special case), +1, +1-1, +1+1

    fn move_set(&self) -> Vec<[i32; 2]> {
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

        // Move forward 1 or attack diagonal 1
        all_moves.push([self.pos[0] + 1 * direction, self.pos[1]]);
        all_moves.push([self.pos[0] + 1 * direction, self.pos[1] - 1]);
        all_moves.push([self.pos[0] + 1 * direction, self.pos[1] + 1]);

        return all_moves;
    }

    // Calls move_set and removes impossible moves
    fn open_moves(&mut self, board: Board) -> () {
        let mut moves: Vec<[i32; 2]> = self.move_set();

    }

    fn get_key(&self) -> i32 {
        return self.key;
    }
}

impl Moves for Tower {
    fn open_moves(&mut self, board: Board){
        if self.key == -1 {
            println!("Yes");
        }

    }
}

impl Moves for Knight {
    fn open_moves(&mut self, board: Board) {
        if self.key == -1 {
            println!("Yes");
        }

    }
}

impl Moves for Bishop {
    fn open_moves(&mut self, board: Board) {
        if self.key == -1 {
            println!("Yes");
        }

    }
}

impl Moves for King {
    fn open_moves(&mut self, board: Board) {
        if self.key == -1 {
            println!("Yes");
        }
    }
}

impl Moves for Queen {
    fn open_moves(&mut self, board: Board) {
        if self.key == -1 {
            println!("Yes");
        }

    }
}


/* 
Cardinal Directions for movement


Each piece has a move_set and a key. The moveset is a Vector of strigs that holds the
instructions for moving each piece. For example, Pawn is ["f"]
It also holds a unique key that is representative of each piece of the board.
*/