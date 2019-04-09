use crate::alloc::string::ToString;
use crate::gameboard;
use crate::network;
use crate::gameboard::{
    Block,
    Board,
};

use crate::network::{
    Connection,
    packets,
    Network,
    EthClient,
};
use crate::display::{
    Display
};

pub struct Game {
    game_state: Gamestate,
    display: Display,
    board: Board,
    network: Network,
    ethernet_c: EthClient,
}

// #[derive(Eq)]
enum Gamestate {
    YourTurn,
    WaitForEnemy,
    Won,
    Lose,
    GameStart,
    SetupShips,
}

//start game, init field and wait for other player
pub fn init_new_game(display: Display,net: network::Network, is_server: bool) -> Game {
    Game::new(display, net, is_server)    
}




impl Game {
    fn new(display: Display, net: network::Network, is_server: bool) -> Game {
        // let mut nw: network::Network = net;
        Game {
            game_state: Gamestate::GameStart,
            // board: Board::new(), //TODO: without params ? gameboard creates the start state ? 
            display,
            board: gameboard::gameboard_init(),
            network: net,
            ethernet_c: EthClient::new(is_server),
        }
    }

    pub fn run_game(&mut self) {
        loop {
            match self.game_state {
                Gamestate::YourTurn => self.select_shoot_location(),
                Gamestate::WaitForEnemy => self.wait_and_check_enemy_shot(),
                Gamestate::Won => self.show_win_screen(),
                Gamestate::Lose => self.show_lose_screen(),
                Gamestate::SetupShips => self.setup_ships(),
                Gamestate::GameStart => {
                    self.display.show_start_screen();
                    self.wait_for_start_screen_interaction();
                },
            } 
        }
    }

    fn set_game_state(&mut self, state: Gamestate) {
        match state {
            Gamestate::YourTurn => {
                // assert!(self.game_state == Gamestate::WaitForEnemy);
                self.game_state = Gamestate::YourTurn;
            },
            Gamestate::WaitForEnemy => {
                // assert!(self.game_state == Gamestate::YourTurn);
                self.game_state = Gamestate::WaitForEnemy;
            },
            Gamestate::Won => {
                // assert!(self.game_state == Gamestate::YourTurn || self.game_state == Gamestate::WaitForEnemy);
                self.game_state = Gamestate::Won;
            },
            Gamestate::SetupShips => {
                // assert!(self.game_state == Gamestate::GameStart);
                self.game_state = Gamestate::SetupShips;
            }
            Gamestate::GameStart => {
                self.game_state = Gamestate::GameStart;
            }
            Gamestate::Lose => {
                self.game_state = Gamestate::Lose;
            }
        }
    }

    fn wait_for_start_screen_interaction(&mut self) {
        let (x,y) = self.display.touch();
        if (x,y) != (0,0) {
            self.set_game_state(Gamestate::SetupShips);
            self.display.print_background();
        }
    }

    fn show_lose_screen(&mut self) {
        self.display.show_lose_screen();
    }

    fn show_win_screen(&mut self) {
        //Show win screen
        self.display.show_win_screen();
    }

    fn wait_for_shoot(&mut self) -> network::packets::ShootPacket {
        loop {
            match self.ethernet_c.recv_shoot(&mut self.network) {
                Some(shoot) => {
                        return shoot;
                }
                None => {}
            }
        }
    }

    //TODO: check if gameboard and network is implemented
    fn wait_and_check_enemy_shot(&mut self) {
        //recvn enemy shot packet and check hit 
        let enemy_shoot = self.wait_for_shoot(); 

        //now check hit
        let (hit, sunk, ship_sunk_size) = self.board.shoot_at(Block {x: enemy_shoot.column, y: enemy_shoot.line});
        //create feedback packet
        let win = self.board.check_win();
        let feedback = packets::FeedbackPacket::new(hit, ship_sunk_size, win);
        self.ethernet_c.send_feedback(&mut self.network, feedback);
        if win {
            self.set_game_state(Gamestate::Lose);
       } else {
            self.set_game_state(Gamestate::YourTurn);
       }
    }

    //send shoot packet and check hit
    fn fire(&mut self, block: Block) {

        let shoot_packet = packets::ShootPacket::new(block.y, block.x); //TODO set x,y public
        //use network file and send package
        self.ethernet_c.send_shoot(&mut self.network, shoot_packet);

        //wait for answer
        let feedback_packet = self.wait_for_feedback();
        // let feedback_packet = self.ethernet_c.recv_feedback(self.network);
        if feedback_packet.you_win == true {
            self.set_game_state(Gamestate::Won);
            return;
        } else if feedback_packet.hit {
            self.display.write_in_field(block.x as usize, block.y as usize, "o");
            let sunk_size = feedback_packet.sunk;
            if feedback_packet.sunk != 0 {
                self.display.print_text_on_display_layer2(format!("sunk ship of length {}", sunk_size).to_string());
            }
        } else {
            self.display.write_in_field(block.x as usize, block.y as usize, "x");
        }

        self.set_game_state(Gamestate::WaitForEnemy);
        //clear all x on layer_1
        // self.board.clear_x_es(&self.display);
    }

    fn wait_for_feedback(&mut self) ->  network::packets::FeedbackPacket {
        loop {
            match self.ethernet_c.recv_feedback(&mut self.network) {
                Some(feedback) => {
                        return feedback;
                }
                None => {}
            }
        }
    }
    
    fn setup_ships(&mut self) {
        self.board.initial_setup(&mut self.display);
        //TODO: send ready packet and wait for other players ready packet

        if self.ethernet_c.is_server {
            self.set_game_state(Gamestate::YourTurn);
        } else {
            self.set_game_state(Gamestate::WaitForEnemy);
        }
    }

    fn select_shoot_location(&mut self) {
        self.display.print_text_on_display_layer2("select a fire location".to_string());
        let mut confirmed = false;
        let mut block_set = false;
        let mut block = Block{x: 0, y: 0};
        //create methods in display to handle touch
        while !confirmed {
            let (x,y) = self.display.touch();
            match self.board.calculate_touch_block(x, y) {
                None => {
                    if block_set && self.display.check_confirm_button_touched(x,y) {
                      //shot location set   
                      self.fire(block); //TODO: in fire -> update gameboard information
                      confirmed = true;
                    }
                }
                Some(ret_block) => {
                    //delete old block and set new
                    self.board.clear_x_es(&mut self.display); 
                    self.display.write_in_field(ret_block.x as usize, ret_block.y as usize, "x");
                    block = ret_block;
                    block_set = true;
                }
            }
        }
    }
}

