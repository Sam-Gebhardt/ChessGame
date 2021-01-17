/* 
https://www.chessprogramming.org/Simplified_Evaluation_Function
https://www.adamberent.com/2019/03/02/piece-square-table/

Evaluates the board to find a numerical representation for the strenght of a 
postion. In order to find this number each piece has a base strength along with a positive
or negative modifier based on their current place on the board.

Score = Material_value(White) – Material_value(Black) + Table_value(All the pieces on board).

*/
use crate::board::Board;

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
        [-20, -10, -10, -5, -5, -10, -10, -20],
        [-10, 0, 0, 0, 0, 0, 0, -10],
        [-10, 0, 5, 5, 5, 5, 0, -10],
        [-5, 0, 5, 5, 5, 5, 0, -5],
        [0, 0, 5, 5, 5, 5, 0, -5],
        [-10, 5, 5, 5, 5, 5, 0, -10],
        [-10, 0, 5, 0, 0, 0, 0, -10],
        [-20, -10, -10, -5, -5, -10, -10, -20]];

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


fn find_piece(key: i8, pos: [i8; 2]) -> i8 {

    // Queens/towers don't seem to have positioal advantage
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


// Evaluate the value of a piece based on its type 
// and position
pub fn eval_board(board: &Board) {

    find_piece(1, [3, 2]);

} 