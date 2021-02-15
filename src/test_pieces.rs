/*
Unit tests for the game

run with:  "-- --nocapture"

*/


#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::pieces::*;

    #[test]
    fn pawn_moves() {
        // Test if a pawn has the correct moves based on its pos and key
        
        // Create a standard board
        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 
        board.construct();

        let pawn: Box<dyn Moves> = piece_type(-1, [1, 0]);
        let pawn_1: Box<dyn Moves> = piece_type(1, [6, 5]);

        // default starting pos for black and white
        assert_eq!(pawn.move_set(&board), vec!([3, 0], [2, 0]));
        assert_eq!(pawn_1.move_set(&board), vec!([4, 5], [5, 5]));

        board.b[5][5] = -1;
        
        // test diagonal move with moving forward 2
        let pawn_2: Box<dyn Moves> = piece_type(1, [6, 4]);
        assert_eq!(pawn_2.move_set(&board), vec!([4, 4], [5, 4], [5, 5]));

        // Add testcase for no valid moves
        let pawn_3: Box<dyn Moves> = piece_type(-1, [0, 0]);
        assert_eq!(pawn_3.move_set(&board).len(), 0);

    }

    #[test] 
    fn tower_moves() {

        // Create a standard board
        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 
        board.construct();

        // No possible moves at starting position
        let tower: Box<dyn Moves> = piece_type(-2, [0, 0]);
        assert_eq!(tower.move_set(&board).len(), 0);

        // test bounds and enemy vs friendly detection
        let tower_2: Box<dyn Moves> = piece_type(-2, [5, 3]);

        assert_eq!(tower_2.move_set(&board), vec!([5, 4], [5, 2], [6, 3], [4, 3], 
                                                  [5, 5], [5, 1], [3, 3], [5, 6], 
                                                  [5, 0], [2, 3], [5, 7]));

        board.b[3][3] = -1;
        board.b[5][1] = 1;
        board.b[5][6] = 1;
        assert_eq!(tower_2.move_set(&board), vec!([5, 4], [5, 2], [6, 3], [4, 3], 
                                                  [5, 5], [5, 1], [5, 6]));

        // let tower_1: Box<dyn Moves> = piece_type(2, [7, 7]);
        // assert_eq!(tower_1.move_set(&board), empty);
    }
    #[test]
    fn knight_moves() {

        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 
        board.construct();

        // test default pos
        let knight: Box<dyn Moves> = piece_type(-3, [0, 2]);
        let knight_1: Box<dyn Moves> = piece_type(3, [7, 6]);

        // starting positions
        assert_eq!(knight.move_set(&board), vec!([2, 3], [2, 1]));
        assert_eq!(knight_1.move_set(&board), vec!([5, 7], [5, 5]));

        // test moves with friendly pieces in move pos
        let knight_2: Box<dyn Moves> = piece_type(-3, [4, 4]);
        let knight_3: Box<dyn Moves> = piece_type(3, [4, 4]);

        board.b[6][5] = -1;
        board.b[6][3] = -1;
        board.b[2][5] = -1;
        board.b[2][3] = -1;

        assert_eq!(knight_2.move_set(&board), vec!([5, 6], [3, 6], [5, 2], [3, 2]));
        assert_eq!(knight_3.move_set(&board), vec!([2, 5], [2, 3], [6, 5], [6, 3], 
                                                   [5, 6], [3, 6], [5, 2], [3, 2]));

    }
    #[test]
    fn bishop_moves() { 

        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 
        board.construct();

        // Test middle of the board
        let bishop: Box<dyn Moves> = piece_type(-4, [4, 4]);
        assert_eq!(bishop.move_set(&board), vec!([5, 5], [3, 5], [5, 3], [3, 3],
                                                 [6, 6], [2, 6], [6, 2], [2, 2]));

        // test starting position
        let bishop_1: Box<dyn Moves> = piece_type(-4, [0, 2]);
        let bishop_2: Box<dyn Moves> = piece_type(4, [7, 4]);

        assert_eq!(bishop_1.move_set(&board).len(), 0);
        assert_eq!(bishop_2.move_set(&board).len(), 0);
    }

    #[test]
    fn queen_moves() {

        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 

        board.construct();

        // test starting position
        let queen: Box<dyn Moves> = piece_type(6, [7, 4]);
        assert_eq!(queen.move_set(&board).len(), 0);

        // test horizontal and verticle movement
        let queen_1: Box<dyn Moves> = piece_type(6, [5, 0]);
        assert_eq!(queen_1.move_set(&board), vec!([4, 1], [3, 2], [2, 3], [1, 4], 
                                                  [5, 1], [4, 0], [5, 2], [3, 0], 
                                                  [5, 3], [2, 0], [5, 4], [1, 0], 
                                                  [5, 5], [5, 6], [5, 7]));

    }
    #[test]
    fn king_moves() {
        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 

        board.construct();

        // Starting pos should be empty
        let king: Box<dyn Moves> = piece_type(5, [7, 3]);
        assert_eq!(king.move_set(&board).len(), 0);

        // The pieces don't detect in check so I'm not putting 
        // those in this test case

        let king_1: Box<dyn Moves> = piece_type(5, [5, 0]);
        let king_2: Box<dyn Moves> = piece_type(-5, [4, 4]);

        assert_eq!(king_1.move_set(&board), vec!([4, 0], [4, 1], [5, 1]));
        assert_eq!(king_2.move_set(&board), vec!([5, 4], [5, 3], [4, 3], [3, 3], 
                                                 [3, 4], [3, 5], [4, 5], [5, 5]));
    }
}
