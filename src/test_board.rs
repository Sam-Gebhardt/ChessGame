/*
run with:  "-- --nocapture"
*/

#[cfg(test)]
mod tests {
    use crate::board::Board;

    // Tests for board.rs
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
    fn king_tracker() {

        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 
    
        board.construct();

        // move isn't possible, but checking that King's
        // pos is correctly tracked
        board.move_piece([0, 3], [3, 3]);
        assert_eq!(board.black, [3, 3]);
        assert_eq!(board.b[0], [-2, -3, -4, 0, -6, -4, -3, -2]);
        assert_eq!(board.b[3], [0, 0, 0, -5, 0, 0, 0, 0]);

    }

    #[test]
    fn in_check() {

        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 
    
        board.construct();
        board.b[1][3] = 2;
    
        assert_eq!(board.in_check([3, 3], [3, 3], -1), true);

        board.b[1][3] = 0;
        board.b[3][2] = 2;
        assert_eq!(board.in_check([3, 2], [1, 3], -1), true);

        // reset board to all 0
        board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 

        board.b[0][0] = 5;
        board.b[2][2] = -2;
        board.b[1][7] = -2;

        assert_eq!(board.in_check([7, 7], [7, 7], 1), false);
        assert_eq!(board.in_check([2, 2], [0, 2], 1), true);

    }
    // Check mate is currently buggy, so Ill wait 
    // to implement a test case till its working
    #[test]
    fn checkmate_simple() {
        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 
    
        board.b[0][0] = 5;
        assert_eq!(board.check_mate(5), false);
        board.b[2][0] = -6;
        board.b[0][2] = -6;
        board.b[2][2] = -6;
        assert_eq!(board.check_mate(5), true);
    }

    #[test]
    fn checkmate_complex() {

        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 

        // David and Goliath Mate
        board.b[2][1] = 2;
        board.b[5][5] = 5;
        board.b[5][6] = 1;
        board.b[5][7] = 1;

        board.b[3][5] = -1;
        board.b[3][7] = -1;
        board.b[3][6] = -5;

    }

    #[test]
    fn checkmate_complex_2() {

        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 

        board.construct();

        // fool's checkmate
        board.b[1][1] = 0;
        board.b[1][2] = 0;
        board.b[1][3] = -1;
        board.b[2][2] = -1;
        board.b[4][3] = 1;
        board.b[6][3] = 0;

        board.check_mate(-5);

    }
    #[test]
    fn stalemate_simple() {
        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 

        board.b[0][0] = 5;
        board.b[1][7] = -2;
        board.b[7][1] = -2;
        assert_eq!(board.stalemate(1), true);

        board.construct();
        assert_eq!(board.stalemate(1), false);

    }
    #[test]
    fn stalemate_complex() {

        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 

        board.b[0][6] = -5;
        board.b[1][0] = -4;
        board.b[4][5] = -1;
        board.b[5][7] = -1;

        board.b[6][7] = 1;
        board.b[7][7] = 5;

        // assert_eq!(board.stalemate(1), false);
    }
}
