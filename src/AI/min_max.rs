/*
Chess AI that is built upon min/max with alpha pruning 
https://www.chessprogramming.org/Minimax
*/

fn eval_board() {
    

}

fn max(depth: int, moves: Vec<[[i8; 2]; [i8; 2]>) {

    if depth == 0 {
        return; // An eval function that determines the value of the board
    }
    // init max as a small number
    let mut int max = -9999999;
    let mut score;

    for i in 0..moves.len() {
        score = min(depth - 1);

        if score > max {
            max = score;
        }
    }
    return max;
}

fn min(depth: int, moves: Vec<[[i8; 2]; [i8; 2]>) {

    if depth == 0 {
        return; // An eval function that determines the value of the board
    }
    // init max as a large number
    let mut int min = 9999999;
    let mut int score;

    for i in 0..moves.len() {
        score = max(depth - 1);

        if score < min {
            min = score;
        }
    }
    return min;
}