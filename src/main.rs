mod board;
mod displaygrid;
mod player;
mod playercontroller;

fn main() {
    let mut controller = playercontroller::Playercontroller::new(displaygrid::DisplayGrid::new());

    controller.add_player(player::Player::new("Player 1".to_string(), 'X'));
    controller.add_player(player::Player::new("Player 2".to_string(), 'O'));

    controller.play_game();
}
