
use crate::ships::Ship;
use crate::display::Display;
use alloc::vec::Vec;

pub struct Board {
    game_field:[[Block; 10];10],
    ships:[Ship; 5],
    fields_shot:[[bool; 10];10],
    display: Display,
    setup_field:[[bool; 10];10],
}

pub struct Block {
    pub x: u8,
    pub y: u8,
}

impl Board {
    pub fn new(game_field: [[Block; 10];10], ships: [Ship; 5], fields_shot:[[bool; 10];10], display: Display, setup_field:[[bool; 10];10]) -> Board {
        Board {
            game_field,
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
        while not ok_button {
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
        self.check_valid_ship(length);
        //check if len blocks selected
        //check if blocks in a row
        //?check if ship at a valid position?
        //Ship:new(Blocks);
        //return ship
    }

    fn check_valid_ship(&mut self, len: u8) -> Ship {
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

        //TODO: change this
        let x = 5



    }


    pub fn check_win() -> bool {

    }

    pub fn shot_at(block: Block) -> (bool,bool) {
        
    }
}


pub fn gameboard_init(display: Display) -> Board {
    //let mut ships : [Ship; 5] = [];
    let mut ships = Vec::new();
    display.setup_ship(5); //only display right side here
    //let ship: [Block; 5] = input_x();
    //check_valid_ship(ship, ships);
    //ships.push(ship);
    display.setup_ship(4);
    //let ship: [Block; 4] = input_x();
    //check_valid_ship(ship, ships);
    //ships.push(ship);
    display.setup_ship(3);
    //let ship: [Block; 3] = input_x();
    //check_valid_ship(ship, ships);
    //ships.push(ship);
    display.setup_ship(3);
    //let ship: [Block; 3] = input_x();
    //check_valid_ship(ship, ships);
    //ships.push(ship);
    display.setup_ship(2);
    //let ship: [Block; 2] = input_x();
    //check_valid_ship(ship, ships);
    //ships.push(ship);

    let fields_shot = [[false; 10];10];
    let setup_field = [[false; 10];10];

    //let game_field = //TODO initialize with the blocks

    Board::new(game_field, ships, fields_shot, display, setup_field)
}