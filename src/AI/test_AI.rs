#[cfg(test)]
mod tests {

    use crate::AI::eval::eval_board;
    use crate::AI::random;
    use crate::board::Board;
    use crate::pieces::piece_type;

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

    #[test]
    fn another_eval() {

        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 

        // trival case
        assert_eq!(eval_board(&board, 1), eval_board(&board, -1));

        board.construct();

        // remove white queen, so black is in a better pos
        board.b[7][4] = 0;
        assert_eq!(eval_board(&board, -1), 19845);
        assert_eq!(eval_board(&board, 1), -20150);

        // remove black queen, so stength is equal
        board.b[0][3] = 0;
        assert_eq!(eval_board(&board, -1), eval_board(&board, 1));

    }

    #[test]
    fn random_opponent() {

        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 

        board.construct();

        // gen all moves for white

        for _j in 0..10 {

            let mut valid = false;
            let out: [[i8; 2]; 2] = random::choose_move(&mut board);
            let moves: Vec<[i8; 2]> = (piece_type(-1, out[0])).move_set(&board);

            for i in 0..moves.len() {
                if out[1] == moves[i] {
                    //println!("{:?} {:?}", out[1], moves);
                    valid = true;
                    break;
                }
            }
            //println!("{:?} {:?}", out[1], moves);
            if !valid { 
                // assert_eq!(false, true);
            }
        }

    }
}
