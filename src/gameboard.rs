
use crate::ships::Ship;
use crate::display::Display;
use alloc::vec::Vec;

pub struct Board {
//    game_field:[[Block; 10];10],
    //ships:[Ship; 5],
    ships:Vec<Ship>,
    fields_shot:[[bool; 10];10],
    display: Display,
    setup_field:[[bool; 10];10],
}

pub struct Block {
    pub x: u8,
    pub y: u8,
}

impl Board {
    //pub fn new(game_field: [[Block; 10];10], ships: Vec<Ship>, fields_shot:[[bool; 10];10], display: Display, setup_field:[[bool; 10];10]) -> Board {
    pub fn new(ships: Vec<Ship>, fields_shot:[[bool; 10];10], display: Display, setup_field:[[bool; 10];10]) -> Board {
        Board {
            //game_field,
            ships,
            fields_shot,
            display,
            setup_field,
        }
    }


    pub fn calculate_touch_block(&mut self, x: u16, y: u16) -> Option<Block>{
        if x<=272 && x>24 && y <= 272 && y > 24 {
            let x_block = x/25;
            let y_block = y/25;
            assert!(x_block <=255);
            assert!(y_block <=255);
            Some(Block {x: x_block as u8, y: y_block as u8})
        } else {
            None
        }
    }

    pub fn setup_ship(&mut self, length: u8) {
        self.display.setup_ship(length);
        while self.display.touch_confirm_button() == false {
            //touch loop
            let (x,y) = self.display.touch();
            match self.calculate_touch_block(x, y) {
                None => {},
                Some(block) => {
                    if self.setup_field[block.x as usize][block.y as usize] == false {
                        self.setup_field[block.x as usize][block.y as usize] = true;
                        self.display.write_in_field(block.x as usize, block.y as usize, "x");
                    }
                    else {
                        self.setup_field[block.x as usize][block.y as usize] = false;
                        self.display.write_in_field(block.x as usize, block.y as usize, " ");
                    }
                }
            }
        }
        self.get_valid_ship(length);
        //check if len blocks selected
        //check if blocks in a row
        //?check if ship at a valid position?
        //Ship:new(Blocks);
        //return ship
    }

    fn get_valid_ship(&mut self, len: u8) -> Ship {

        let mut x_start = 0;
        let mut y_start = 0;

        //check if length is correct
        let mut marked_fields = 0;
        for i in 1..=10 {
            for j in 1..=10{
                if self.setup_field[i][j] {
                    marked_fields = marked_fields + 1;
                }
            }
        }
        if marked_fields != len {
            //Error - TODO what to do here
        }

        //check if ship is in a line
        let mut x_pos = 0;
        let mut y_pos = 0;
        let mut found = false;
        let mut vertical = false;
        let mut direction_known = false;
        for i in 1..=10 {
            for j in 1..=10 {
                if self.setup_field[i][j] {
                    if found == false {
                        found = true;
                        x_start = i; //for ship init
                        y_start = j; //for ship init
                        x_pos = i;
                        y_pos = j;
                    }
                    else {
                        if i != x_pos + 1 || j != y_pos + 1 {
                            //Error, the next block is not adjacent to the previous - TODO what to do here
                        }
                        if direction_known == false {
                            if i == x_pos + 1 {
                                vertical = false;
                            }
                            else {
                                vertical = true;
                            }
                            direction_known = true;
                            x_pos = i;
                            y_pos = j;
                        } 
                        else {
                            if vertical == false {
                                if i != x_pos + 1 {
                                    //Error, next block is at the wrong location
                                }
                                x_pos = i;
                            }
                            else {
                                if j != y_pos + 1 {
                                    //Error, next block is at the wrong location
                                }
                                y_pos = j;
                            }
                        }
                    }
                }
            }
        }


        //check if ship not adjacent to existing ship
        //TODO



        self.display.print_ship(len as usize, x_start, y_start, vertical);
        let ship = Ship::new(len, x_start as u8, y_start as u8, vertical);
        ship
        //TODO: change this
        //let x = 5



    }


    pub fn check_win(&mut self) -> bool {
        for ship in self.ships.iter() {
            //if ship.sunk() == false {
            //    return false;
            //}
        }
        //true

        //stub:
        false
    }

    //returns if hit and if sunk
    pub fn shot_at(block: Block) -> (bool,bool) {
        return (false, false);
    }

    pub fn initial_setup(&mut self) {

        self.display.setup_ship(5); //only display right side here
        //let ship: [Block; 5] = input_x();
        //get_valid_ship(5);
        //ships.push(ship);
        self.display.setup_ship(4);
        //let ship: [Block; 4] = input_x();
        //get_valid_ship(ship, ships);
        //ships.push(ship);
        self.display.setup_ship(3);
        //let ship: [Block; 3] = input_x();
        //get_valid_ship(ship, ships);
        //ships.push(ship);
        self.display.setup_ship(3);
        //let ship: [Block; 3] = input_x();
        //get_valid_ship(ship, ships);
        //ships.push(ship);
        self.display.setup_ship(2);
        //let ship: [Block; 2] = input_x();
        //get_valid_ship(ship, ships);
        //ships.push(ship);

    }
}


pub fn gameboard_init(display: Display) -> Board {
    //let mut ships : [Ship; 5] = [];
    let mut ships = Vec::new();

    let fields_shot = [[false; 10];10];
    let setup_field = [[false; 10];10];

    //Board::new(game_field, ships, fields_shot, display, setup_field)
    let mut board = Board::new(ships, fields_shot, display, setup_field);

    board.initial_setup();
    board

    //let game_field = //TODO initialize with the blocks

}