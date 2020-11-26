
struct Board {
    b: [[i32; 8]; 8]
}


impl Board {
    fn construct(&mut self) {
        for i in 0..8 {
            self.b[1][i] = 1;
            self.b[6][i] = -1;
        }

        let mut start = 2;
        let mut change = 1;
        for i in 0..8 {
            self.b[0][i] = start;
            self.b[7][i] = start * -1;
            start += change;
            if start == 7 {
                start -= 3;
                change = -1; 
            }
        }
        self.b[7][3] = -6;
        self.b[7][4] = -5;
        }
}

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
