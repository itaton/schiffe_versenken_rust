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
const CLIENT_ETH_ADDR: EthernetAddress = EthernetAddress([0x00, 0x11, 0x22, 0x33, 0x44, 0x01]);
const CLIENT_IP_ADDR: Ipv4Address = Ipv4Address([192, 168, 42, 2]);
const SERVER_ETH_ADDR: EthernetAddress = EthernetAddress([0x00, 0x11, 0x22, 0x33, 0x44, 0x02]);
const SERVER_IP_ADDR: Ipv4Address = Ipv4Address([192, 168, 42, 1]);

pub struct Network {
    ethernet_interface: EthernetInterface<'static, 'static, 'static, ethernet::EthernetDevice>,
    sockets: SocketSet<'static, 'static, 'static>,
    partner_ip_addr: Ipv4Address,
}

impl Network {
    pub fn get_udp_packet(&mut self) -> Result<Option<Vec<u8>>, smoltcp::Error> {
        match self.ethernet_interface.poll(
            &mut self.sockets,
            Instant::from_millis(system_clock::ms() as i64),
        ) {
            Err(smoltcp::Error::Exhausted) => {
                // Exhausted may mean full -> we need to read more
                // hprintln!("exhausted");
                // let mut socket = &mut self.sockets.iter_mut().nth(0).unwrap();
                for mut socket in self.sockets.iter_mut() {
                    return Network::poll_udp_packet(&mut socket);
                }
                Err(smoltcp::Error::Illegal)
            },
            Err(e) => Err(e),
            Ok(socket_changed) => if socket_changed {
                // hprintln!("ok");
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
            Socket::Udp(ref mut socket) => { 

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

    pub fn pull_all(&mut self) {
        self.ethernet_interface.poll(&mut self.sockets, Instant::from_millis(system_clock::ms() as i64));
        for mut socket in self.sockets.iter_mut() {
            Network::poll_udp_packet(&mut socket);
        }
    }

    pub fn send_udp_packet(&mut self, data: &[u8]) {
        let endpoint = IpEndpoint::new(IpAddress::Ipv4(self.partner_ip_addr), PORT);
        for mut socket in self.sockets.iter_mut() {
            Network::push_udp_packet(&mut socket, endpoint, data);
        }
    }

    fn push_udp_packet(socket: &mut Socket, endpoint: IpEndpoint, data: &[u8]) {
        if let Socket::Udp(ref mut socket) = socket {
            if socket.can_send() {
                let result = socket.send_slice(data, endpoint); // TODO: Error handling
                match result {
                    Ok(_) => {}
                    Err(e) => {match hprintln!("error {:?}", e) {_ => {}}}
                }
            }
        }
    }
}

pub fn init(
    rcc: &mut RCC, 
    syscfg: &mut SYSCFG, 
    ethernet_mac: &mut ETHERNET_MAC, 
    ethernet_dma: ETHERNET_DMA,
    is_server: bool) -> Result<Network, ethernet::PhyError> {
    let ethernet_addr = if is_server {SERVER_ETH_ADDR} else {CLIENT_ETH_ADDR};
    let ip_addr = if is_server {SERVER_IP_ADDR} else {CLIENT_IP_ADDR};
    let partner_ip_addr = if is_server {CLIENT_IP_ADDR} else {SERVER_IP_ADDR};
    let ethernet_interface = ethernet::EthernetDevice::new(
        Default::default(),
        Default::default(),
        rcc,
        syscfg,
        ethernet_mac,
        ethernet_dma,
        ethernet_addr,
        ip_addr
    ).map(|device| {
        let iface = device.into_interface();
        let prev_ip_addr = iface.ipv4_addr().unwrap();
        (iface, prev_ip_addr)
    });
    if let Err(e) = ethernet_interface { return Err(e);}

    let mut sockets = SocketSet::new(Vec::new());
    let endpoint = IpEndpoint::new(IpAddress::Ipv4(ip_addr), PORT);
    match hprintln!("IP: {:?}", ip_addr) {_ => {}}

    let udp_rx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY; 20], vec![0u8; 512]);
    let udp_tx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY; 20], vec![0u8; 512]);
    let mut udp_socket = UdpSocket::new(udp_rx_buffer, udp_tx_buffer);
    udp_socket.bind(endpoint).unwrap();
    sockets.add(udp_socket);

    Ok(Network {
        ethernet_interface: ethernet_interface.unwrap().0,
        sockets,
        partner_ip_addr,
    })
}

pub trait Connection {
    fn send_shoot(&mut self, network: &mut Network, shoot: ShootPacket);
    fn recv_shoot(&mut self, network: &mut Network) -> Option<ShootPacket>;
    fn send_feedback(&mut self, network: &mut Network, feedback: FeedbackPacket);
    fn recv_feedback(&mut self, network: &mut Network) -> Option<FeedbackPacket>;
    fn is_other_connected(&mut self, network: &mut Network) -> bool;
    fn send_whoami(&mut self, network: &mut Network);
}

pub struct EthClient {
    pub is_server: bool
}

impl EthClient {
    pub fn new(server: bool) -> EthClient {
        EthClient {
            is_server: server
        }
    }
}

impl Connection for EthClient {
    fn send_shoot(&mut self, network: &mut Network, shoot: ShootPacket) {
        // hprintln!("sent: {:?}", shoot);
        network.send_udp_packet(&shoot.serialize());
    }

    fn recv_shoot(&mut self, network: &mut Network) -> Option<ShootPacket> {
        let result = network.get_udp_packet();
        match result {
            Ok(value) => if let Some(data) = value {
                if data.len() == ShootPacket::len() {
                    let shoot = ShootPacket::deserialize(&data);
                    // hprintln!("received: {:?}", shoot);
                    return Some(shoot);
                }
                else {
                    match hprintln!("wrong package length") {_ => {}}
                }
            },
            //Err(smoltcp::Error::Exhausted) => {}
            //Err(smoltcp::Error::Unrecognized) => {}
            Err(e) => {
                match hprintln!("error: {:?}", e) {_ => {}}
            }
        }
        None
    }

    fn send_feedback(&mut self, network: &mut Network, feedback: FeedbackPacket) {
        //hprintln!("sent: {:?}", feedback);
        network.send_udp_packet(&feedback.serialize());
    }

    fn recv_feedback(&mut self, network: &mut Network) -> Option<FeedbackPacket> {
        let result = network.get_udp_packet();
        match result {
            Ok(value) => if let Some(data) = value {
                if data.len() == FeedbackPacket::len() {
                    let feedback = FeedbackPacket::deserialize(&data);
                    // hprintln!("received: {:?}", feedback);
                    return Some(feedback);
                }
            },
            Err(smoltcp::Error::Exhausted) => {}
            Err(smoltcp::Error::Unrecognized) => {}
            Err(e) => {
                match hprintln!("error: {:?}", e) {_ => {}}
            }
        }
        None
    }

    fn is_other_connected(&mut self, network: &mut Network) -> bool {
        let result = network.get_udp_packet();
        match result {
            Ok(value) => if let Some(data) = value {
                if data.len() == WhoamiPacket::len() {
                    return true;
                }
                if data.len() == FeedbackPacket::len() {
                    return true;
                }
                if data.len() == ShootPacket::len() {
                    return true;
                }
            },
            Err(e) => {
                match hprintln!("errortest: {:?}", e) {_ => {}}
            }
        }
        false
    }

    fn send_whoami(&mut self, network: &mut Network) {
        network.send_udp_packet(&WhoamiPacket {is_server: self.is_server}.serialize());
    }
}