
use stm32f7_discovery::init::pins;
use stm32f7_discovery::{
    lcd::Lcd,
    lcd::Color,
};
pub fn init_display(lcd: &Lcd) {
    pins.display_enable.set(true);
    pins.backlight.set(true);
    let mut layer_1 = lcd.layer_1().unwrap();
    let mut layer_2 = lcd.layer_2().unwrap();
    layer_1.clear();
    layer_2.clear();

    let bg_color = Color{red: 0,green: 0 ,blue: 255,alpha: 190};
    set_background_color(bg_color);
    print_background();
}  

fn set_background_color(color: Color) {
    lcd.set_background_color(color);
}

fn print_background() {
    let arr = [24,48,72,96,120,144,168,192,216,240,264];

    for c in arr.iter() {
            //let i1 = 124 + 5 * c;
            //let i2 = 356 - 5 * c;
            //let j1 = 10 + 5 * c;
            //let j2 = 262 - 5 * c;
            //for i in i1..i2 {
        for i in 0..272 {
            layer_1.print_point_color_at(*c, i, black);
                //for j in j1..j2 {
                //    layer_1.print_point_color_at(i, j, blue);
                //}
            }
    }
        for c in arr2.iter() {
            for i in 0..480 {
                layer_1.print_point_color_at(i, *c, black);
            }
        }
}        


