/*
Unit tests for the game

run with:  "-- --nocapture"

*/


#[cfg(test)]
mod tests {

    use crate::AI::eval::eval_board;
    use crate::board::Board;

    #[test]
    fn eval() {

        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 

        board.b[0][0] = 5;
        assert_eq!(eval_board(&board, 1), 19980);
        assert_eq!(eval_board(&board, -1), -20000);

        board.b[3][6] = -6;
        assert_eq!(eval_board(&board, 1), 19080);
        assert_eq!(eval_board(&board, -1), -19130);

        board.construct();
        board.b[3][6] = 0;

        // Black and white should have the same starting val
        assert_eq!(eval_board(&board, 1), eval_board(&board, -1));

    }
}
