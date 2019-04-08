use crate::display::Display;
use crate::ships::Ship;
use alloc::vec::Vec;
use stm32f7_discovery::system_clock::{self, Hz};

pub struct Board {
    //game_field:[[Block; 10];10],
    //ships:[Ship; 5],
    ships: Vec<Ship>,
    fields_shot: [[bool; 10]; 10],
    //display: Display,
    setup_field: [[bool; 10]; 10],
    //TODO: maybe do this more beautiful?
    placed_ships: [[bool; 10]; 10],
}

#[derive(Copy, Clone)]
pub struct Block {
    pub x: u8,
    pub y: u8,
}

impl Board {
    pub fn new(
        ships: Vec<Ship>,
        fields_shot: [[bool; 10]; 10],
        //display: Display,
        setup_field: [[bool; 10]; 10],
        placed_ships: [[bool; 10]; 10],
    ) -> Board {
        Board {
            //game_field,
            ships,
            fields_shot,
            //display,
            setup_field,
            placed_ships,
        }
    }

    pub fn calculate_touch_block(&mut self, x: u16, y: u16) -> Option<Block> {
        if x <= 272 && x > 24 && y <= 272 && y > 24 {
            let x_block = x / 25;
            let y_block = y / 25;
            assert!(x_block <= 255);
            assert!(y_block <= 255);
            Some(Block {
                x: x_block as u8,
                y: y_block as u8,
            })
        } else {
            None
        }
    }

    pub fn setup_ship(&mut self, length: u8, display: &mut Display) {
        //self.display.setup_ship(length);
        display.setup_ship(length);
        let mut confirmed = false;
        //let ticks = system_clock::ticks();
        //while system_clock::ticks() - ticks <= 5 {}
        while !confirmed {
        // display.check_confirm_button_touched() {

            //touch loop
            // let ticks = system_clock::ticks();
            // while system_clock::ticks() - ticks <= 5 {}
            let (x, y) = display.touch();
            match self.calculate_touch_block(x, y) {
                None => {
                    if display.check_confirm_button_touched(x,y) {
                        confirmed = true;
                    }
                }
                Some(block) => {
                    let (x, y) = (block.x - 1, block.y - 1);
                    if !self.setup_field[x as usize][y as usize] {
                        self.setup_field[x as usize][y as usize] = true;
                        display
                            .write_in_field(block.x as usize, block.y as usize, "x");
                    } else {
                        self.setup_field[x as usize][y as usize] = false;
                        display
                            .write_in_field(block.x as usize, block.y as usize, " ");
                    }
                }
            }
        }
        //let ticks = system_clock::ticks();
        //while system_clock::ticks() - ticks <= 3 {}
        match self.get_valid_ship(length, display) {
            None => {
                self.clear_x_es(display);
                self.setup_field = [[false; 10]; 10];
                self.setup_ship(length, display);
            }
            Some(ship) => {
                self.clear_x_es(display);
                self.setup_field = [[false; 10]; 10];
                self.ships.push(ship);
            }
        }
    }

    fn get_valid_ship(&mut self, len: u8, display: &mut Display) -> Option<Ship> {
        let mut x_start = 0;
        let mut y_start = 0;

        //check if length is correct
        let mut marked_fields = 0;
        for i in 0..10 {
            for j in 0..10 {
                if self.setup_field[i][j] {
                    marked_fields += 1;
                }
            }
        }
        if marked_fields != len {
            //wrong number of blocks set
            return None;
        }

        //check if ship is in a line
        let mut x_pos = 0;
        let mut y_pos = 0;
        let mut found = false;
        let mut vertical = false;
        let mut direction_known = false;
        for i in 0..10 {
            for j in 0..10 {
                if self.setup_field[i][j] {
                    if !found {
                        //find start field of the ship
                        found = true;
                        x_start = i; //for ship init
                        y_start = j; //for ship init
                        x_pos = i;
                        y_pos = j;
                    } else {
                        if i != x_pos + 1 && j != y_pos + 1 {
                            // Next block is neither right nor below the previous
                            return None;
                        }
                        if !direction_known {
                            //find
                            if i == x_pos + 1 {
                                vertical = false;
                            } else {
                                vertical = true;
                            }
                            direction_known = true;
                            x_pos = i;
                            y_pos = j;
                        } else {
                            if !vertical {
                                if i != x_pos + 1 || j != y_pos {
                                    //Error, next block is at the wrong location
                                    return None;
                                }
                                x_pos = i;
                            } else {
                                if j != y_pos + 1 || i != x_pos {
                                    //Error, next block is at the wrong location
                                    return None;
                                }
                                y_pos = j;
                            }
                        }
                    }
                }
            }
        }

        //check if ship not adjacent to existing ship
        for i in 0..10 {
            for j in 0..10 {
                if self.setup_field[i][j] {
                    // i love R
                    for k in if i == 0 {
                        0..=1
                    } else if i == 9 {
                        8..=9
                    } else {
                        i - 1..=i + 1
                    } {
                        for l in if j == 0 {
                            0..=1
                        } else if j == 9 {
                            8..=9
                        } else {
                            j - 1..=j + 1
                        } {
                            if self.placed_ships[k][l] {
                                return None;
                            }
                        }
                    }
                }
            }
        }

        //save ship - TODO maybe do this nicer via Ship or sth. Quick and dirty solution
        for i in 0..10 {
            for j in 0..10 {
                if self.setup_field[i][j] {
                    self.placed_ships[i][j] = true;
                }
            }
        }

        display
            .print_ship(len as usize, x_start + 1, y_start + 1, vertical);
        let ship = Ship::new(len, x_start as u8, y_start as u8, vertical);
        Some(ship)
    }

    pub fn clear_x_es(&mut self, display: &mut Display) {
        for i in 1..11 {
            for j in 1..11 {
                display.write_in_field(i, j, " ");
            }
        }
    }

    pub fn check_win(&mut self) -> bool {
        for ship in self.ships.iter() {
            if ship.size != ship.sunken_fields {
                return true;
            }
        }
        false
    }

    //returns if hit and if sunk
    pub fn shoot_at(&mut self, block: Block) -> (bool, bool) {
        //todo check index
        if !self.fields_shot[block.x as usize - 1][block.y as usize - 1]
            && self.placed_ships[block.x as usize - 1][block.y as usize - 1]
        {
            //ship = getShip(x,y)
            match self.get_ship_at(block.x - 1, block.y - 1) {
                None => {
                    return (false, false);
                }
                Some(mut ship) => {
                    ship.sunken_fields += 1; //TODO am I really working on the ship from the vector here or on a copy?
                    if ship.sunken_fields == ship.size {
                        return (true, true);
                    } else {
                        return (true, false);
                    }
                }
            }
        }
        (false, false)
    }

    fn get_ship_at(&mut self, x: u8, y: u8) -> Option<&mut Ship> {
        for ship in self.ships.iter_mut() {
            if ship.vertical {
                if x != ship.x_start_location {
                    continue;
                }
                for i in 0..ship.size {
                    if y == ship.y_start_location + i {
                        return Some(ship);
                    }
                }
            } else {
                if y != ship.y_start_location {
                    continue;
                }
                for i in 0..ship.size {
                    if y == ship.y_start_location + i {
                        return Some(ship);
                    }
                }
            }
        }
        None
    }

    pub fn initial_setup(&mut self, display: &mut Display) {
        display.setup_ship(5); //only display right side here
        self.setup_ship(5, display);
        //let ship: [Block; 5] = input_x();
        //get_valid_ship(5);
        //ships.push(ship);
        display.setup_ship(4);
        self.setup_ship(4, display);
        //let ship: [Block; 4] = input_x();
        //get_valid_ship(ship, ships);
        //ships.push(ship);
        display.setup_ship(3);
        self.setup_ship(3, display);
        //let ship: [Block; 3] = input_x();
        //get_valid_ship(ship, ships);
        //ships.push(ship);
        display.setup_ship(3);
        self.setup_ship(3, display);
        //let ship: [Block; 3] = input_x();
        //get_valid_ship(ship, ships);
        //ships.push(ship);
        display.setup_ship(2);
        self.setup_ship(2, display);
        //let ship: [Block; 2] = input_x();
        //get_valid_ship(ship, ships);
        //ships.push(ship);
    }
}

pub fn gameboard_init() -> Board {
    //let mut ships : [Ship; 5] = [];
    let mut ships = Vec::new();

    let fields_shot = [[false; 10]; 10];
    let setup_field = [[false; 10]; 10];
    let placed_ships = [[false; 10]; 10];

    //Board::new(game_field, ships, fields_shot, display, setup_field)
    // let mut board = Board::new(ships, fields_shot, display, setup_field, placed_ships);
    let mut board = Board::new(ships, fields_shot, setup_field, placed_ships);

    // board.initial_setup(display);
    board

    //let game_field = //TODO initialize with the blocks
}
