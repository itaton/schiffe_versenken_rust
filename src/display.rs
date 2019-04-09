use alloc::string::String;
use alloc::vec::Vec;
use core::ptr;
use core::fmt::Write;
use stm32f7_discovery::{
    lcd::Color, lcd::FramebufferAl88, lcd::FramebufferArgb8888, lcd::Layer, lcd::Lcd,
    i2c::I2C,
    touch,
    system_clock::{self, Hz},

};
use stm32f7::stm32f7x6::I2C3;
//pub static BACKGROUNDSMALL: &'static [u8] = include_bytes!("../water.bmp");
pub static BACKGROUND: &'static [u8] = include_bytes!("../waterBig.bmp");
pub static STARTSCREEN: &'static [u8] = include_bytes!("../l33tBackground.bmp");

static BLUE: Color = Color {
    red: 0,
    green: 0,
    blue: 255,
    alpha: 255,
};
static GREEN: Color = Color {
    red: 0,
    green: 255,
    blue: 0,
    alpha: 255,
};
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
    alpha: 127,
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
struct Bmp {
    width: usize,
    height: usize,
    color: [Color; 24500],
    // color: Vec<Color>,
}
struct BmpPixel<'a> {
    red: &'a [u8],
    green: &'a [u8],
    blue: &'a [u8],
}

struct BmpPic<'a> {
    width: &'a [u32],
    height: &'a [u32],
    pixels: Vec<BmpPixel<'a>>,
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

pub fn init_display(mut lcd: &mut Lcd, mut touchscreen: I2C<I2C3>) -> Display {
    let mut layer_1 = lcd.layer_1().unwrap();
    let mut layer_2 = lcd.layer_2().unwrap();
    let mut display = Display::new(layer_1, layer_2, touchscreen);
    // display.draw_background(0,
    //                   0,
    //                   (480, 272), BACKGROUND);  
    display.layer1.clear();
    display.layer2.clear();

    //print_background(&mut layer_1);

    // display.print_bmp_at_location(BACKGROUND, 0, 0);
    // display.draw_background_with_bitmap();
    // display.print_background();
    //lcd.set_background_color(black);
    lcd.set_background_color(WATER_BLUE);

    //print_indicies(&mut layer_1);
    display.print_indicies();
    //print_ship(display.layer2, 4, 5, 5, true);
    // print_ship(layer_2, 2, 4, 5, true);
    // printShip(layer_2, 6, 6, 1, false);
    display
}
impl Display {
    
    pub fn print_background(&mut self) {
        
        self.print_bmp_at_location(BACKGROUND, 0, 0);
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
                self.layer1.print_point_color_at(*c, i, BLACK);
            }
        }
        for c in yarr.iter() {
            for i in 0..275 {
                self.layer1.print_point_color_at(i, *c, BLACK);
            }
        }
        self.print_indicies();
    }
    
    /**
     * print a confirm button on the right side of the display
     */
    pub fn print_confirm_button(&mut self) {
        for i in 299..301 {
            for j in 199..250 {
                //todo change this to lookup color since layer 2 is lookup only
                self.layer1.print_point_color_at(i, j, BLACK);
            }
        }
        for i in 455..457 {
            for j in 199..250 {
                self.layer1.print_point_color_at(i, j, BLACK);
            }
        }
        for i in 299..457 {
            for j in 199..201 {
                self.layer1.print_point_color_at(i, j, BLACK);
            }
        }
        for i in 299..457 {
            for j in 249..251 {
                self.layer1.print_point_color_at(i, j, BLACK);
            }
        }
        for i in 299..457 {
            for j in 199..251 {
                self.layer1.print_point_color_at(i, j, BLACK);
            }
        }
        let mut text_writer = self.layer2.text_writer_at(350, 220);
        let result = text_writer.write_str("CONFIRM");
        match result {
            Ok(result) => result,
            Err(error) => panic!("error while writing text on display: {}", error),
        };
    }

    pub fn print_text_on_display(&mut self, text: String) {
        assert!(text.len() < 50); //TODO check max string length for the gui
        let split = text.split_whitespace();
        let mut y = 50;
        for word in split {
            let mut text_writer = self.layer1.text_writer_at(350, y);
            let result = text_writer.write_str(word);
            match result {
                Ok(result) => result,
                Err(error) => panic!("error while writing text on display: {}", error),
            };
            y += 20;
        }
    }

    pub fn print_text_on_display_layer2(&mut self, text: String) {
        assert!(text.len() < 50); //TODO check max string length for the gui
        let split = text.split_whitespace();
        let mut y = 50;
        for word in split {
            let mut text_writer = self.layer2.text_writer_at(350, y);
            let result = text_writer.write_str(word);
            match result {
                Ok(result) => result,
                Err(error) => panic!("error while writing text on display: {}", error),
            };
            y += 20;
        }
    }

    pub fn setup_ship(&mut self, ship_len: u8) {
        let arr = [
            24, 25, 49, 50, 74, 75, 99, 100, 124, 125, 149, 150, 174, 175, 199, 200, 224, 225, 249,
            250, 274, 275,
        ];
        //let arr2 = [24,25,49,50,74,75,99,100,124,125,149,150,174,175,199,200,224,225,249,250];
        let arr = [299, 300, 380, 381];
        let arr2 = [199, 200, 249, 250];
        for i in 299..301 {
            for j in 199..250 {
                //todo change this to lookup color since layer 2 is lookup only
                self.layer1.print_point_color_at(i, j, BLACK);
            }
        }
        for i in 455..457 {
            for j in 199..250 {
                self.layer1.print_point_color_at(i, j, BLACK);
            }
        }
        for i in 299..457 {
            for j in 199..201 {
                self.layer1.print_point_color_at(i, j, BLACK);
            }
        }
        for i in 299..457 {
            for j in 249..251 {
                self.layer1.print_point_color_at(i, j, BLACK);
            }
        }
        let mut text_writer = self.layer1.text_writer_at(300, 100);
        text_writer.write_str("Please set up your");
        let mut text_writer = self.layer1.text_writer_at(300, 120);
        text_writer.write_fmt(format_args!("{} ship", ship_len));
        let mut text_writer = self.layer1.text_writer_at(350, 220);
        text_writer.write_str("Confirm");
    }

    //fn print_indicies(mut text_writer: &mut TextWriter<FramebufferArgb8888>) {
    //fn print_indicies(mut layer: &mut Layer<FramebufferArgb8888>) {
    fn print_indicies(&mut self) {
        let text_writer = self.layer1.text_writer();
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
        let x_pos = 9 + 25 * x;
        let y_pos = 9 + 25 * y;
        if x == 0 {
            let x_pos = 9;
        };
        if y == 0 {
            let y_pos = 9;
        };
        //text_writer.x_pos = x_pos;
        //text_writer.y_pos = y_pos;
        let mut text_writer = self.layer1.text_writer_at(x_pos, y_pos);
        text_writer.write_str(letter);
    }

    pub fn write_in_field_layer2(&mut self, x: usize, y: usize, letter: &str) {
        let x_pos = 9 + 25 * x;
        let y_pos = 9 + 25 * y;
        if x == 0 {
            let x_pos = 9;
        };
        if y == 0 {
            let y_pos = 9;
        };
        //text_writer.x_pos = x_pos;
        //text_writer.y_pos = y_pos;
        let mut text_writer = self.layer2.text_writer_at(x_pos, y_pos);
        text_writer.write_str(letter);
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
        let x_start_pixel = ship_start_xblock * block_size + 3;
        let y_start_pixel = ship_start_yblock * block_size + 2;
        if vertical {
            let x_end_pixel = x_start_pixel + block_size - 5;
            let mut y_end_pixel = y_start_pixel + (block_size * ship_size) - 5;
            if y_end_pixel >= 272 {y_end_pixel = 271;}
            //vertical
            for c in x_start_pixel..x_end_pixel {
                for i in y_start_pixel..y_end_pixel {
                    self.layer2.print_point_color_at(c, i, WHITE);
                }
            }
        } else {
            let x_end_pixel = x_start_pixel + (block_size * ship_size) - 5;
            let mut y_end_pixel = y_start_pixel + block_size - 5;
            if y_end_pixel >= 272 {y_end_pixel = 271;}
            //horizontal
            for c in x_start_pixel..x_end_pixel {
                for i in y_start_pixel..y_end_pixel {
                    self.layer2.print_point_color_at(c, i, WHITE);
                }
            }
        }
    }

    //TODO: without self.touch -> pass x, y paramters ?
    pub fn check_confirm_button_touched(&mut self, x: u16, y: u16) -> bool {
        // let (x,y) = self.touch();

        (x,y).0 < 457 && (x,y).0 >= 299 && (x,y).1 < 251 && (x,y).1 >= 199
    }


    pub fn touch(&mut self) -> (u16, u16) {
        let mut touch_x = 0;
        let mut touch_y = 0;

        let curr_ticks = system_clock::ticks();
        
        if curr_ticks - self.last_touch >= 8 {
            for touch in &touch::touches(&mut self.touchscreen).unwrap() {
                //cortex_m::asm::bkpt();
                //let (x,y) = calculate_touch_block(touch.x, touch.y);
                touch_x = touch.x;
                touch_y = touch.y;
                self.last_touch = curr_ticks;
            }
            (touch_x, touch_y)
        }
        else {
            (0,0)
        }
        // let ticks = system_clock::ticks();
        // while system_clock::ticks()-ticks <= 3 {

        // }

        //calculate_touch_block(touch_x, touch_y)
    }

    pub fn render_bg(&mut self, x: u16, y: u16, color: u16) {
        let addr: u32 = 0xC000_0000;
        let pixel = (y as u32) * 480 + (x as u32);
        let pixel_color = (addr + pixel * 2) as *mut u16;
        unsafe { ptr::write_volatile(pixel_color, color) };
    }

    pub fn draw_background(&mut self, x: u16, y: u16, size: (u16, u16), dump: &[u8]) {
        let img_cnt = size.0 as usize * size.1 as usize;
        for i in 0..img_cnt {
            let idx = i * 4;
            let dsp_y = y + (i / size.0 as usize) as u16;
            let dsp_x = x + (i % size.0 as usize) as u16;
            let c = self.from_rgb_with_alpha(dump[idx + 3],
                                                  dump[idx],
                                                  dump[idx + 1],
                                                  dump[idx + 2]);
            self.render_bg(dsp_x, dsp_y, c)
        }
    } 
    
    fn from_rgb_with_alpha(&mut self, a: u8, r: u8, g: u8, b: u8) -> u16 {
        let r_f = (r / 8) as u16;
        let g_f = (g / 8) as u16;
        let b_f = (b / 8) as u16;
        let c: u16 = if a >= 42 { 1 << 15 } else { 0 };
        c | (r_f << 10) | (g_f << 5) | b_f
    } 

    //TODO delete this and use the one in gameboard. Then get x and y from the Block returned
    pub fn calculate_touch_block(&mut self, x: u16, y: u16) -> (u16,u16) {
        if x<=272 && x>24 && y <= 272 && y > 24 {
            let x_block = x/25;
            let y_block = y/25;
            (x_block,y_block)
        } else {
            (0,0)
        }
    }

    pub fn show_start_screen(&mut self) {
        self.print_bmp_at_location(STARTSCREEN, 0, 0);
    }

    pub fn draw_button_at(&mut self) {

    }

    pub fn show_win_screen(&mut self) {

    }

    // pub fn draw_background_with_bitmap(&mut self) {
        // let bmp = self.read_bmp(BACKGROUND);
        // self.draw_test(bmp);
        // self.print_bmp_at_location(BACKGROUND, 0, 0);
        // for l in 0..5 {
        //     for k in 0..5 {
        //         for i in 0..bmp.height {
        //             for j in 0..bmp.width { 
        //                 self.layer1.print_point_color_at(j+(k*bmp.width), i+(l*bmp.height), bmp.color[(bmp.height - i - 1) * bmp.width + j]);
        //             }
        //         }
        //     }
        // }
    // }

    // fn read_bmp(&mut self, map_format : &[u8]) -> Bmp {
    //     let colormap_offset = u32::from(map_format[10]);
    //     let width = u32::from(map_format[18]) + (u32::from(map_format[19]) * 256_u32);
    //     let height = u32::from(map_format[22]) + (u32::from(map_format[23]) * 256_u32);

    //     let mut image_colors = [grey; 24500];
    //     let mut current_index = colormap_offset;
    //     // let mut image_colors = Vec::new();
    //     for i in 0..(w * h - 1) { //get colors from colormap
    //         current_index += 3;
    //         image_colors[i] = Color{blue: map_format[current_index], green: map_format[current_index + 1], red: map_format[current_index + 2],alpha: 255};
    //         // image_colors.push(Color {blue: map_format[current_index], green: map_format[current_index + 1], red: map_format[current_index + 2],alpha: 255});
    //     }
    //     Bmp{width: w, height: h , color: image_colors,}
    // }   

    pub fn print_bmp_at_location(&mut self, pic: &[u8], x: u32, y: u32) {
        let pixels_start = u32::from(pic[10]);
        let width = u32::from(pic[18]) + (u32::from(pic[19]) * 256_u32);
        let height = u32::from(pic[22]) + (u32::from(pic[23]) * 256_u32);
        let pixel_rest = width % 4;

        let loc_x = x;
        let loc_y = y;
        let mut bytenr: u32 = pixels_start;
        let pixel_end: u32 = pic.len() as u32 - 1;
        // println!("{},{}",pixel_rest,pixel_end );

        for i in 0..height {
            bytenr = pixel_end + 1 - (pixel_rest + width * 3) * (i + 1);
            for j in 0..width {
                if !(pic[(bytenr + 2) as usize] > 245 && pic[(bytenr + 1) as usize] > 245
                    && pic[(bytenr) as usize] > 245) {
                    self.layer1.print_point_color_at(
                        (loc_x + j) as usize,
                        (loc_y + i) as usize,
                        Color::rgba(
                            pic[(bytenr + 2) as usize],
                            pic[(bytenr + 1) as usize],
                            pic[(bytenr) as usize],
                            pic[(bytenr) as usize]-50,
                        ),
                    );
                }
                bytenr += 3;
            }
        }
    }
}