
use crate::ships::Ship;
use crate::display::Display;

pub struct Board {
    game_field:[[Block; 10];10],
    ships:[Ship; 5],
    fields_shot:[[bool; 10];10],
    display: Display,
}

impl Board {
    pub fn new(game_field: [[Block; 10];10], ships: [Ship; 5], fields_shot:[[bool; 10];10], display: Display) -> Board {
        Board {
            game_field,
            ships,
            fields_shot,
            display,
        }
    }
}

pub struct Block {
    x: u8,
    y: u8,
}

pub fn gameboard_init(display: Display) -> Board {
    //let mut ships : [Ship] = [];
    display.setup_ship(5); //only display right side here
    //let ship: [Block; 5] = input_x();
    //check_valid_ship(ship, ships);
    //ships.push_back(ship);
    display.setup_ship(4);
    //let ship: [Block; 4] = input_x();
    //check_valid_ship(ship, ships);
    //ships.push_back(ship);
    display.setup_ship(3);
    //let ship: [Block; 3] = input_x();
    //check_valid_ship(ship, ships);
    //ships.push_back(ship);
    display.setup_ship(3);
    //let ship: [Block; 3] = input_x();
    //check_valid_ship(ship, ships);
    //ships.push_back(ship);
    display.setup_ship(2);
    //let ship: [Block; 2] = input_x();
    //check_valid_ship(ship, ships);
    //ships.push_back(ship);

    let fields_shot = [[false; 10];10];

    let game_field = //TODO initialize with the blocks

    board = Board::new(game_field, ships, fields_shot, display);
    return board;

    
}

pub fn calculate_touch_block(x: u16, y: u16) -> Block {
    if x<=272 && x>24 && y <= 272 && y > 24 {
        let x_block = x/25;
        let y_block = y/25;
        (x_block,y_block) //TODO: change to Block
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

