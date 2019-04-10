use crate::display::Display;
use crate::ships::Ship;
use alloc::vec::Vec;
// use stm32f7_discovery::system_clock::{self, Hz};

pub struct Board {
    ships: Vec<Ship>,
    fields_shot: [[bool; 10]; 10],
    setup_field: [[bool; 10]; 10], //temporary setup field, cleared after successful ship setup
    placed_ships: [[bool; 10]; 10], //holds all placed ships for adjacency checks
    enemy_ships_hit: [[bool; 10]; 10],
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
        setup_field: [[bool; 10]; 10],
        placed_ships: [[bool; 10]; 10],
    ) -> Board {
        Board {
            ships,
            fields_shot,
            setup_field,
            placed_ships,
            enemy_ships_hit: [[false; 10]; 10],
        }
    }

    /**
     * return the block at position `x`, `y` on the display
     */
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

    /**
     * get the user input for setting up the ships, i.e. the x'es where the ship is supposed to be set up
     */
    pub fn setup_ship(&mut self, length: u8, display: &mut Display) {
        display.setup_ship(length); //This is basically double - maybe remove in the initBoard function
        let mut confirmed = false;
        while !confirmed {
            let (x, y) = display.touch();
            match self.calculate_touch_block(x, y) {
                None => {
                    if display.check_confirm_button_touched(x, y) {
                        confirmed = true;
                    }
                }
                Some(block) => {
                    let (x, y) = (block.x - 1, block.y - 1);
                    if !self.setup_field[x as usize][y as usize] {
                        self.setup_field[x as usize][y as usize] = true;
                        display.write_in_field(block.x as usize, block.y as usize, "x");
                    } else {
                        self.setup_field[x as usize][y as usize] = false;
                        display.write_in_field(block.x as usize, block.y as usize, " ");
                    }
                }
            }
        }
        self.clear_x_es(display);
        if !self.get_valid_ship(length, display) {
            for ship in self.ships.iter() {
                display.print_ship(
                    ship.size as usize,
                    ship.x_start_location as usize + 1,
                    ship.y_start_location as usize + 1 as usize,
                    ship.vertical,
                );
            }
            self.setup_field = [[false; 10]; 10];
            self.setup_ship(length, display);
        } else {
            self.setup_field = [[false; 10]; 10];
        }
    }

    /**
     * check whether the given user input is a valid ship
     */
    fn get_valid_ship(&mut self, len: u8, display: &mut Display) -> bool {

        if !self.ship_length_correct(len) {
            return false;
        }

        //check if ship is in a line
        let mut x_start = 0;
        let mut y_start = 0;
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
                            return false;
                        }
                        if !direction_known { //find ship
                            if i == x_pos + 1 {
                                vertical = false;
                            } else {
                                vertical = true;
                            }
                            direction_known = true;
                            x_pos = i;
                            y_pos = j;
                        } else if !vertical {
                            if i != x_pos + 1 || j != y_pos {
                                //Error, next block is at the wrong location
                                return false;
                            }
                            x_pos = i;
                        } else { //vertical
                            if j != y_pos + 1 || i != x_pos {
                                //Error, next block is at the wrong location
                                return false;
                            }
                            y_pos = j;
                        }
                    }
                }
            }
        }

        //check if ship not adjacent to existing ship
        for i in 0..10 {
            for j in 0..10 {
                if self.setup_field[i][j] {
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
                                return false;
                            }
                        }
                    }
                }
            }
        }

        //save ship
        for i in 0..10 {
            for j in 0..10 {
                if self.setup_field[i][j] {
                    self.placed_ships[i][j] = true;
                }
            }
        }

        let current_ship = Ship::new(len, x_start as u8, y_start as u8, vertical);
        self.ships.push(current_ship);
        for ship in self.ships.iter() {
            display.print_ship(
                ship.size as usize,
                ship.x_start_location as usize + 1,
                ship.y_start_location as usize + 1 as usize,
                ship.vertical,
            );
        }
        // Some(current_ship)
        true
    }

    /**
     * check if there are `len` x'es given by the user
     */
    fn ship_length_correct(&mut self, len: u8) -> bool {
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
            return false;
        }
        true
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
                return false;
            }
        }
        true
    }

    /**
     * shoot at a location, return if hit.  If sunk returns the sunken ship's length. If not sunk, return 0 instead.
     */
    pub fn shoot_at(&mut self, block: Block) -> (bool, u8) {
        if !self.fields_shot[block.x as usize - 1][block.y as usize - 1]
            && self.placed_ships[block.x as usize - 1][block.y as usize - 1]
        {
            match self.get_ship_at(block.x - 1, block.y - 1) {
                None => {
                    return (false, 0);
                }
                Some(mut ship) => {
                    ship.sunken_fields += 1;
                    if ship.sunken_fields == ship.size {
                        return (true, ship.size);
                    } else {
                        return (true, 0);
                    }
                }
            }
        }
        (false, 0)
    }

    /**
     * return Some(ship) if there is a ship at the given position, None otherwise
     */
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
                    if x == ship.x_start_location + i {
                        return Some(ship);
                    }
                }
            }
        }
        None
    }

    /**
     * call this when we hit an enemy
     */
    pub fn save_hit_enemy(&mut self, x: u8, y: u8) {
        self.enemy_ships_hit[x as usize][y as usize] = true;
    }
    
    /**
     * only call this method once the ship is sunk. Returns the start location, the lenth and whether ship is vertical
     */
    pub fn get_enemy_ship_start_dir_len(&mut self) -> (u8, u8, bool, u8) {
        //stub
        (0,0,false,0)
    }

    /**
     * Get the 5 ships from the user
     */
    pub fn initial_setup(&mut self, display: &mut Display) {
        display.setup_ship(5);
        self.setup_ship(5, display);
        display.setup_ship(4);
        self.setup_ship(4, display);
        display.setup_ship(3);
        self.setup_ship(3, display);
        display.setup_ship(3);
        self.setup_ship(3, display);
        display.setup_ship(2);
        self.setup_ship(2, display);
    }
}

/**
 * Initializes the gameboard with an empty field and no ships
 */
pub fn gameboard_init() -> Board {
    let ships = Vec::new();
    let fields_shot = [[false; 10]; 10];
    let setup_field = [[false; 10]; 10];
    let placed_ships = [[false; 10]; 10];

    Board::new(ships, fields_shot, setup_field, placed_ships)
}
