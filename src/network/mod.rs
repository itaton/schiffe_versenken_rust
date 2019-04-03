#![warn(clippy::all)]

use smoltcp;
use smoltcp::iface::EthernetInterface;
use smoltcp::socket::{Socket, UdpPacketMetadata, UdpSocket, UdpSocketBuffer};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, IpEndpoint, Ipv4Address};

use stm32f7::stm32f7x6::{RCC, SYSCFG, ETHERNET_MAC, ETHERNET_DMA};
use stm32f7_discovery::{ethernet};

const PORT: u16 = 1337;

struct Network {
    ethernet_interface: EthernetInterface<'static, 'static, ethernet::EthernetDevice>,
    socket: Socket<'static, 'static>,
    partner_ip_addr: Ipv4Address,
}

impl Network {
    fn send_udp_packet(&mut self, data: &[u8]) {
        let endpoint = IpEndpoint::new(IpAddress::Ipv4(self.partner_ip_addr), PORT);
        match self.socket {
            &mut Socket::Udp(socket) => {
                if socket.can_send() {
                    let _result = socket.send_slice(data, endpoint);
                }
            }
            _ => {}
        }
    }
}

fn init(
    rcc: &mut RCC,
    syscfg: &mut SYSCFG,
    ethernet_mac: &mut ETHERNET_MAC,
    ethernet_dma: &mut ETHERNET_DMA,
    ethernet_addr: EthernetAddress,
    ip_addr: Ipv4Address,
    partner_ip_addr: Ipv4Address,

) -> Result<Network, ethernet::PhyError> {
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

    let endpoint = IpEndpoint::new(IpAddress::Ipv4(ip_addr), PORT);

    let udp_rx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY; 3], vec![0u8; 512]);
    let udp_tx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY; 1], vec![0u8; 512]);
    let mut udp_socket = UdpSocket::new(udp_rx_buffer, udp_tx_buffer);
    udp_socket.bind(endpoint).unwrap();

    Ok(Network {
        ethernet_interface: ethernet_interface.unwrap(),
        socket: udp_socket,
        partner_ip: partner_ip,
    })
}