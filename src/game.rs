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
        Game {
            game_state: Gamestate::GameStart,
            display,
            board: gameboard::gameboard_init(),
            network: net,
            ethernet_c: EthClient::new(is_server),
        }
    }

    pub fn run_game(&mut self) {

        self.display.show_start_screen();
        loop {
            match self.game_state {
                Gamestate::YourTurn => self.select_shoot_location(),
                Gamestate::WaitForEnemy => self.wait_and_check_enemy_shot(),
                Gamestate::Won => self.show_win_screen(),
                Gamestate::Lose => self.show_lose_screen(),
                Gamestate::SetupShips => self.setup_ships(),
                Gamestate::GameStart => {
                    self.wait_for_start_screen_interaction();
                },
            } 
        }
    }

    fn set_game_state(&mut self, state: Gamestate) {
        self.display.update_status_text(self.board.get_own_ships_of_len(), self.board.get_enemy_ships_of_len());
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
        self.set_game_state(Gamestate::GameStart);
        self.display.show_lose_screen();
        self.board = gameboard::gameboard_init();
    }

    fn show_win_screen(&mut self) {
        //Show win screen
        self.set_game_state(Gamestate::GameStart);
        self.display.show_win_screen();
        self.board = gameboard::gameboard_init();
    }

    fn wait_for_shoot(&mut self) -> network::packets::ShootPacket {
        loop {
            if let Some(shoot) = self.ethernet_c.recv_shoot(&mut self.network) {
                return shoot;
            }
        }
    }

    fn wait_and_check_enemy_shot(&mut self) {
        self.display.print_confirm_button_disabled();
        self.display.clear_text_on_display();
        self.display.print_text_on_display_layer2("wait for the enemy".to_string());
        //recvn enemy shot packet and check hit 
        let enemy_shoot = self.wait_for_shoot(); 

        //now check hit
        let (hit, ship_sunk_size) = self.board.shoot_at(Block {x: enemy_shoot.column, y: enemy_shoot.line});
        //create feedback packet
        let win = self.board.check_win();
        let feedback = packets::FeedbackPacket::new(hit, ship_sunk_size, win);
        self.ethernet_c.send_feedback(&mut self.network, feedback);
        self.network.poll_all();
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
        if feedback_packet.you_win {
            self.set_game_state(Gamestate::Won);
            return;
        } else if feedback_packet.hit {
            self.display.write_in_field(block.x as usize, block.y as usize, "X");
            self.board.enemy_ships_hit[(block.x - 1) as usize][(block.y-1) as usize] = true;
            let sunk_size = feedback_packet.sunk;
            if feedback_packet.sunk != 0 {
                self.display.clear_text_on_display();
                self.display.print_text_on_display_layer2(format!("sunk ship of length {}", sunk_size).to_string());
                let (x, y, dir, size) = self.board.get_enemy_ship_start_dir_len(block.x-1, block.y-1);
                assert!(size != 0);
                self.display.print_ship(size as usize, x as usize, y as usize, dir);
            } else {
                self.display.clear_text_on_display();
                self.display.print_text_on_display_layer2("You hit the enemy".to_string().to_string());
            }
        } else {
            self.display.write_in_field(block.x as usize, block.y as usize, "O");
        }

        self.set_game_state(Gamestate::WaitForEnemy);
        //clear all x on layer_1
        // self.board.clear_x_es(&self.display);
    }

    fn wait_for_feedback(&mut self) ->  network::packets::FeedbackPacket {
        loop {
            if let Some(feedback) = self.ethernet_c.recv_feedback(&mut self.network) {
        return feedback;
}
        }
    }
    
    fn setup_ships(&mut self) {
        self.board.initial_setup(&mut self.display);
        //TODO: send ready packet and wait for other players ready packet

        self.display.layer_2_clear();
        self.display.print_background();
        self.display.print_confirm_button_enabled();
        if self.ethernet_c.is_server {
            self.set_game_state(Gamestate::YourTurn);
        } else {
            self.set_game_state(Gamestate::WaitForEnemy);
        }
    }

    fn select_shoot_location(&mut self) {
        self.display.print_confirm_button_enabled();
        self.display.clear_text_on_display();
        self.display.print_text_on_display_layer2("select a fire location".to_string());
        let mut confirmed = false;
        let mut block_set = false;
        let mut block = Block{x: 1, y: 1};
        //create methods in display to handle touch
        while !confirmed {
            let (x,y) = self.display.touch();
            match self.board.calculate_touch_block(x, y) {
                None => {
                    if block_set && self.display.check_confirm_button_touched(x,y) {
                      //shot location set   
                      self.board.enemy_fields_shot[(block.x - 1) as usize][(block.y -1 ) as usize] = true;
                      self.fire(block); //TODO: in fire -> update gameboard information
                      confirmed = true;
                    }
                }
                Some(ret_block) => {
                    // cortex_m_semihosting::hprintln!("touched block : x:{}, y:{} ", ret_block.x, ret_block.y );
                    //delete old block and set new
                    if !self.board.enemy_fields_shot[(block.x-1) as usize][(block.y-1) as usize] {
                        self.display.write_in_field(block.x as usize, block.y as usize, " ");
                    }
                    if !self.board.enemy_fields_shot[(ret_block.x-1) as usize][(ret_block.y-1) as usize] {
                        self.display.write_in_field(ret_block.x as usize, ret_block.y as usize, "x");
                    }
                    block = ret_block;
                    block_set = true;
                    // self.board.clear_x_es(&mut self.display); 
                }
            }
        }
    }
}

