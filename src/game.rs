use crate::gameboard::{
    Block,
};

use crate::network::{
    packets,
    Network,
    EthClient,
};
use crate::display::{
    Display
};

struct Game<'a> {
    game_state: Gamestate,
    board: gameboard, //TODO: 
    display: Display,
    network: Network<'a>,
    ethernet_c: EthClient,
}

enum Gamestate {
    YourTurn,
    WaitForEnemy,
    Won,
    GameStart,
}

//start game, init field and wait for other player
pub fn init_new_game(display: Display) -> Game {
    Game::new(display, is_server)    
}




//game loop
impl Game {
    fn new(display: Display) -> Game {
        Game {
            game_state: Gamestate::GameStart,
            board: gameboard::init,
            display,
            network: network.new(),
            ethernet_c: EthClient::new()
        }
    }

    pub fn run_game(&self) {
        loop {
            match self.game_state {
                Gamestate::YourTurn => self.select_shoot_location(),
                Gamestate::WaitForEnemy => self.wait_and_check_enemy_shot(),
                Gamestate::Won => self.show_win_screen(),
                Gamestate::GameStart => self.setup_ships()
            } 
        }
    }

    fn show_win_screen(&self) {

    }

    //TODO: check if gameboard and network is implemented
    fn wait_and_check_enemy_shot(&self) {
        //recvn enemy shot packet and check hit 
        let enemy_shot = self.ethernet_c.recv_shoot(self.ethernet_c, self.network);
        let (hit, sunk) = self.board.shot_at(Block {x: enemy_shot.column, y: enemy_shot.line});
        let mut ship_sunk_size = 0;
        if sunk {
            //get ship size 
            // ship_sunk = 
        }
        //create feedback packet
        let win = self.board.check_win();
        let feedback = packets::FeedbackPacket::new(hit, ship_sunk_size, win);
        self.ethernet_c.send_feedback(self.network, feedback);
        self.game_state = Gamestate::YourTurn;
    }

    //send shoot packet and check hit
    fn fire(&self, block: Block) {
        let shoot_packet = packets::ShootPacket::new(block.y, block.x); //TODO set x,y public
        //use network file and send package
        self.ethernet_c.send_shoot(self.network, shoot_packet); 
        //wait for answer
        let feedback_packet = self.ethernet_c.recv_feedback(self.network);
        if feedback_packet.you_win = true {
            self.game_state = Gamestate::Won;
        } else if feedback_packet.hit {
            let sunk_size = feedback_packet.sunk;
            //TODO: set red cross on display 
        } else {
            //TODO: set white cross and set corresponding field as shot in gameboard
        }
    }

    //check if coordinates hit one of the your ship
    fn check_hit() {

    }

    fn check_win() {

    }
    
    fn setup_ships(&self) {
        self.select_ship_locations(5);
        self.select_ship_locations(4);
        self.select_ship_locations(3);
        self.select_ship_locations(3);
        self.select_ship_locations(2);

        //TODO: send ready packet and wait for other players ready packet

        if self.ethernet_c.is_server {
            self.game_state = Gamestate::YourTurn;
        } else {
            self.game_state = Gamestate::WaitForEnemy;
        }
    }

    fn select_ship_locations(&self, ship_size: u8) {
        //for each ship, select location and confirm with button
        let ship_one_selections = self.display.get_touch_locations(ship_size); //TODO implement get touch location in display
        for selection in ship_one_selections {
            gameboard::calculate_touch_block(x: u16, y: u16);
            gameboard::setup_ship(ship_size);
        }
    }


    //TODO: change -> new gameboard and display 
    fn select_shoot_location(&self) {
        let confirmed = false;
        let block;
        //create methods in display to handle touch
        while !confirmed {
            for touch in &touch::touches(&mut i2c_3).unwrap() {
                block = gameboard::calculate_touch_block(touch.x, touch.y);
                if block.x == 0 && block.y == 0 {
                    if touch_confirm() {
                        confirmed = true;
                        fire(block);
                    }
                } else {
                    //TODO: delete last block marker first

                    //set new block 
                    on display
                    //TODO: write method in display to avoid the layer parameter !
                    display::write_in_field(block.x as usize, block.y as usize, &mut layer_1, "x");
                }
            }
        }
        //select a block and confirm your choise
        for touch in &touch::touches(&mut i2c_3).unwrap() {
            let (x,y) = calculate_touch_block(touch.x, touch.y);
            if (x,y) != (0,0) {
                display::write_in_field((x,y).0 as usize, (x,y).1 as usize, &mut layer_1, "x");
            }
        }
        for touch in &touch::touches(&mut i2c_3).unwrap() {
            if touch_confirm(touch.x, touch.y) {

            }
            //remove last choise and set new.

        }  
    }
}

