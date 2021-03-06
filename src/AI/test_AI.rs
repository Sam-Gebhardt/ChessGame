#[cfg(test)]
mod tests {

    use crate::AI::eval::eval_board;
    use crate::AI::min_max::select;
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
    fn random_opponent_starting() {

        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 

        board.construct();

        // test 10 random moves
        for _j in 0..10 {

            let mut valid = false;
            let out: [[i8; 2]; 2] = random::choose_move(&mut board);
            let num = board.get_piece(out[0][0], out[0][1]);
            let moves: Vec<[i8; 2]> = (piece_type(num, out[0])).move_set(&board);

            for i in 0..moves.len() {
                if out[1] == moves[i] {
                    valid = true;
                    break;
                }
            }
            board.print_b();
            if !valid { 
                assert_eq!(false, true);
            }
        }
        assert_eq!(true, true);
    }

    #[test]
    fn random_opponent() {

        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 

        board.construct();

        // Move some pieces around
        board.b[0][0] = 0;
        board.b[1][0] = 0;
        board.b[3][4] = -2;
        board.b[0][6] = 0;
        board.b[5][0] = -3;
        board.b[4][4] = 6;


        // test 10 random moves
        for _j in 0..10 {

            let mut valid = false;
            let out: [[i8; 2]; 2] = random::choose_move(&mut board);
            let num = board.get_piece(out[0][0], out[0][1]);
            let moves: Vec<[i8; 2]> = (piece_type(num, out[0])).move_set(&board);         

            for i in 0..moves.len() {
                if out[1] == moves[i] {
                    valid = true;
                    break;
                }
            }

            if !valid { 
                assert!(false);
            }
        }
        assert!(true);
    }

    #[test]
    fn min_max_open() {

        // AI should take best possible move

        let mut board = Board{
            b: [[0; 8]; 8],
            white: [0, 0],
            black: [0, 0]
        }; 

        board.construct();
        board.move_piece([6, 3], [5, 3]);

        select(&mut board);
        assert_eq!(board.b[1], [-1, -1, -1, 0, -1, -1, -1, -1]);
        assert_eq!(board.b[3], [0, 0, 0, -1, 0, 0, 0, 0]);

    }

    #[test]
    fn min_max_random() {


    }

    #[test]
    fn min_max_checkmate() {


    }

    #[test]
    fn min_max_save() {


    }
}
