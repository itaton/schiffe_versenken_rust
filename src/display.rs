//mod Ships;
use stm32f7_discovery::{
    lcd::FramebufferAl88,
    lcd::FramebufferArgb8888,
    lcd::Lcd,
    lcd::Color,
    lcd::Layer,
    lcd::TextWriter,
};
use core::fmt::Write;

static blue:Color = Color{red: 0,green: 0 ,blue: 255,alpha: 255};
static green:Color = Color{red: 0,green: 255 ,blue: 0,alpha: 255};
static black:Color = Color{red: 0,green: 0 ,blue: 0,alpha: 255};
static grey:Color = Color{red: 127,green: 127 ,blue: 127,alpha: 127};
static white:Color = Color{red: 0, green: 0, blue: 0, alpha: 255};
 
pub fn init_display(mut lcd: &mut Lcd) {
    let mut layer_1 = lcd.layer_1().unwrap();
    let mut layer_2 = lcd.layer_2().unwrap();
    layer_1.clear();
    layer_2.clear();
    print_background(&mut layer_1);
    let mut text_writer = layer_1.text_writer();
    

    //let bg_color = Color{red: 0,green: 0 ,blue: 255,alpha: 190};
    //set_background_color(bg_color, lcd);
    //set_background_color(blue, *lcd);
    lcd.set_background_color(blue);
    print_indicies(&mut text_writer);
    print_ship(layer_2, 4, 2, 2, false);
    // print_ship(layer_2, 2, 4, 5, true);
    // printShip(layer_2, 6, 6, 1, false);
}  

fn set_background_color(color: Color,mut lcd: Lcd) {
    lcd.set_background_color(color);
}

fn print_background(mut layer_1: &mut Layer<FramebufferArgb8888>) {
    let arr = [24,25,49,50,74,75,99,100,124,125,149,150,174,175,199,200,224,225,249,250,274,275];
    let arr2 = [24,25,49,50,74,75,99,100,124,125,149,150,174,175,199,200,224,225,249,250];
    for c in arr.iter() {
            for i in 0..272 {
                layer_1.print_point_color_at(*c, i, black);
            }
        }
        for c in arr2.iter() {
            for i in 0..275 {
                layer_1.print_point_color_at(i, *c, black);
            }
        }

}

pub fn setup_ship_5(mut layer_1: &mut Layer<FramebufferArgb8888>, mut text_writer : &mut TextWriter<FramebufferArgb8888>) {
    let arr = [24,25,49,50,74,75,99,100,124,125,149,150,174,175,199,200,224,225,249,250,274,275];
    //let arr2 = [24,25,49,50,74,75,99,100,124,125,149,150,174,175,199,200,224,225,249,250];
    let arr = [299,300,380,381];
    let arr2 = [199,200,249,250];
    for i in 299..300 {
        for j in 199..250 {
            layer_1.print_point_color_at(i, j, black);
        }
    }
    for i in 380..381 {
        for j in 199..250 {
            layer_1.print_point_color_at(i, j, black);
        }
    }
    for i in 299..381 {
        for j in 199..200 {
            layer_1.print_point_color_at(i, j, black);
        }
    }
    for i in 299..381 {
        for j in 249..250 {
            layer_1.print_point_color_at(i, j, black);
        }
    }
    text_writer.x_pos = 300;
    text_writer.y_pos = 100;
    text_writer.write_str("Please set up your 5 ship");
} 

fn print_indicies(mut text_writer: &mut TextWriter<FramebufferArgb8888>) {
    write_in_field(1, 0, text_writer, "1");
    write_in_field(2, 0, text_writer, "2");
    write_in_field(3, 0, text_writer, "3");
    write_in_field(4, 0, text_writer, "4");
    write_in_field(5, 0, text_writer, "5");
    write_in_field(6, 0, text_writer, "6");
    write_in_field(7, 0, text_writer, "7");
    write_in_field(8, 0, text_writer, "8");
    write_in_field(9, 0, text_writer, "9");
    write_in_field(10, 0, text_writer, "0");
    //we need a special case here since '10' is to characters
    //text_writer.x_pos=255;
    //text_writer.y_pos=9;
    //text_writer.write_str("10")
    write_in_field(0, 1, text_writer, "a");
    write_in_field(0, 2, text_writer, "b");
    write_in_field(0, 3, text_writer, "c");
    write_in_field(0, 4, text_writer, "d");
    write_in_field(0, 5, text_writer, "e");
    write_in_field(0, 6, text_writer, "f");
    write_in_field(0, 7, text_writer, "g");
    write_in_field(0, 8, text_writer, "h");
    write_in_field(0, 9, text_writer, "i");
    write_in_field(0, 10, text_writer, "j");

}

pub fn write_in_field(x: usize, y: usize, mut text_writer: &mut TextWriter<FramebufferArgb8888>, letter: &str) {
    let x_pos = 9 + 25 * x;
    let y_pos = 9 + 25 * y;
    if x == 0 {let x_pos = 9;};
    if y == 0 {let y_pos = 9;};
    text_writer.x_pos = x_pos;
    text_writer.y_pos = y_pos;
    text_writer.write_str(letter);
}

/**
 * draw ship on x, y coordination. The direction is vertical for true and horizontal for false.
 */
pub fn print_ship(mut layer_2: Layer<FramebufferAl88>, ship_size: usize, x_ship_start_location: usize, y_ship_start_location: usize, vertical: bool) {
    let block_size = 25;
    let x_start_pixel = x_ship_start_location*block_size;
    let y_start_pixel = y_ship_start_location*block_size;
    if vertical {
        let x_end_pixel = x_start_pixel + block_size;
        let y_end_pixel = y_start_pixel + (block_size*ship_size);
        //vertical
        for c in x_start_pixel..x_end_pixel {
            for i in y_start_pixel..y_end_pixel {
                layer_2.print_point_color_at(c, i, white);
            }
        }
    } else {
        let x_end_pixel = x_start_pixel + (block_size*ship_size);
        let y_end_pixel = y_start_pixel + block_size;
        //horizontal
        for c in x_start_pixel..x_end_pixel {
            for i in y_start_pixel..y_end_pixel {
                layer_2.print_point_color_at(c, i, white);
            }
        }
    }
}

