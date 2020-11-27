
#[cfg(test)]
mod tests {
    use crate::pieces::Board;
    use crate::pieces::*;
    // use std::io;
    // use std::io::Write;

    // use pieces;
    #[test]
    fn construct() {
        //Check to make sure board is built correctly

        // Create a board 
        let mut board = Board{
            b: [[0; 8]; 8]
        }; 

        // Construct the board
        board.construct();

        assert_eq!(board.b[0], [2, 3, 4, 5, 6, 4, 3, 2]);
        assert_eq!(board.b[1], [1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(board.b[2], [0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(board.b[3], [0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(board.b[4], [0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(board.b[5], [0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(board.b[6], [-1, -1, -1, -1, -1, -1, -1, -1]);
        assert_eq!(board.b[7], [-2, -3, -4, -6, -5, -4, -3, -2]);
    }

    #[test]
    fn pawn_moves() {
        // Test if a pawn has the correct moves based on its pos and key
        
        // Create a standard board
        let mut board = Board{
            b: [[0; 8]; 8]
        }; 
        board.construct();

        let pawn: Pawn = Pawn{pos: [1, 0], key: 1};
        let pawn_1: Pawn = Pawn{pos: [6, 5], key: -1};

        // default starting pos for black and white
        assert_eq!(pawn.move_set(&board), vec!([3, 0], [2, 0]));
        assert_eq!(pawn_1.move_set(&board), vec!([4, 5], [5, 5]));

        board.b[5][5] = 1;

        println!("{}", board.b[5][5]);
        // run with -- --nocapture
        
        // test diagonal move with moving forward 2
        let pawn_2: Pawn = Pawn{pos: [6, 4], key: -1};
        assert_eq!(pawn_2.move_set(&board), vec!([4, 4], [5, 4], [5, 5]));

        // Add testcase for no valid moves
        // let pawn_3: Pawn = Pawn{pos: [1, 0], key: 1};
        // let pawn_4: Pawn = Pawn{pos: [1, 0], key: 1};
    }
    #[test]
    fn tower_moves() {
        // Create a standard board
        let mut board = Board{
            b: [[0; 8]; 8]
        }; 
        board.construct();

        // No possible moves at starting position
        let tower: Tower = Tower{pos: [0, 0], key: 2};
        let tower_1: Tower = Tower{pos: [7, 7], key: -2};

        let empty: Vec<[i8; 2]> = Vec::new();
        assert_eq!(tower.move_set(&board), empty);
        assert_eq!(tower_1.move_set(&board), empty);

    }
}

