#![no_std]
#![no_main]
#![feature(alloc_error_handler,alloc)]
#![warn(clippy::all)]

#[macro_use]
extern crate alloc;

use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout as AllocLayout;
use core::panic::PanicInfo;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{hprintln};
use stm32f7::stm32f7x6::{CorePeripherals, Peripherals};
use stm32f7_discovery::{
    gpio::{GpioPort, OutputPin},
    init,
    system_clock::{self, Hz},
    touch,
};
mod display;
mod ships;
mod game;
mod gameboard;
mod network;

const IS_SERVER: bool = false;

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
    let mut syscfg = peripherals.SYSCFG;
    let mut ethernet_mac = peripherals.ETHERNET_MAC;
    let ethernet_dma = peripherals.ETHERNET_DMA;

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

    let mut touchscreen = init::init_i2c_3(peripherals.I2C3, &mut rcc);
    touchscreen.test_2();
    touchscreen.test_2();
    //we need an empty loop here to wait for the touch scree to be initialized. Otherwise the release build crashes
    let ticks = system_clock::ticks();
    while system_clock::ticks() - ticks <= 10 {}
    touch::check_family_id(&mut touchscreen).unwrap();

    let display = display::init_display(&mut lcd, touchscreen);

    // Initialize the allocator BEFORE you use it
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, 50_000) }

    // turn led on
    pins.led.set(true);

    let net = network::init(&mut rcc, &mut syscfg, &mut ethernet_mac, ethernet_dma, IS_SERVER);
    
    match net {
        Ok(value) => {
            let nw: network::Network = value;
            let mut game = game::init_new_game(display , nw, IS_SERVER);
            game.run_game();
            match hprintln!("connected") {_ => {}}
        }
        Err(_e) => {match hprintln!("failed to init network") {_ => {}}}
    }


    let mut last_led_toggle = system_clock::ticks();
    
    loop {

        let ticks = system_clock::ticks();
        // every 0.5 seconds (we have 20 ticks per second)
        if ticks - last_led_toggle >= 10 {
            pins.led.toggle();
            last_led_toggle = ticks;
        }
    }
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

