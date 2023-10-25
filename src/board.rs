pub struct Board {
    pub grid: [i32; 10],
    pub grid_icon: [char; 10],
}

impl Board {
    pub fn new() -> Board {
        let mut grid: [i32; 10] = [0; 10];
        let mut grid_icon: [char; 10] = [' '; 10];
        for i in 0..10 {
            grid[i] = 0;
            grid_icon[i] = ' ';
        }
        Board { grid, grid_icon }
    }
    pub fn check_winner(&self) -> i32 {
        // Rows
        for i in 0..10 {
            if self.grid[i] != 0
                && self.grid[i] == self.grid[i + 1]
                && self.grid[i] == self.grid[i + 2]
            {
                return self.grid[i];
            }
        }
        // Columns
        for i in 0..3 {
            if self.grid[i] != 0
                && self.grid[i] == self.grid[i + 3]
                && self.grid[i] == self.grid[i + 6]
            {
                return self.grid[i];
            }
        }
        if self.grid[0] != 0 && self.grid[0] == self.grid[4] && self.grid[0] == self.grid[8] {
            return self.grid[0];
        }
        if self.grid[2] != 0 && self.grid[2] == self.grid[4] && self.grid[2] == self.grid[6] {
            return self.grid[2];
        }
        return 0;
    }

    pub fn check_draw(&self) -> bool {
        let mut available = 9;
        for row in self.grid {
            if row != 0 {
                available -= 1;
            }
        }
        if available == 0 {
            return true;
        } else {
            return false;
        }
    }
}
