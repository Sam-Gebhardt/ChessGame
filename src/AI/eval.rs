/* 
https://www.chessprogramming.org/Simplified_Evaluation_Function
https://www.adamberent.com/2019/03/02/piece-square-table/

Evaluates the board to find a numerical representation for the strenght of a 
postion. In order to find this number each piece has a base strength along with a positive
or negative modifier based on their current place on the board.

Score = base_value(White) – base_value(Black) + Table_value(All the pieces on board)
(From black's perspective it's same except * -1)

*/
use crate::board::Board;
use crate::pieces::sign_checker;


fn pawn(pos: [i8; 2]) -> i8 {

    let PawnTable: [[i8; 8]; 8] = [
        [0,  0,  0,  0,  0,  0,  0,  0],
        [50, 50, 50, 50, 50, 50, 50, 50],
        [10, 10, 20, 30, 30, 20, 10, 10],
        [5,  5, 10, 27, 27, 10,  5,  5],
        [0,  0,  0, 25, 25,  0,  0,  0],
        [5, -5,-10,  0,  0,-10, -5,  5],
        [5, 10, 10,-25,-25, 10, 10,  5],
        [0,  0,  0,  0,  0,  0,  0,  0]];

    return PawnTable[pos[0] as usize][pos[1] as usize];
}

fn tower(pos: [i8; 2]) -> i8 {

    let TowerTable: [[i8; 8]; 8] = [
        [0,  0,  0,  0,  0,  0,  0,  0],
        [5, 10, 10, 10, 10, 10, 10,  5],
        [-5,  0,  0,  0,  0,  0,  0, -5],
        [-5,  0,  0,  0,  0,  0,  0, -5],
        [-5,  0,  0,  0,  0,  0,  0, -5],
        [-5,  0,  0,  0,  0,  0,  0, -5],
        [-5,  0,  0,  0,  0,  0,  0, -5],
        [0,  0,  0,  5,  5,  0,  0,  0]];
    

    return TowerTable[pos[0] as usize][pos[1] as usize];
}

fn knight(pos: [i8; 2]) -> i8 {

    let KnightTable: [[i8; 8]; 8] = [
        [-50,-40,-30,-30,-30,-30,-40,-50],
        [-40,-20,  0,  0,  0,  0,-20,-40],
        [-30,  0, 10, 15, 15, 10,  0,-30],
        [-30,  5, 15, 20, 20, 15,  5,-30],
        [-30,  0, 15, 20, 20, 15,  0,-30],
        [-30,  5, 10, 15, 15, 10,  5,-30],
        [-40,-20,  0,  5,  5,  0,-20,-40],
        [-50,-40,-20,-30,-30,-20,-40,-50]];
    
    return KnightTable[pos[0] as usize][pos[1] as usize];
}

fn bishop(pos: [i8; 2]) -> i8 {

    let BishopTable: [[i8; 8]; 8] = [
        [-20,-10,-10,-10,-10,-10,-10,-20],
        [-10,  0,  0,  0,  0,  0,  0,-10],
        [-10,  0,  5, 10, 10,  5,  0,-10],
        [-10,  5,  5, 10, 10,  5,  5,-10],
        [-10,  0, 10, 10, 10, 10,  0,-10],
        [-10, 10, 10, 10, 10, 10, 10,-10],
        [-10,  5,  0,  0,  0,  0,  5,-10],
        [-20,-10,-40,-10,-10,-40,-10,-20]];

    return BishopTable[pos[0] as usize][pos[1] as usize];
}

fn queen(pos: [i8; 2]) -> i8 {

    let QueenTable: [[i8; 8]; 8] = [
        [-20,-10,-10,-5,-5,-10,-10,-20],
        [-10,  0, 0,  0, 0,  0,  0,-10],
        [-10,  0, 5,  5, 5,  5,  0,-10],
        [ -5,  0, 5,  5, 5,  5,  0, -5],
        [  0,  0, 5,  5, 5,  5,  0, -5],
        [-10,  5, 5,  5, 5,  5,  0,-10],
        [-10,  0, 5,  0, 0,  0,  0,-10],
        [-20,-10,-10,-5,-5,-10,-10,-20]];

    return QueenTable[pos[0] as usize][pos[1] as usize];
}

fn king(pos: [i8; 2]) -> i8 {

    let KingTable: [[i8; 8]; 8] = [
        [-30, -40, -40, -50, -50, -40, -40, -30],
        [-30, -40, -40, -50, -50, -40, -40, -30],
        [-30, -40, -40, -50, -50, -40, -40, -30],
        [-30, -40, -40, -50, -50, -40, -40, -30],
        [-20, -30, -30, -40, -40, -30, -30, -20],
        [-10, -20, -20, -20, -20, -20, -20, -10], 
        [20,  20,   0,   0,   0,   0,  20,  20],
        [20,  30,  10,   0,   0,  10,  30,  20]];

    return KingTable[pos[0] as usize][pos[1] as usize];
}


// Both sides have no queens or
// Every side which has a queen has additionally no other pieces or one minorpiece maximum.

    // let KingTableEndGame: [[i8; 8]; 8] = [
    //     [-50,-40,-30,-20,-20,-30,-40,-50],
    //     [-30,-20,-10,  0,  0,-10,-20,-30],
    //     [-30,-10, 20, 30, 30, 20,-10,-30],
    //     [-30,-10, 30, 40, 40, 30,-10,-30],
    //     [-30,-10, 30, 40, 40, 30,-10,-30],
    //     [-30,-10, 20, 30, 30, 20,-10,-30],
    //     [-30,-30,  0,  0,  0,  0,-30,-30],
    //     [-50,-30,-30,-30,-30,-30,-30,-50]];


fn table_value(key: i8, pos: [i8; 2]) -> i8 {

    let p = match key {
        1 => pawn(pos),
        2 => tower(pos),
        3 => knight(pos),
        4 => bishop(pos),
        5 => queen(pos),
        6 => king(pos), 
        _ => 0
    };
    
    return p;
}


// The base value of a piece reguardless of position
fn base_value(key: i8) -> i32 {

    let p = match key {
        1 => 100,
        2 => 500,
        3 => 320,
        4 => 330,
        5 => 900,
        6 => 20000, 
        _ =>  100
    };
    return p;
}


/*
Evaluate the value of a board
color: which color to eval the given board for
    ie if 1, eval for white, -1 for black 
*/
pub fn eval_board(board: &Board, color: i8) -> i32{

    let mut black: i32 = 0;
    let mut white: i32 = 0; 
    let mut board_value: i32 = 0;
    let mut key: i8;

    for i in 0..8 {
        for j in 0..8 {
            key = board.get_piece(i, j);

            if key > 0 {
                white += base_value(key);
            } else if key < 0 {
                black += base_value(key * -1);
            }

            board_value += table_value(key, [i, j]) as i32;
        }
    }

    if color == 1 {
        // println!("White: W-{} B-{} val {}", white, black, board_value);
        return (white - black) + board_value;
    }
    // println!("Black: W-{} B-{} val {}", white, black, board_value);

    return ((white - black) * -1) + board_value;
} 
