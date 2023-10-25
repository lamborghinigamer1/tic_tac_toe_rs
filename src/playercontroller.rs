use crate::{displaygrid::DisplayGrid, player::Player};

pub struct Playercontroller {
    game: DisplayGrid,
    players: Vec<Player>,
}

impl Playercontroller {
    pub fn new(game: DisplayGrid) -> Playercontroller {
        let players: Vec<Player> = Vec::new();

        Playercontroller { game, players }
    }
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
    pub fn play_game(&mut self) {
        while self.game.board.check_winner() == 0 || !self.game.board.check_draw() {
            let mut x: i32;
            let mut y: i32;
            let mut selectedoption: usize;

            let mut playernumber: i32 = 1;

            for player in &self.players {
                self.game.show_grid();
                println!("{} [{}] it's your turn", player.name, player.icon);
                loop {
                    loop {
                        println!("Select a number between 1 and 3 for the X");
                        let mut inputx = String::new();
                        std::io::stdin()
                            .read_line(&mut inputx)
                            .expect("Failed to read input");

                        let inputx = inputx.trim();

                        match inputx.parse::<i32>() {
                            Ok(parsed) if (1..=3).contains(&parsed) => {
                                x = parsed;
                                break;
                            }
                            _ => {
                                println!(
                                    "Invalid! Please select a number between 1 and 3 for the X"
                                );
                            }
                        }
                    }
                    loop {
                        println!("Select a number between 1 and 3 for the Y");
                        let mut inputy = String::new();
                        std::io::stdin()
                            .read_line(&mut inputy)
                            .expect("Failed to read input");

                        let inputx = inputy.trim();

                        match inputx.parse::<i32>() {
                            Ok(parsed) if (1..=3).contains(&parsed) => {
                                y = parsed;
                                break;
                            }
                            _ => {
                                println!(
                                    "Invalid! Please select a number between 1 and 3 for the Y"
                                );
                            }
                        }
                    }

                    selectedoption = ((y - 1) * 3 + (x - 1)) as usize;

                    if self.game.board.grid[selectedoption] != 0 {
                        println!("Position already in use");
                    } else {
                        break;
                    }
                }
                self.game.board.grid[selectedoption] = playernumber;
                self.game.board.grid_icon[selectedoption] = player.icon;
                if self.game.board.check_winner() != 0 {
                    break;
                }
                if self.game.board.check_draw() {
                    break;
                }
                playernumber += 1;
            }
            if self.game.board.check_winner() != 0 {
                break;
            }
            if self.game.board.check_draw() {
                break;
            }
        }
        self.game.show_grid();
        if self.game.board.check_draw() {
            println!("Draw!");
        } else {
            println!(
                "{} [{}] wins!",
                self.players[(self.game.board.check_winner() - 1) as usize].name,
                self.players[(self.game.board.check_winner() - 1) as usize].icon
            );
        }
    }
}
