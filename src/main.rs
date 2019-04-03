#![no_std]
#![no_main]
#![feature(alloc_error_handler,alloc)]
#![warn(clippy::all)]

#[macro_use]
extern crate alloc;

use alloc_cortex_m::CortexMHeap;
use core::fmt::Write;
use core::alloc::Layout as AllocLayout;
use core::panic::PanicInfo;
use cortex_m_rt::{entry, exception};
use stm32f7::stm32f7x6::{CorePeripherals, Peripherals};
use stm32f7_discovery::{
    gpio::{GpioPort, OutputPin},
    init,
    system_clock::{self, Hz},
    lcd::{self,Color},
    touch,
};
//use lcd::Framebuffer;
//use lcd::FramebufferL8;
//use lcd::TextWriter;

#[entry]
fn main() -> ! {
    let core_peripherals = CorePeripherals::take().unwrap();
    let mut systick = core_peripherals.SYST;

    let peripherals = Peripherals::take().unwrap();
    let mut rcc = peripherals.RCC;
    let mut pwr = peripherals.PWR;
    let mut flash = peripherals.FLASH;

    init::init_system_clock_216mhz(&mut rcc, &mut pwr, &mut flash);
    init::enable_gpio_ports(&mut rcc);
    let mut fmc = peripherals.FMC;
    let mut ltdc = peripherals.LTDC;
    let mut sai_2 = peripherals.SAI2;
    let mut rng = peripherals.RNG;
    let mut sdmmc = peripherals.SDMMC1;
    let mut syscfg = peripherals.SYSCFG;
    let mut ethernet_mac = peripherals.ETHERNET_MAC;
    let mut ethernet_dma = peripherals.ETHERNET_DMA;

    let gpio_a = GpioPort::new(peripherals.GPIOA);
    let gpio_b = GpioPort::new(peripherals.GPIOB);
    let gpio_c = GpioPort::new(peripherals.GPIOC);
    let gpio_d = GpioPort::new(peripherals.GPIOD);
    let gpio_e = GpioPort::new(peripherals.GPIOE);
    let gpio_f = GpioPort::new(peripherals.GPIOF);
    let gpio_g = GpioPort::new(peripherals.GPIOG);
    let gpio_h = GpioPort::new(peripherals.GPIOH);
    let gpio_i = GpioPort::new(peripherals.GPIOI);
    let gpio_j = GpioPort::new(peripherals.GPIOJ);
    let gpio_k = GpioPort::new(peripherals.GPIOK);
    let mut pins = init::pins(
        gpio_a, gpio_b, gpio_c, gpio_d, gpio_e, gpio_f, gpio_g, gpio_h, gpio_i, gpio_j, gpio_k,
    );


    // configure the systick timer 20Hz (20 ticks per second)
    init::init_systick(Hz(20), &mut systick, &rcc);
    systick.enable_interrupt();

    init::init_sdram(&mut rcc, &mut fmc);
    let mut lcd = init::init_lcd(&mut ltdc, &mut rcc);
    pins.display_enable.set(true);
    pins.backlight.set(true);
    let mut layer_1 = lcd.layer_1().unwrap();
    let mut layer_2 = lcd.layer_2().unwrap();
    //lcd::init_stdout(layer_1);
    let bg_color = Color{red: 255,green: 0 ,blue: 0,alpha: 255};
    let blue = Color{red: 0,green: 0 ,blue: 255,alpha: 255};
    let green = Color{red: 0,green: 255 ,blue: 0,alpha: 255};
    let black = Color{red: 0,green: 0 ,blue: 0,alpha: 255};
    let grey = Color{red: 127,green: 127 ,blue: 127,alpha: 255};
    layer_1.clear();
    layer_2.clear();
    lcd.set_background_color(blue);

    let mut i2c_3 = init::init_i2c_3(peripherals.I2C3, &mut rcc);
    i2c_3.test_2();
    i2c_3.test_2();
    touch::check_family_id(&mut i2c_3).unwrap();
        

    //let mut framebuffer = FramebufferL8::new();
    //framebuffer.init();
    //lcd.framebuffer_addr = framebuffer.get_framebuffer_addr() as u32;
    //lcd.backbuffer_addr = framebuffer.get_backbuffer_addr() as u32;

    let mut text_writer = layer_1.text_writer();
    //let lib_writer = TextWriter::new(TTF, 40.0);
    //text_writer.write_str("\n");
    //text_writer.write_str("     1  2  3  4  5  6  7  8  9  10");
    //text_writer.write_str("\n\n\n");
    //text_writer.write_str(" a");
    //text_writer.write_str("\n\n\n");
    //text_writer.write_str(" b");
    //text_writer.write_str("\n\n\n");
    //text_writer.write_str(" c");
    //text_writer.write_str("\n\n\n");
    //text_writer.write_str(" d");
    //text_writer.write_str("\n\n\n\n");
    //text_writer.write_str(" e");
    //text_writer.write_str("\n\n\n");
    //text_writer.write_str(" f");
    //text_writer.write_str("\n\n\n");
    //text_writer.write_str(" g");
    //text_writer.write_str("\n\n\n");
    //text_writer.write_str(" h");
    //text_writer.write_str("\n\n\n");
    //text_writer.write_str(" i");
    //text_writer.write_str("\n\n\n");
    //text_writer.write_str(" j");
    text_writer.x_pos=35;
    text_writer.y_pos=9;
    text_writer.write_str("1");
    text_writer.x_pos=60;
    text_writer.y_pos=9;
    text_writer.write_str("2");
    text_writer.x_pos=85;
    text_writer.y_pos=9;
    text_writer.write_str("3");
    text_writer.x_pos=110;
    text_writer.y_pos=9;
    text_writer.write_str("4");
    text_writer.x_pos=135;
    text_writer.y_pos=9;
    text_writer.write_str("5");
    text_writer.x_pos=160;
    text_writer.y_pos=9;
    text_writer.write_str("6");
    text_writer.x_pos=185;
    text_writer.y_pos=9;
    text_writer.write_str("7");
    text_writer.x_pos=210;
    text_writer.y_pos=9;
    text_writer.write_str("8");
    text_writer.x_pos=235;
    text_writer.y_pos=9;
    text_writer.write_str("9");
    text_writer.x_pos=255;
    text_writer.y_pos=9;
    text_writer.write_str("10");


    text_writer.x_pos=9;
    text_writer.y_pos=35;
    text_writer.write_str("a");
    text_writer.x_pos=9;
    text_writer.y_pos=60;
    text_writer.write_str("b");
    text_writer.x_pos=9;
    text_writer.y_pos=85;
    text_writer.write_str("c");
    text_writer.x_pos=9;
    text_writer.y_pos=110;
    text_writer.write_str("d");
    text_writer.x_pos=9;
    text_writer.y_pos=135;
    text_writer.write_str("e");
    text_writer.x_pos=9;
    text_writer.y_pos=160;
    text_writer.write_str("f");
    text_writer.x_pos=9;
    text_writer.y_pos=185;
    text_writer.write_str("g");
    text_writer.x_pos=9;
    text_writer.y_pos=210;
    text_writer.write_str("h");
    text_writer.x_pos=9;
    text_writer.y_pos=235;
    text_writer.write_str("i");
    text_writer.x_pos=9;
    text_writer.y_pos=260;
    text_writer.write_str("j");

    write_in_field(3,3,&mut text_writer,"X");
    write_in_field(4,4,&mut text_writer,"O");
    //write_in_field(3,3, text_writer,"O");
    //lib_writer.write_at(framebuffer, "hi", 50, 50);
    //text_writer.x_pos = 20;
    

    let arr = [24,25,49,50,74,75,99,100,124,125,149,150,174,175,199,200,224,225,249,250,274,275];
    //let arr = [200,210,220];
    //let arr = [10,20,30];
    let arr2 = [24,25,49,50,74,75,99,100,124,125,149,150,174,175,199,200,224,225,249,250];
    //let arr2 = [24,48,72,96,120,144,168,192,216,240,264];
    //let ship1 = []

    // turn led on
    pins.led.set(true);

    let mut last_led_toggle = system_clock::ticks();
    loop {

// poll for new touch data  u
        for touch in &touch::touches(&mut i2c_3).unwrap() {
            layer_2.print_point_color_at(
                touch.x as usize,
                touch.y as usize,
                Color::from_hex(0xffffff),
            );
        }

        let ticks = system_clock::ticks();
        // every 0.5 seconds (we have 20 ticks per second)
        if ticks - last_led_toggle >= 10 {
            pins.led.toggle();
            last_led_toggle = ticks;
        }
        //let color = [0xff00, 0x0f00]; //yellow;green
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
        for c in 53..72 {
            for i in 78..172 {
                layer_2.print_point_color_at(c, i, grey);
            }
        }
        //layer_1.clear();
        //layer_2.clear();

        
    }
}

fn write_in_field(x: usize, y: usize, mut text_writer: &mut stm32f7_discovery::lcd::TextWriter<stm32f7_discovery::lcd::FramebufferArgb8888>, letter: &str) {
    let x_pos = 9 + 25 * x;
    let y_pos = 9 + 25 * y;
    if x == 0 {let x_pos = 9;};
    if y == 0 {let y_pos = 9;};
    text_writer.x_pos = x_pos;
    text_writer.y_pos = y_pos;
    text_writer.write_str(letter);
}

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[exception]
fn SysTick() {
    system_clock::tick();
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn rust_oom(_: AllocLayout) -> ! {
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use core::fmt::Write;
    use cortex_m::asm;
    use cortex_m_semihosting::hio;

    if let Ok(mut hstdout) = hio::hstdout() {
        let _ = writeln!(hstdout, "{}", info);
    }

    // OK to fire a breakpoint here because we know the microcontroller is connected to a debugger
    asm::bkpt();

    loop {}
}

