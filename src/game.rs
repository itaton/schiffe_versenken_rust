use crate::gameboard::{
    Block,
    Board,
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
    board: Board,
    display: Display,
    network: Network<'a>,
    ethernet_c: EthClient,
}

enum Gamestate {
    YourTurn,
    WaitForEnemy,
    Won,
    GameStart,
    SetupShips,
}

//start game, init field and wait for other player
pub fn init_new_game(display: Display,is_server: bool) -> Game<'a> {
    Game::new(display, is_server)    
}




//game loop
impl<'a> Game<'a> {
    fn new(display: Display, is_server: bool) -> Game<'a> {
        Game {
            game_state: Gamestate::GameStart,
            board: Board::new(), //TODO: without params ? gameboard creates the start state ? 
            display,
            network: network.new(),
            ethernet_c: EthClient::new(is_server),
        }
    }

    pub fn run_game(&self) {
        loop {
            match self.game_state {
                Gamestate::YourTurn => self.select_shoot_location(),
                Gamestate::WaitForEnemy => self.wait_and_check_enemy_shot(),
                Gamestate::Won => self.show_win_screen(),
                Gamestate::SetupShips => self.setup_ships(),
                Gamestate::GameStart => self.show_start_screen();
            } 
        }
    }

    fn set_game_state(&self, state: Gamestate) {
        match state {
            Gamestate::YourTurn => {
                assert!(self.game_state == Gamestate::WaitForEnemy);
                self.game_state = Gamestate::YourTurn;
            },
            Gamestate::WaitForEnemy => {
                assert!(self.game_state == Gamestate::YourTurn);
                self.game_state = Gamestate::WaitForEnemy;
            },
            Gamestate::Won => {
                assert!(self.game_state == Gamestate::YourTurn || self.game_state == Gamestate::WaitForEnemy);
                self.game_state = Gamestate::Won;
            },
            Gamestate::SetupShips => {
                assert!(self.game_state == Gamestate::GameStart);
                self.game_state = Gamestate::SetupShips;
            }
            Gamestate::GameStart => {
                self.game_state = Gamestate::GameStart;
            }
        }
    }

    fn show_start_screen(&self) {
        self.display.show_start_screen();
        loop {
            (x,y) = self.display.touch();
            if !((x,y) == (0,0)) {
                self.set_game_state(Gamestate::SetupShips);
            }
        }
   }

    fn show_win_screen(&self) {
        //Show win screen
        self.display.show_win_screen();
    }

    //TODO: check if gameboard and network is implemented
    fn wait_and_check_enemy_shot(&self) {
        //recvn enemy shot packet and check hit 
        let enemy_shot = self.ethernet_c.recv_shoot(self.ethernet_c, self.network);
        //now check hit
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
        self.set_game_state(Gamestate::YourTurn);
    }

    //send shoot packet and check hit
    fn fire(&self, block: Block) {
        let shoot_packet = packets::ShootPacket::new(block.y, block.x); //TODO set x,y public
        //use network file and send package
        self.ethernet_c.send_shoot(self.network, shoot_packet); 
        //wait for answer
        let feedback_packet = self.ethernet_c.recv_feedback(self.network);
        if feedback_packet.you_win = true {
            self.set_game_state(Gamestate::Won);
        } else if feedback_packet.hit {
            let sunk_size = feedback_packet.sunk;
            //TODO: set red cross on display 
        } else {
            //TODO: set white cross and set corresponding field as shot in gameboard
        }
    }
    
    fn setup_ships(&self) {
        self.board.initial_setup();
        //TODO: send ready packet and wait for other players ready packet

        if self.ethernet_c.is_server {
            self.set_game_state(Gamestate::YourTurn);
        } else {
            self.set_game_state(Gamestate::WaitForEnemy);
        }
    }

    fn select_shoot_location(&self) {
        let confirmed = false;
        let block_set = false;
        let mut block;
        //create methods in display to handle touch
        while !confirmed {
            let (x,y) = display.touch();
            match self.board.calculate_touch_block(touch.x, touch.y) {
                None => {
                    if (block_set && self.display.check_confirm_button_touched()) {
                      //shot location set   
                      self.fire(block); //TODO: in fire -> update gameboard information
                      confirmed = true;
                    }
                }
                Ok(ret_block) = {
                    //delete old block and set new
                    if (block_set) {
                       self.board.clear_x_es(); 
                    }
                    self.display.write_in_field(ret_block.x, ret_block.y, "x");
                    block = ret_block;
                }
            }
        }
    }
}

