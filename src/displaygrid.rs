use crate::board::Board;

pub struct DisplayGrid {
    pub board: Board,
}

impl DisplayGrid {
    pub fn new() -> DisplayGrid {
        let board = Board::new();
        DisplayGrid { board }
    }
    pub fn show_grid(&self) {
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");
        
        println!("Y |");
        for i in (0..3).rev() {
            print!("{}", i + 1);
            for j in 0..3 {
                let toeindex = i * 3 + j;
                if self.board.grid[toeindex] == 0 {
                    print!("[ ]");
                } else {
                    print!("[{}]", self.board.grid_icon[toeindex]);
                }
            }
            println!("");
        }
        print!("0 ");
        for i in 0..3 {
            print!("{}  ", i + 1);
        }
        println!("");
        print!("X -");
        println!("");
    }

    pub fn _show_grid_string(&self) -> String {
        let mut fullstring: String = String::new();
        fullstring.push_str("Y |\n");
        for i in (0..3).rev() {
            fullstring.push_str(&format!("{}", i + 1));
            for j in 0..3 {
                let toeindex = i * 3 + j;
                if self.board.grid[toeindex] == 0 {
                    fullstring.push_str("[ ]");
                } else {
                    fullstring.push_str(&format!("[{}]", self.board.grid_icon[toeindex]));
                }
            }
            fullstring.push_str("\n");
        }
        fullstring.push_str("0 ");
        for i in 0..3 {
            fullstring.push_str(&format!("{}  ", i + 1));
        }
        fullstring.push_str("\n");
        fullstring.push_str("X -");
        fullstring.push_str("\n");
        return fullstring;
    }
}
