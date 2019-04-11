use alloc::string::String;
use crate::alloc::string::ToString;
use core::fmt::Write;
use stm32f7_discovery::{
    lcd::Color, lcd::FramebufferAl88, lcd::FramebufferArgb8888, lcd::Layer, lcd::Lcd,
    i2c::I2C,
    touch,
    system_clock::{self},
};
use stm32f7::stm32f7x6::I2C3;
static BACKGROUND: &'static [u8] = include_bytes!("../WaterBig3Small.bmp");
static STARTSCREEN: &'static [u8] = include_bytes!("../StartScreen.bmp");
static WIN_FONT: &'static [u8] = include_bytes!("../win_font_small_without_alpha.bmp");
static LOSE_FONT: &'static [u8] = include_bytes!("../loser_font_small.bmp");



static BLACK: Color = Color {
    red: 0,
    green: 0,
    blue: 0,
    alpha: 255,
};
static GREY: Color = Color {
    red: 127,
    green: 127,
    blue: 127,
    alpha: 100,
};
static WHITE: Color = Color {
    red: 255,
    green: 255,
    blue: 255,
    alpha: 255,
};
static WATER_BLUE: Color = Color {
    red: 49,
    green: 190,
    blue: 190,
    alpha: 255,
};

pub struct Display {
    layer1: Layer<FramebufferArgb8888>,
    layer2: Layer<FramebufferAl88>,
    touchscreen: I2C<I2C3>,
    last_touch: usize,
}

impl Display {
    pub fn new(layer1: Layer<FramebufferArgb8888>, layer2: Layer<FramebufferAl88>, touchscreen: I2C<I2C3>) -> Display {
        Display {
            layer1,
            layer2,
            touchscreen,
            last_touch: system_clock::ticks(),
        }
    }
}

pub fn init_display(lcd: &mut Lcd, touchscreen: I2C<I2C3>) -> Display {
    let layer_1 = lcd.layer_1().unwrap();
    let layer_2 = lcd.layer_2().unwrap();
    let mut display = Display::new(layer_1, layer_2, touchscreen);
    display.layer1.clear();
    display.layer2.clear();
    lcd.set_background_color(WATER_BLUE);
    display
}
impl Display {
    
    pub fn print_background(&mut self) {
        
        self.print_bmp_at_location(BACKGROUND, 0, 0);
        self.print_bmp_at_location(BACKGROUND, 240, 0);
        self.print_bmp_at_location(BACKGROUND, 0, 136);
        self.print_bmp_at_location(BACKGROUND, 240, 136);
        let xarr = [
            24, 25, 49, 50, 74, 75, 99, 100, 124, 125, 149, 150, 174, 175, 199, 200, 224, 225, 249,
            250, 274, 275,
        ];
        let yarr = [
            24, 25, 49, 50, 74, 75, 99, 100, 124, 125, 149, 150, 174, 175, 199, 200, 224, 225, 249,
            250,
        ];
        for c in xarr.iter() {
            for i in 0..272 {
                self.layer2.print_point_color_at(*c, i, BLACK);
            }
        }
        for c in yarr.iter() {
            for i in 0..275 {
                self.layer2.print_point_color_at(i, *c, BLACK);
            }
        }
        self.print_indicies();
        self.print_status_information();
    }

    pub fn update_status_text(&mut self, own_ships: (u8, u8, u8, u8), enemy_ships: (u8, u8, u8, u8)) {
        let x_you = 350;
        let x_enemy = 451;
        let y = 55;
        self.write_text_on_location(x_you, y, format!("{}", own_ships.0).to_string());
        self.write_text_on_location(x_you, y+15, format!("{}", own_ships.1).to_string());
        self.write_text_on_location(x_you, y+30, format!("{}", own_ships.2).to_string());
        self.write_text_on_location(x_you, y+45, format!("{}", own_ships.3).to_string());
        
        self.write_text_on_location(x_enemy, y, format!("{}", enemy_ships.0).to_string());
        self.write_text_on_location(x_enemy, y+15, format!("{}", enemy_ships.1).to_string());
        self.write_text_on_location(x_enemy, y+30, format!("{}", enemy_ships.2).to_string());
        self.write_text_on_location(x_enemy, y+45, format!("{}", enemy_ships.3).to_string());
    }

    fn write_text_on_location(&mut self, x: usize, y: usize, text: String) {
        let mut text_writer = self.layer2.text_writer_at(x, y);
        let result = text_writer.write_str(&text);
        match result {
            Ok(result) => result,
            Err(error) => panic!("error while writing text on display: {}", error),
        };
    }
    
    fn print_status_information(&mut self) {
        for i in 0..136 {
            self.layer2.print_point_color_at(378, i, BLACK);
        }
        for i in 275..480 {
            self.layer2.print_point_color_at(i, 136, BLACK);
        }
        let y = 55;
        let x_you = 290;
        self.write_text_on_location(x_you+20, 10, "Your".to_string());
        self.write_text_on_location(x_you+15, 20, "Ships".to_string());
        self.write_text_on_location(x_you, y, "size 2:".to_string());
        self.write_text_on_location(x_you, y+15, "size 3:".to_string());
        self.write_text_on_location(x_you, y+30, "size 4:".to_string());
        self.write_text_on_location(x_you, y+45, "size 5:".to_string());

        let x_enemy = x_you + 101;
        self.write_text_on_location(x_enemy+20, 10, "Enemy".to_string());
        self.write_text_on_location(x_enemy+20, 20, "Ships".to_string());
        self.write_text_on_location(x_enemy, y, "size 2:".to_string());
        self.write_text_on_location(x_enemy, y+15, "size 3:".to_string());
        self.write_text_on_location(x_enemy, y+30, "size 4:".to_string());
        self.write_text_on_location(x_enemy, y+45, "size 5:".to_string());
    }

    /**
     * print a confirm button on the right side of the display
     */
    fn print_confirm_button(&mut self, color: Color) {
        for i in 299..301 {
            for j in 199..250 {
                //todo change this to lookup color since layer 2 is lookup only
                self.layer1.print_point_color_at(i, j, color);
            }
        }
        for i in 455..457 {
            for j in 199..250 {
                self.layer1.print_point_color_at(i, j, color);
            }
        }
        for i in 299..457 {
            for j in 199..201 {
                self.layer1.print_point_color_at(i, j, color);
            }
        }
        for i in 299..457 {
            for j in 249..251 {
                self.layer1.print_point_color_at(i, j, color);
            }
        }
        for i in 299..457 {
            for j in 199..251 {
                self.layer1.print_point_color_at(i, j, color);
            }
        }
        let mut text_writer = self.layer2.text_writer_at(350, 220);
        let result = text_writer.write_str("CONFIRM");
        match result {
            Ok(result) => result,
            Err(error) => panic!("error while writing text on display: {}", error),
        };
    }

    

    pub fn print_text_on_display_layer2(&mut self, text: String) {

        assert!(text.len() < 50); //TODO check max string length for the gui
        // let split = text.split_whitespace();
        let y = 160;
        // for word in split {
            let mut text_writer = self.layer2.text_writer_at(300, y);
            let result = text_writer.write_str(&text.to_string());
            match result {
                Ok(result) => result,
                Err(error) => panic!("error while writing text on display: {}", error),
            };
            // y += 20;
        // }
    }

    //TODO refactor method -> not neccesary 
    pub fn setup_ship(&mut self, ship_len: u8) {
        self.print_text_on_display_layer2(format_args!("Set up your {} ship", ship_len).to_string());
        self.print_confirm_button(BLACK);
    }

    //fn print_indicies(mut text_writer: &mut TextWriter<FramebufferArgb8888>) {
    //fn print_indicies(mut layer: &mut Layer<FramebufferArgb8888>) {
    fn print_indicies(&mut self) {
        self.write_in_field(1, 0, "1");
        self.write_in_field(2, 0, "2");
        self.write_in_field(3, 0, "3");
        self.write_in_field(4, 0, "4");
        self.write_in_field(5, 0, "5");
        self.write_in_field(6, 0, "6");
        self.write_in_field(7, 0, "7");
        self.write_in_field(8, 0, "8");
        self.write_in_field(9, 0, "9");
        self.write_in_field(10, 0, "0");
        self.write_in_field(0, 1, "a");
        self.write_in_field(0, 2, "b");
        self.write_in_field(0, 3, "c");
        self.write_in_field(0, 4, "d");
        self.write_in_field(0, 5, "e");
        self.write_in_field(0, 6, "f");
        self.write_in_field(0, 7, "g");
        self.write_in_field(0, 8, "h");
        self.write_in_field(0, 9, "i");
        self.write_in_field(0, 10, "j");
    }

    //pub fn write_in_field(x: usize, y: usize, mut text_writer: &mut TextWriter<FramebufferArgb8888>, letter: &str) {
    pub fn write_in_field(&mut self, x: usize, y: usize, letter: &str) {
        let mut x_pos = 9 + 25 * x;
        let mut y_pos = 9 + 25 * y;
        if x == 0 {
            x_pos = 9;
        }
        if y == 0 {
            y_pos = 9;
        }
        let mut text_writer = self.layer2.text_writer_at(x_pos, y_pos);
        if let Ok(value) = text_writer.write_str(letter) {
            value
        }
    }

    pub fn layer_2_clear(&mut self) {
        self.layer2.clear();
    }

    pub fn clear_text_on_display(&mut self) {
        let mut y = 160;
        for _ in 0..3 {
            let mut text_writer = self.layer2.text_writer_at(350, y);
            let result = text_writer.write_str("                ");
            match result {
                Ok(result) => result,
                Err(error) => panic!("error while writing text on display: {}", error),
            };
            y += 20;
        }    
    }

    /**
     * draw ship on x, y coordination. The direction is vertical for true and horizontal for false.
     */
    pub fn print_ship(
        &mut self,
        ship_size: usize,
        ship_start_xblock: usize,
        ship_start_yblock: usize,
        vertical: bool,
    ) {
        let block_size = 25;
        let x_start_pixel = (ship_start_xblock+1) * block_size + 2;
        let y_start_pixel = (ship_start_yblock+1) * block_size + 2;
        if vertical {
            let x_end_pixel = x_start_pixel + block_size - 4;
            let mut y_end_pixel = y_start_pixel + (block_size * ship_size) - 4;
            if y_end_pixel >= 272 {y_end_pixel = 271;}
            //vertical
            for c in x_start_pixel..x_end_pixel {
                for i in y_start_pixel..y_end_pixel {
                    self.layer2.print_point_color_at(c, i, WHITE);
                }
            }
        } else {
            let x_end_pixel = x_start_pixel + (block_size * ship_size) - 4;
            let mut y_end_pixel = y_start_pixel + block_size - 4;
            if y_end_pixel >= 272 {y_end_pixel = 271;}
            //horizontal
            for c in x_start_pixel..x_end_pixel {
                for i in y_start_pixel..y_end_pixel {
                    self.layer2.print_point_color_at(c, i, WHITE);
                }
            }
        }
    }

    pub fn print_confirm_button_enabled(&mut self) {
        self.print_confirm_button(BLACK);
    }

    pub fn print_confirm_button_disabled(&mut self) {
        self.print_confirm_button(GREY);
    }

    pub fn check_confirm_button_touched(&mut self, x: u16, y: u16) -> bool {
        if (x,y).0 < 457 && (x,y).0 >= 299 && (x,y).1 < 251 && (x,y).1 >= 199 {
            self.print_confirm_button(WHITE);
            self.print_confirm_button(BLACK);
            true
        } else {
            false
        }
    }


    pub fn touch(&mut self) -> (u16, u16) {
        let mut touch_x = 0;
        let mut touch_y = 0;
        let curr_ticks = system_clock::ticks();
        if curr_ticks - self.last_touch >= 8 {
            for touch in &touch::touches(&mut self.touchscreen).unwrap() {
                touch_x = touch.x;
                touch_y = touch.y;
                self.last_touch = curr_ticks;
            }
            (touch_x, touch_y)
        }
        else {
            (0,0)
        }
    }

    pub fn show_start_screen(&mut self) {
        self.print_bmp_at_location(STARTSCREEN, 0, 0);
    }

    pub fn show_lose_screen(&mut self) {
        self.layer1.clear();
        self.layer2.clear();
        self.show_start_screen();
        self.print_bmp_at_location_black_white(LOSE_FONT, 0, 45); 
    }

    pub fn show_win_screen(&mut self) {
        self.layer1.clear();
        self.layer2.clear();
        self.show_start_screen();
        self.print_bmp_at_location_black_white(WIN_FONT, 0, 45); 
    }

    fn print_bmp_at_location_black_white(&mut self, pic: &[u8], x: u32, y: u32) {
        let width = u32::from(pic[18]) + (u32::from(pic[19]) * 256_u32);
        let height = u32::from(pic[22]) + (u32::from(pic[23]) * 256_u32);
        let pixel_rest = width % 4;
        let loc_x = x;
        let loc_y = y;
        let pixel_end: u32 = pic.len() as u32 - 1;

        for i in 0..height {
            let mut bytenr = pixel_end + 1 - (pixel_rest + width * 3) * (i + 1);
            for j in 0..width {
                if pic[(bytenr + 2) as usize] == 0 && pic[(bytenr + 1) as usize] == 0  && pic[(bytenr) as usize] == 0  {
                    self.layer1.print_point_color_at(
                        (loc_x + j) as usize,
                        (loc_y + i) as usize,
                        Color::rgba(
                            0,
                            0,
                            0,
                            255,
                        ),
                    );
                }
                bytenr += 3;
            }
        }
    }

    fn print_bmp_at_location(&mut self, pic: &[u8], x: u32, y: u32) {
        let width = u32::from(pic[18]) + (u32::from(pic[19]) * 256_u32);
        let height = u32::from(pic[22]) + (u32::from(pic[23]) * 256_u32);
        let pixel_rest = width % 4;
        let loc_x = x;
        let loc_y = y;
        let pixel_end: u32 = pic.len() as u32 - 1;

        for i in 0..height {
            let mut bytenr = pixel_end + 1 - (pixel_rest + width * 3) * (i + 1);
            for j in 0..width {
                    self.layer1.print_point_color_at(
                        (loc_x + j) as usize,
                        (loc_y + i) as usize,
                        Color::rgba(
                            pic[(bytenr + 2) as usize],
                            pic[(bytenr + 1) as usize],
                            pic[(bytenr) as usize],
                            255,
                        ),
                    );
                // }
                bytenr += 3;
            }
        }
    }
}