use stm32f7_discovery::{
    lcd::FramebufferAl88,
    lcd::FramebufferArgb8888,
    lcd::Lcd,
    lcd::Color,
    lcd::Layer,
};

    static blue:Color = Color{red: 0,green: 0 ,blue: 255,alpha: 255};
    static green:Color = Color{red: 0,green: 255 ,blue: 0,alpha: 255};
    static black:Color = Color{red: 0,green: 0 ,blue: 0,alpha: 255};
    static grey:Color = Color{red: 127,green: 127 ,blue: 127,alpha: 127};
 
pub fn init_display(mut lcd: Lcd) {
    let mut layer_1 = lcd.layer_1().unwrap();
    let mut layer_2 = lcd.layer_2().unwrap();
    layer_1.clear();
    layer_2.clear();

    let bg_color = Color{red: 0,green: 0 ,blue: 255,alpha: 190};
    set_background_color(bg_color, lcd);
    print_background(layer_1);
}  

fn set_background_color(color: Color,mut lcd: Lcd) {
    lcd.set_background_color(color);
}

fn print_background(mut layer_1: Layer<FramebufferArgb8888>) {
    let arr = [24,25,49,50,74,75,99,100,124,125,149,150,174,175,199,200,224,225,249,250,274,275];
    let arr2 = [24,25,49,50,74,75,99,100,124,125,149,150,174,175,199,200,224,225,249,250];
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
            for i in 0..275 {
                layer_1.print_point_color_at(i, *c, black);
            }
        }

}        

pub fn printShip(mut layer_2: Layer<FramebufferAl88>) {
    for c in 53..72 {
        for i in 78..172 {
            layer_2.print_point_color_at(c, i, grey);
        }
    }
}

pub fn printRedCross(mut layer_2: Layer<FramebufferAl88>) {

}

pub fn printWhiteCross(mut layer_2: Layer<FramebufferAl88>) {

}


