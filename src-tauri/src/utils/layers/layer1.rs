use super::packet_info;
use pnet::packet::{ipv4::Ipv4Packet, ipv6::Ipv6Packet};

#[derive(Debug, Clone, PartialEq,)]
pub enum LayerOne {
    V4(Vec<u8>),
    V6(Vec<u8>)
}

use std::collections::HashMap;

impl packet_info::PacketInfo for LayerOne {
    fn get_info(&self) -> HashMap<String, String> {
        match self {
            LayerOne::V4(ip_data) => {
                if let Some(ip_packet) = Ipv4Packet::new(ip_data) {
                    let mut info = HashMap::new();
                    info.insert("Version".into(), "IPv4".into());
                    info.insert("Source IP".into(), ip_packet.get_source().to_string());
                    info.insert("Destination IP".into(), ip_packet.get_destination().to_string());
                    info.insert("TTL".into(), ip_packet.get_ttl().to_string());
                    info.insert("Total Length".into(), ip_packet.get_total_length().to_string());
                    info
                } else {
                    let mut error = HashMap::new();
                    error.insert("Error".into(), "Invalid IPv4 packet".into());
                    error
                }
            }
            LayerOne::V6(ip_data) => {
                if let Some(ip_packet) = Ipv6Packet::new(ip_data) {
                    let mut info = HashMap::new();
                    info.insert("Version".into(), "IPv6".into());
                    info.insert("Source IP".into(), ip_packet.get_source().to_string());
                    info.insert("Destination IP".into(), ip_packet.get_destination().to_string());
                    info.insert("Traffic Class".into(), ip_packet.get_traffic_class().to_string());
                    info.insert("Flow Label".into(), ip_packet.get_flow_label().to_string());
                    info
                } else {
                    let mut error = HashMap::new();
                    error.insert("Error".into(), "Invalid IPv6 packet".into());
                    error
                }
            }
        }
    }
}

// First vec is raw packet and second is the layer packet