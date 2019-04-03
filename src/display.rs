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
    let arr = [24,48,72,96,120,144,168,192,216,240,264];
    let arr2 = [24,25,49,50,74,75,99,100,124,125,149,150,174,175,199,200,224,225,249,250];
    for c in arr.iter() {
        for i in 0..272 {
            layer_1.print_point_color_at(*c, i, black);
            }
    }
        for c in arr2.iter() {
            for i in 0..480 {
                layer_1.print_point_color_at(i, *c, black);
            }
        }
}        


