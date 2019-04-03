#![warn(clippy::all)]

use smoltcp;
use smoltcp::iface::EthernetInterface;
use smoltcp::socket::{Socket, SocketSet, UdpPacketMetadata, UdpSocket, UdpSocketBuffer};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, IpEndpoint, Ipv4Address};

pub mod packets;
use self::packets::ShootPacket;
use self::packets::FeedbackPacket;
use self::packets::WhoamiPacket;
use self::packets::Serializable;

use alloc::vec::Vec;
use stm32f7::stm32f7x6::{RCC, SYSCFG, ETHERNET_MAC, ETHERNET_DMA};
use stm32f7_discovery::{ethernet, system_clock};
use cortex_m_semihosting::hprintln;

const PORT: u16 = 1337;

pub struct Network {
    ethernet_interface: EthernetInterface<'static, 'static, 'static, ethernet::EthernetDevice<'static>>,
    sockets: SocketSet<'static, 'static, 'static>,
    partner_ip_addr: Ipv4Address,
}

impl Network {
    pub fn get_udp_packet(&mut self) -> Result<Option<Vec<u8>>, smoltcp::Error> {
        match self.ethernet_interface.poll(
            &mut self.sockets,
            Instant::from_millis(system_clock::ticks() as i64),
        ) {
            Err(smoltcp::Error::Exhausted) => {
                // Exhausted may mean full -> we need to read more

                // let mut socket = &mut self.sockets.iter_mut().nth(0).unwrap();
                for mut socket in self.sockets.iter_mut() {
                    return Network::poll_udp_packet(&mut socket);
                }
                Err(smoltcp::Error::Illegal)
            },
            Err(e) => Err(e),
            Ok(socket_changed) => if socket_changed {
                // let mut socket = &mut self.sockets.iter_mut().nth(0).unwrap();
                for mut socket in self.sockets.iter_mut() {
                    return Network::poll_udp_packet(&mut socket);
                }
                Ok(None)
            } else {
                Ok(None)
            },
        }
    }

    fn poll_udp_packet(socket: &mut Socket) -> Result<Option<Vec<u8>>, smoltcp::Error> {
        match socket {
            &mut Socket::Udp(ref mut socket) => { 

                if socket.can_recv() {
                    match socket.recv() {
                        Ok((data, _remote_endpoint)) => Ok(Some(Vec::from(data))),
                        Err(err) => Err(err),
                    }
                } else {
                    Ok(None)
                }
            },
            _ => Ok(None),
        }
    }

    pub fn send_udp_packet(&mut self, data: &[u8]) {
        let endpoint = IpEndpoint::new(IpAddress::Ipv4(self.partner_ip_addr), PORT);
        for mut socket in self.sockets.iter_mut() {
            Network::push_udp_packet(&mut socket, endpoint, data);
        }
    }

    fn push_udp_packet(socket: &mut Socket, endpoint: IpEndpoint, data: &[u8]) {
        match socket {
            &mut Socket::Udp(ref mut socket) => {
                if socket.can_send() {
                    let _result = socket.send_slice(data, endpoint); // TODO: Error handling
                }
            }
            _ => {}
        }
    }
}

pub fn init(
    rcc: &mut RCC, 
    syscfg: &mut SYSCFG, 
    ethernet_mac: &'static mut ETHERNET_MAC, 
    ethernet_dma: &'static mut ETHERNET_DMA,
    ethernet_addr: EthernetAddress, 
    ip_addr: Ipv4Address, 
    partner_ip_addr: Ipv4Address) -> Result<Network, ethernet::PhyError> {
    let ethernet_interface = ethernet::EthernetDevice::new(
        Default::default(),
        Default::default(),
        rcc,
        syscfg,
        ethernet_mac,
        ethernet_dma,
        ethernet_addr,
    ).map(|device| device.into_interface());
    if let Err(e) = ethernet_interface { return Err(e);}

    let mut sockets = SocketSet::new(Vec::new());
    let endpoint = IpEndpoint::new(IpAddress::Ipv4(ip_addr), PORT);

    let udp_rx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY; 3], vec![0u8; 512]);
    let udp_tx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY; 1], vec![0u8; 512]);
    let mut udp_socket = UdpSocket::new(udp_rx_buffer, udp_tx_buffer);
    udp_socket.bind(endpoint).unwrap();
    sockets.add(udp_socket);

    Ok(Network {
        ethernet_interface: ethernet_interface.unwrap(),
        sockets,
        partner_ip_addr,
    })
}

pub trait Connection {
    fn send_shoot(&mut self, network: &mut Network, shoot: &ShootPacket);
    fn recv_shoot(&mut self, network: &mut Network) -> ShootPacket;
    fn send_feedback(&mut self, network: &mut Network, feedback: &FeedbackPacket);
    fn recv_feedback(&mut self, network: &mut Network) -> FeedbackPacket;
    fn is_other_connected(&mut self, network: &mut Network) -> bool;
    fn send_whoami(&mut self, network: &mut Network);
}

pub struct Client {
    shoot: ShootPacket,
    feedback: FeedbackPacket,
    is_server: bool,
}

impl Connection for Client {
    fn send_shoot(&mut self, network: &mut Network, shoot: &ShootPacket) {
        network.send_udp_packet(&shoot.serialize());
    }

    fn recv_shoot(&mut self, network: &mut Network) -> ShootPacket {
        let result = network.get_udp_packet();
        match result {
            Ok(value) => match value {
                Some(data) => {
                    if data.len() == ShootPacket::len() {
                        self.shoot = ShootPacket::deserialize(&data);
                    }
                }
                None => {}
            },
            Err(smoltcp::Error::Exhausted) => {}
            Err(smoltcp::Error::Unrecognized) => {}
            Err(e) => {
                hprintln!("error: {:?}", e);
            }
        }
        self.shoot
    }

    fn send_feedback(&mut self, network: &mut Network, feedback: &FeedbackPacket) {
        network.send_udp_packet(&feedback.serialize());
    }

    fn recv_feedback(&mut self, network: &mut Network) -> FeedbackPacket {
        let result = network.get_udp_packet();
        match result {
            Ok(value) => match value {
                Some(data) => {
                    if data.len() == FeedbackPacket::len() {
                        self.feedback = FeedbackPacket::deserialize(&data);
                    }
                }
                None => {}
            },
            Err(smoltcp::Error::Exhausted) => {}
            Err(smoltcp::Error::Unrecognized) => {}
            Err(e) => {
                hprintln!("error: {:?}", e);
            }
        }
        self.feedback
    }

    fn is_other_connected(&mut self, network: &mut Network) -> bool {
        let result = network.get_udp_packet();
        match result {
            Ok(value) => match value {
                Some(data) => {
                    if data.len() == WhoamiPacket::len() {
                        return true;
                    }
                    if data.len() == FeedbackPacket::len() {
                        return true;
                    }
                    if data.len() == ShootPacket::len() {
                        return true;
                    }
                }
                None => {}
            },
            Err(e) => {
                hprintln!("error: {:?}", e);
            }
        }
        false
    }

    fn send_whoami(&mut self, network: &mut Network) {
        network.send_udp_packet(&WhoamiPacket {is_server: self.is_server}.serialize());
    }
}