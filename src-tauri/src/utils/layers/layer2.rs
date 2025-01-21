use pnet::packet::{icmpv6::Icmpv6Packet, tcp::TcpPacket, udp::UdpPacket};
use super::packet_info::PacketInfo;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum LayerTwo {
    TCP(Vec<u8>),
    UDP(Vec<u8>),
    ICMPV6(Vec<u8>)
}


// Implement for LayerTwo

impl PacketInfo for LayerTwo {
    fn get_info(&self) -> HashMap<String, String> {
        match self {
            LayerTwo::TCP(tcp_data) => {
                if let Some(tcp_packet) = TcpPacket::new(tcp_data) {
                    let mut info = HashMap::new();
                    info.insert("Protocol".into(), "TCP".into());
                    info.insert("Source Port".into(), tcp_packet.get_source().to_string());
                    info.insert("Destination Port".into(), tcp_packet.get_destination().to_string());
                    info.insert("Sequence".into(), tcp_packet.get_sequence().to_string());
                    info.insert("Window Size".into(), tcp_packet.get_window().to_string());
                    info.insert(
                        "Flags".into(),
                        format!(
                            "ACK:{} RST:{} SYN:{} FIN:{}",
                            tcp_packet.get_flags() & 0b00010000 != 0,
                            tcp_packet.get_flags() & 0b00000100 != 0,
                            tcp_packet.get_flags() & 0b00000010 != 0,
                            tcp_packet.get_flags() & 0b00000001 != 0,
                        ),
                    );
                    info
                } else {
                    let mut error = HashMap::new();
                    error.insert("Error".into(), "Invalid TCP packet".into());
                    error
                }
            }
            LayerTwo::UDP(udp_data) => {
                if let Some(udp_packet) = UdpPacket::new(udp_data) {
                    let mut info = HashMap::new();
                    info.insert("Protocol".into(), "UDP".into());
                    info.insert("Source Port".into(), udp_packet.get_source().to_string());
                    info.insert("Destination Port".into(), udp_packet.get_destination().to_string());
                    info.insert("Length".into(), udp_packet.get_length().to_string());
                    info.insert("Checksum".into(), format!("0x{:04x}", udp_packet.get_checksum()));
                    info
                } else {
                    let mut error = HashMap::new();
                    error.insert("Error".into(), "Invalid UDP packet".into());
                    error
                }
            }
            LayerTwo::ICMPV6(icmp_data) => {
                if let Some(icmp_packet) = Icmpv6Packet::new(icmp_data) {
                    let mut info = HashMap::new();
                    info.insert("Protocol".into(), "ICMPv6".into());
                    info.insert("Checksum".into(), format!("0x{:04x}", icmp_packet.get_checksum()));
                    info
                } else {
                    let mut error = HashMap::new();
                    error.insert("Error".into(), "Invalid ICMPv6 packet".into());
                    error
                }
            }
        }
    }
}
