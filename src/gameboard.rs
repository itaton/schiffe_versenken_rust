
struct Board {
    game_field:[[Block];Block],
    ships:[Ship],
    fields_shot:[[bool]bool],
    display: Display,
}

pub struct Block {
    x: u8,
    y: u8,
}

pub fn gameboard_init() -> Board {
    
}

pub fn calculate_touch_block(x: u16, y: u16) -> Block {
    if x<=272 && x>24 && y <= 272 && y > 24 {
        let x_block = x/25;
        let y_block = y/25;
        (x_block,y_block) //change to Block
    } else {
        (0,0)
    }
}

impl Board {
    pub fn setup_ship(&mut self, length: u8) {
        self.display.setup_ship(ship_len: u8)

    }
}
fn check_valid_ship(ship: Ship) {

}


pub fn check_win() -> bool {

}

pub fn shot_at(block: Block) -> (bool,bool) {

}

