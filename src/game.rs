
use network::packets;
struct game_information {
    game_field:[[u8];u8],
    game_state:Gamestate,
    
}

enum Gamestate {
    your_turn,
    wait_for_enemy,
    won,
}



pub init_new_game() {
    
}

fn fire(x: u8, y: u8) {
   let shoot_packet = ShootPacket::new(y, x);
}

fn check_enemy_shot() {

}

fn check_hit() {

}

fn check_win() {

}

pub fn start_game() {

    loop {
       match game_information.state {
           Gamestate::your_turn => 
           Gamestate::wait_for_enemy =>
           Gamestate::won =>
       } 
    }
}

