use core::fmt::Write;
use stm32f7_discovery::{
    lcd::Color, lcd::FramebufferAl88, lcd::FramebufferArgb8888, lcd::Layer, lcd::Lcd,
    i2c::I2C,
    touch,
};
use stm32f7::stm32f7x6::I2C3;

static blue: Color = Color {
    red: 0,
    green: 0,
    blue: 255,
    alpha: 255,
};
static green: Color = Color {
    red: 0,
    green: 255,
    blue: 0,
    alpha: 255,
};
static black: Color = Color {
    red: 0,
    green: 0,
    blue: 0,
    alpha: 255,
};
static grey: Color = Color {
    red: 127,
    green: 127,
    blue: 127,
    alpha: 127,
};
static white: Color = Color {
    red: 0,
    green: 0,
    blue: 0,
    alpha: 255,
};

pub struct Display {
    layer1: Layer<FramebufferArgb8888>,
    layer2: Layer<FramebufferAl88>,
    touchscreen: I2C<I2C3>,
}

impl Display {
    pub fn new(layer1: Layer<FramebufferArgb8888>, layer2: Layer<FramebufferAl88>, touchscreen: I2C<I2C3>) -> Display {
        Display {
            layer1,
            layer2,
            touchscreen,
        }
    }
}

pub fn init_display(mut lcd: &mut Lcd, mut touchscreen: I2C<I2C3>) -> Display {
    let mut layer_1 = lcd.layer_1().unwrap();
    let mut layer_2 = lcd.layer_2().unwrap();
    let mut display = Display::new(layer_1, layer_2, touchscreen);

    display.layer1.clear();
    display.layer2.clear();
    //print_background(&mut layer_1);
    display.print_background();
    lcd.set_background_color(blue);
    //print_indicies(&mut layer_1);
    display.print_indicies();
    //print_ship(display.layer2, 4, 5, 5, true);
    // print_ship(layer_2, 2, 4, 5, true);
    // printShip(layer_2, 6, 6, 1, false);
    display
}
//fn set_background_color(color: Color,mut lcd: Lcd) {
//    lcd.set_background_color(color);
//}
impl Display {
    //fn print_background(mut layer_1: &mut Layer<FramebufferArgb8888>) {
    fn print_background(&mut self) {
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
                self.layer1.print_point_color_at(*c, i, black);
            }
        }
        for c in yarr.iter() {
            for i in 0..275 {
                self.layer1.print_point_color_at(i, *c, black);
            }
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
                self.layer1.print_point_color_at(i, j, black);
            }
        }
        for i in 455..457 {
            for j in 199..250 {
                self.layer1.print_point_color_at(i, j, black);
            }
        }
        for i in 299..457 {
            for j in 199..201 {
                self.layer1.print_point_color_at(i, j, black);
            }
        }
        for i in 299..457 {
            for j in 249..251 {
                self.layer1.print_point_color_at(i, j, black);
            }
        }
        let mut text_writer = self.layer1.text_writer_at(300, 100);
        text_writer.write_str("Please set up your");
        let mut text_writer = self.layer1.text_writer_at(300, 120);
        text_writer.write_fmt(format_args!("{} ship", ship_len));
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
            let y_end_pixel = y_start_pixel + (block_size * ship_size) - 4;
            //vertical
            for c in x_start_pixel..x_end_pixel {
                for i in y_start_pixel..y_end_pixel {
                    self.layer2.print_point_color_at(c, i, white);
                }
            }
        } else {
            let x_end_pixel = x_start_pixel + (block_size * ship_size) - 5;
            let y_end_pixel = y_start_pixel + block_size - 4;
            //horizontal
            for c in x_start_pixel..x_end_pixel {
                for i in y_start_pixel..y_end_pixel {
                    self.layer2.print_point_color_at(c, i, white);
                }
            }
        }
    }


    pub fn touch(&mut self) -> (u16, u16) {
        let mut touch_x = 0;
        let mut touch_y = 0;
        for touch in &touch::touches(&mut self.touchscreen).unwrap() {
            //let (x,y) = calculate_touch_block(touch.x, touch.y);
            touch_x = touch.x;
            touch_y = touch.y;
        }
        (touch_x, touch_y)
        //calculate_touch_block(touch_x, touch_y)

    }

    pub fn calculate_touch_block(&mut self, x: u16, y: u16) -> (u16,u16) {
        if x<=272 && x>24 && y <= 272 && y > 24 {
            let x_block = x/25;
            let y_block = y/25;
            (x_block,y_block)
        } else {
            (0,0)
        }
    }
}