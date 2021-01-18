/*
Unit tests for the game

run with -- --nocapture
*/


#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::pieces::*;

    // Test functions in board
    #[test]
    fn construct() {
        //Check to make sure board is built correctly

        // Create a board 
        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 

        // Construct the board
        board.construct();

        assert_eq!(board.b[0], [-2, -3, -4, -5, -6, -4, -3, -2]);
        assert_eq!(board.b[1], [-1, -1, -1, -1, -1, -1, -1, -1]);
        assert_eq!(board.b[2], [0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(board.b[3], [0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(board.b[4], [0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(board.b[5], [0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(board.b[6], [1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(board.b[7], [2, 3, 4, 6, 5, 4, 3, 2]);
    }

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
        assert_eq!(tower_2.move_set(&board), vec!([5, 4], [5, 2], [6, 3], [4, 3], [5, 5], [5, 1], [5, 6]));

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
        assert_eq!(knight_3.move_set(&board), vec!([2, 5], [2, 3], [6, 5], [6, 3], [5, 6], [3, 6], [5, 2], [3, 2]));

    }
    // #[test]
    // fn bishop_moves() { 

    //     let mut board = Board{
    //         b: [[0; 8]; 8],
    //         white: [0, 0],
    //         black: [0, 0]
    //     }; 
    //     board.construct();

    //     let bishop: Bishop = Bishop{pos: [4, 4], key: 4};
    //     assert_eq!(bishop.move_set(&board), vec!([5, 5], [3, 5], [5, 3], [3, 3], [6, 6], [2, 6], [6, 2], [2, 2]));

    //     let empty: Vec<[i8; 2]> = Vec::new();
    //     let bishop_1: Bishop = Bishop{pos: [0, 2], key: 4};
    //     let bishop_2: Bishop = Bishop{pos: [7, 2], key: -4};

    //     assert_eq!(bishop_1.move_set(&board), empty);
    //     assert_eq!(bishop_2.move_set(&board), empty);
    // }
    // #[test]
    // fn queen_moves() {

    // }
    // #[test]
    // fn king_moves() {
        
    // }

    // // #[test]
    // // fn check() {
    // //     let mut board = Board{
    // //         b: [[0; 8]; 8],
    // //         white: [0, 0],
    // //         black: [0, 0]
    // //     }; 
    
    // //     board.construct();
    // //     board.b[1][3] = -2;
    
    // //     assert_eq!(board.in_check([0, 0], [0, 0]), true);
    
    // // }
    // // fn mate() {
    // //     let mut board = Board{
    // //         b: [[0; 8]; 8],
    // //         white: [0, 0],
    // //         black: [0, 0]
    // //     }; 
    
    // //     board.b[0][0] = 5;
    // //     assert_eq!(board.check_mate(), false);
    // //     board.b[2][0] = -6;
    // //     board.b[0][2] = -6;
    // //     board.b[2][2] = -6;
    // //     assert_eq!(board.check_mate(), true);
    // // }
    // #[test]
    // fn stale() {
    //     let mut board = Board{
    //         b: [[0; 8]; 8],
    //         white: [0, 0],
    //         black: [0, 0]
    //     }; 
    //     board.construct();
    //     assert_eq!(board.stalemate(), false)

    // }

    // Testing functions in the AI directory
    use crate::AI::eval::eval_board;

    #[test]
    fn eval() {

        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 

        board.construct();

        // Black and white should have the same starting val
        assert_eq!(eval_board(&board, 1), eval_board(&board, -1));

    }
}