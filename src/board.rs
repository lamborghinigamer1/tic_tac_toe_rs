pub struct Board {
    pub grid: [char; 10],
}

impl Board {
    pub fn new() -> Board {
        let mut grid: [char; 10] = [' '; 10];
        for i in 0..9 {
            grid[i] = ' ';
        }
        Board { grid }
    }

    pub fn check_winner(&self) -> char {
        for i in 0..3 {
            // Check rows
            if self.grid[i] != ' '
                && self.grid[i] == self.grid[i + 3]
                && self.grid[i + 3] == self.grid[i + 6]
            {
                return self.grid[i];
            }
            // Check columns
            if self.grid[i * 3] != ' '
                && self.grid[i * 3] == self.grid[i * 3 + 1]
                && self.grid[i * 3 + 1] == self.grid[i * 3 + 2]
            {
                return self.grid[i * 3];
            }
            // Check diagonals
            if self.grid[0] != ' ' && self.grid[0] == self.grid[4] && self.grid[4] == self.grid[8] {
                return self.grid[0];
            }

            if self.grid[2] != ' ' && self.grid[2] == self.grid[4] && self.grid[4] == self.grid[6] {
                return self.grid[2];
            }
        }
        return ' ';
    }

    pub fn check_draw(&self) -> bool {
        let mut available = 9;
        for &cell in &self.grid {
            if cell != ' ' {
                available -= 1;
            }
        }
        available == 0
    }
}
