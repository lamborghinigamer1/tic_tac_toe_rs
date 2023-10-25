pub struct Player {
    pub name: String,
    pub icon: char,
}

impl Player {
    pub fn new(name: String, icon: char) -> Player {
        Player { name, icon }
    }
}
