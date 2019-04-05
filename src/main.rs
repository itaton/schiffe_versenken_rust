#![no_std]
#![no_main]
#![feature(alloc_error_handler,alloc)]
#![warn(clippy::all)]

#[macro_use]
extern crate alloc;

use smoltcp::{
    socket::{Socket, SocketSet, TcpSocket, TcpSocketBuffer, UdpPacketMetadata, UdpSocket, UdpSocketBuffer},
    time::{Instant},
    wire::{EthernetAddress, IpCidr, IpEndpoint},
};

use alloc::vec::Vec;
use alloc_cortex_m::CortexMHeap;
use core::fmt::Write;
use core::alloc::Layout as AllocLayout;
use core::panic::PanicInfo;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{hprint, hprintln};
use stm32f7::stm32f7x6::{CorePeripherals, Peripherals};
use stm32f7_discovery::{
    ethernet,
    gpio::{GpioPort, OutputPin},
    init,
    system_clock::{self, Hz},
    lcd::{self,Color,TextWriter,FramebufferArgb8888,Layer},
    touch,
};
mod display;
mod ships;
//mod game;
//mod gameboard;
mod network;
use network::EthClient;
use network::Connection;

//use lcd::Framebuffer;
//use lcd::FramebufferL8;
//use lcd::TextWriter;

const ETH_ADDR: EthernetAddress = EthernetAddress([0x00, 0x08, 0xdc, 0xab, 0xcd, 0xef]);
const PORT: u16 = 1337;
const is_server: bool = true;

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

    let mut touchscreen = init::init_i2c_3(peripherals.I2C3, &mut rcc);
    touchscreen.test_2();
    touchscreen.test_2();
    //we need an empty loop here to wait for the touch scree to be initialized. Otherwise the release build crashes
    let ticks = system_clock::ticks();
    while system_clock::ticks() - ticks <= 10 {}
    touch::check_family_id(&mut touchscreen).unwrap();

    let mut display = display::init_display(&mut lcd, touchscreen);

    // Initialize the allocator BEFORE you use it
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, 50_000) }

    let mut layer_1 = lcd.layer_1().unwrap();

    //display.write_in_field(3,3,&mut layer_1,"X");
    //display::write_in_field(4,4,&mut layer_1,"O");
    display.write_in_field(3,3,"X");
    display.write_in_field(4,4,"O");
    
    display.print_ship(4, 5, 5, true);


    display.setup_ship(5);
    //let ship1 = []

    // turn led on
    pins.led.set(true);

    let net = network::init(&mut rcc, &mut syscfg, &mut ethernet_mac, &mut ethernet_dma, is_server);
    test_network(net);


    /*let mut ethernet_interface = ethernet::EthernetDevice::new(
        Default::default(),
        Default::default(),
        &mut rcc,
        &mut syscfg,
        &mut ethernet_mac,
        &mut ethernet_dma,
        ETH_ADDR
    ).map(|device| {
        let iface = device.into_interface();
        let prev_ip_addr = iface.ipv4_addr().unwrap();
        (iface, prev_ip_addr)
    });
    if let Err(e) = ethernet_interface {
        hprintln!("ethernet init failed: {:?}", e);
    };

    let mut sockets = SocketSet::new(Vec::new());

    if let Ok((ref mut iface, ref mut prev_ip_addr)) = ethernet_interface {
        iface.update_ip_addrs(|ipa| *(ipa.first_mut().unwrap()) = IpCidr::new(smoltcp::wire::IpAddress::v4(192, 168, 42, 2), 24));
        hprintln!("assigned {}", iface.ipv4_addr().unwrap());

        let endpoint = IpEndpoint::new(iface.ipv4_addr().unwrap().into(), PORT);

        let udp_rx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY; 3], vec![0u8; 256]);
        let udp_tx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY; 1], vec![0u8; 128]);
        let mut udp_socket = UdpSocket::new(udp_rx_buffer, udp_tx_buffer);
        udp_socket.bind(endpoint).unwrap();
        sockets.add(udp_socket);

        let tcp_rx_buffer = TcpSocketBuffer::new(vec![0; ethernet::MTU]);
        let tcp_tx_buffer = TcpSocketBuffer::new(vec![0; ethernet::MTU]);
        let mut tcp_socket = TcpSocket::new(tcp_rx_buffer, tcp_tx_buffer);
        sockets.add(tcp_socket);

        loop {
            if let Ok((ref mut iface, ref mut prev_ip_addr)) = ethernet_interface {
			    let timestamp = Instant::from_millis(system_clock::ms() as i64);
				for mut sock in sockets.iter_mut() {
				    if let Socket::Tcp(ref mut sockt) = *sock {
					    if sockt.state() == smoltcp::socket::TcpState::CloseWait {
						    sockt.close();
						}
						if sockt.state() == smoltcp::socket::TcpState::Closed {
						    sockt.listen(endpoint);
						}
					}
				}
				match iface.poll(&mut sockets, timestamp) {
				    Err(::smoltcp::Error::Exhausted) => {
					    continue;
					}
					Err(::smoltcp::Error::Unrecognized) => {
                        hprintln!("U");
                    }
					Err(e) => {
                        hprintln!("Network error: {:?}", e);
                    }
					Ok(socket_changed) => {
					    if socket_changed {
						    for mut socket in sockets.iter_mut() {
							    poll_socket(&mut socket).expect("socket poll failed");
							}
						}
					}
				}
				iface
				    .poll_delay(&sockets, timestamp);
			};

        }
    }*/


    let mut last_led_toggle = system_clock::ticks();
    
    loop {

        let (x_pixel, y_pixel) = display.touch();
        let (x_block, y_block) = display.calculate_touch_block(x_pixel, y_pixel);
        //display.write_in_field(x_block, y_block, "x")
        if (x_block, y_block) != (0,0) {
            display.write_in_field(x_block as usize, y_block as usize, "x");
        }

        let ticks = system_clock::ticks();
        // every 0.5 seconds (we have 20 ticks per second)
        if ticks - last_led_toggle >= 10 {
            pins.led.toggle();
            last_led_toggle = ticks;
        }
        //layer_1.clear();
        //layer_2.clear();

        
    }
}

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[exception]
fn SysTick() {
    system_clock::tick();
}

/*fn poll_socket(socket: &mut Socket) -> Result<(), smoltcp::Error> {
    match socket {
        &mut Socket::Tcp(ref mut socket) => match socket.local_endpoint().port {
            PORT => {
                if !socket.may_recv() {
                    return Ok(());
                }
                let reply = socket.recv(|data| {
                    if data.len() > 0 {
                        (data.len(), 1)
                    }
                    else {
                        (data.len(), 0)
                    }
                })?;
                if reply == 1 {
                    hprintln!("recv success");
                    socket.send_slice(b"received packet");
                }
            }
            _ => {}
        },
        _ => {}
    }
    Ok(())
}*/


fn test_network(net: Result<network::Network, stm32f7_discovery::ethernet::PhyError>) {
   match net {
       Ok(value) => {
           let mut nw: network::Network = value;
           let mut client = EthClient::new(is_server);

           while !client.is_other_connected(&mut nw) {
               // hprintln!("not yet connected");
           }
           hprintln!("connected");

        //    loop {
        //        client.send_whoami(&mut nw);
        //    }

            loop {
                if is_server {
                    client.send_whoami(&mut nw);
                    hprintln!("ping");
                }
                while !client.is_other_connected(&mut nw) {

                }
                if !is_server {
                    client.send_whoami(&mut nw);
                    hprintln!("pong");
                }
            }

       },
       Err(e) => {
           hprintln!("connection error");
           cortex_m::asm::bkpt();
       }
   }
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
    // asm::bkpt();

    loop {}
}

