use crate::gameboard::{
    Block,
};
use crate::network::{
    packets,
};

struct Game {
    game_state: Gamestate,
    board: gameboard, //TODO: 
}

enum Gamestate {
    YourTurn,
    WaitForEnemy,
    Won,
    GameStart,
}

//start game, init field and wait for other player
pub fn init_new_game() -> Game {
    Game::new()    
}





//game loop
impl Game {
    fn new() -> Game {
        Game {
            game_state: Gamestate::GameStart
        }
    }

    pub fn run_game() {
        loop {
            match self.state {
                Gamestate::your_turn => select_shoot_location(),
                Gamestate::wait_for_enemy => wait_and_check_enemy_shot(),
                Gamestate::won => show_win_screen(),
            } 
        }
    }

    //send shoot packet and check hit
    fn fire(block: Block) {
        let shoot_packet = packets::ShootPacket::new(block.y, block.x);
        //use network file and send package

        //wait for answer
    }

    //receive shoot packet and check hit
    fn check_enemy_shot() {
        gameboard::shot_at(block: Block)
    }

    //check if coordinates hit one of the your ship
    fn check_hit() {

    }

    fn check_win() {

    }

    fn select_shoot_location() {
        let confirmed = false;
        let block;
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

