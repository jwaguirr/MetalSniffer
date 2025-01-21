use std::collections::HashMap;
use std::io::Error;
use pnet::packet::Packet;
use pnet::packet::{ipv4::Ipv4Packet, ipv6::Ipv6Packet, udp::UdpPacket, tcp::TcpPacket, icmpv6::Icmpv6Packet};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use crate::utils::layers::packet_info::PacketInfo;

use super::layers::{layer1::*, layer2::*, layer3::*};

pub struct MyPacket {
    layer_1 : LayerOne,
    layer_2 : LayerTwo,
    layer_3 : Option<LayerThree>
}

impl MyPacket {
    pub fn new(layer_1_packet : &EthernetPacket) -> Result<MyPacket, Error> {
        // Creating the first layer of the network packet: IE. IP4 || IP6
        let layer_1: LayerOne = match layer_1_packet.get_ethertype() {
            EtherTypes::Ipv4 => {
                let v4_packet = Ipv4Packet::new(layer_1_packet.payload()).unwrap();
                LayerOne::V4(Vec::from(v4_packet.packet()))
            },
            EtherTypes::Ipv6 => {
                let v6_packet = Ipv6Packet::new(layer_1_packet.payload()).unwrap();
                LayerOne::V6(Vec::from(v6_packet.packet()))
            }
            _ => return Err(Error::new(std::io::ErrorKind::Unsupported, "Unsupported LayerOne protocol")),
        };

        // Creating the second layer of the network packet IE. UDP, TCP...
        let layer_2: LayerTwo = match &layer_1 {
            // Checking if the layer 1 is a v4 or v6, and updating the layer 2 values
            LayerOne::V4(v4_packet) => {
                // We can safely unwrap, because it has already been handled at level 1
                let v4_packet: Ipv4Packet<'_> = Ipv4Packet::new(v4_packet).unwrap();
                match v4_packet.get_next_level_protocol() {
                    IpNextHeaderProtocols::Udp => {
                        let udp_packet: UdpPacket<'_> = UdpPacket::new(v4_packet.payload()).unwrap();
                        LayerTwo::UDP(Vec::from(udp_packet.packet()))
                    },
                    IpNextHeaderProtocols::Tcp => {
                        let tcp_packet: TcpPacket<'_> = TcpPacket::new(v4_packet.payload()).unwrap();
                        LayerTwo::TCP(Vec::from(tcp_packet.packet()))
                    },
                    // Add more protocols
                    _ => return Err(Error::new(std::io::ErrorKind::Unsupported, "Unsupported LayerTwo protocol")),
                }
            }
            LayerOne::V6(v6_packet) => {
                // // We can safely unwrap, because it has already been handled at level 1
                let v6_packet: Ipv6Packet<'_> = Ipv6Packet::new(v6_packet).unwrap();
                match v6_packet.get_next_header() {
                    IpNextHeaderProtocols::Udp => {
                        let udp_packet: UdpPacket<'_> = UdpPacket::new(v6_packet.payload()).unwrap();
                        LayerTwo::UDP(Vec::from(udp_packet.packet()))
                    },
                    IpNextHeaderProtocols::Tcp => {
                        let tcp_packet: TcpPacket<'_> = TcpPacket::new(v6_packet.payload()).unwrap();
                        LayerTwo::TCP(Vec::from(tcp_packet.packet()))
                    },
                    IpNextHeaderProtocols::Icmpv6 => {
                        let tcp_packet: Icmpv6Packet<'_> = Icmpv6Packet::new(v6_packet.payload()).unwrap();
                        LayerTwo::ICMPV6(Vec::from(tcp_packet.packet()))

                    }
                    _ => return Err(Error::new(std::io::ErrorKind::Unsupported, "Unsupported LayerTwo protocol")),
                }
            }
        };
        let layer_3: Option<LayerThree> = match &layer_2 {
            LayerTwo::UDP(udp_packet) => {
                // We can safely unwrap, because it has already been handled at level 1
                let udp_packet = UdpPacket::new(udp_packet).unwrap();
                // Check for common ports this packet may be and convert
                let src_port = udp_packet.get_source();
                let dst_port = udp_packet.get_destination();
                match (src_port, dst_port) {
                    (53, _) | (_, 53) => Some(LayerThree::DNS(DNS {
                        payload: Vec::from(udp_packet.payload())
                    })),
                    (5353, _) | (_, 5353) => Some(LayerThree::MDNS(MDNS {
                        payload: Vec::from(udp_packet.payload())

                    })),
                    // Default to Raw if no match
                    _ => Some(LayerThree::Raw(Raw {
                        payload: Vec::from(udp_packet.payload())
                    })),
                }
            },
            LayerTwo::TCP(tcp_packet) => {
                // We can safely unwrap, because it has already been handled at level 1
                let tcp_packet: TcpPacket<'_> = TcpPacket::new(tcp_packet).unwrap();
                let src_port = tcp_packet.get_source();
                let dst_port = tcp_packet.get_destination();

                match (src_port, dst_port) {
                    // HTTP typically uses port 80
                    (80, _) | (_, 80) => Some(LayerThree::HTTP(HTTP {
                        payload: Vec::from(tcp_packet.payload())
                    })),
                    
                    // HTTPS typically uses port 443
                    (443, _) | (_, 443) => Some(LayerThree::HTTPS(HTTPS {
                        payload: Vec::from(tcp_packet.payload())
                    })),
                    
                    // Add other TCP-based protocols as needed

                    // Default to Raw if no match
                    _ => Some(LayerThree::Raw(Raw {
                        payload: Vec::from(tcp_packet.payload())
                    })),
                }
            },
            LayerTwo::ICMPV6(icmpv6_packet) => {
                // We can safely unwrap, because it has already been handled at level 1
                let icmpv6_packet: Icmpv6Packet<'_> = Icmpv6Packet::new(icmpv6_packet).unwrap();
                Some(LayerThree::Raw(Raw {
                    payload: Vec::from(icmpv6_packet.payload())
                }))

            },
            _ => None
        };

        Ok(MyPacket {
            layer_1,
            layer_2,
            layer_3
        })
            }
}

pub trait CustomPacket {
    fn get_network(&self) -> LayerOne;
    fn get_transport(&self) -> LayerTwo;
    fn get_application(&self) -> Option<LayerThree>;
    fn pretty_print(&self) -> ();
    fn get_packet_info(&self) ->  HashMap<String, HashMap<String, String>>;
}

impl CustomPacket for MyPacket {
    fn get_network(&self) -> LayerOne {
        self.layer_1.clone()
    }
    
    fn get_transport(&self) -> LayerTwo {
        self.layer_2.clone()
    }
    
    fn get_application(&self) -> Option<LayerThree> {
        self.layer_3.clone()
    }
    
    fn get_packet_info(&self) -> HashMap<String, HashMap<String, String>> {
        let mut packet_info: HashMap<String, HashMap<String, String>> = HashMap::new();
    
        // Add Layer 1 information
        packet_info.insert("Layer 1".to_string(), self.layer_1.get_info());
    
        // Add Layer 2 information
        packet_info.insert("Layer 2".to_string(), self.layer_2.get_info());
    
        // Add Layer 3 information if available
        if let Some(layer_3) = &self.layer_3 {
            packet_info.insert("Layer 3".to_string(), layer_3.get_info());
        }
    
        
        packet_info
    }
    
    

    
    fn pretty_print(&self) {
        println!("=== Packet Information ===\n");
        
        // Print Layer 1 information
        println!("Layer 1 (Network):");
        println!("----------------");
        for (key, value) in &self.layer_1.get_info() {
            println!("{}: {}", key, value);
        }
        
        // Print Layer 2 information
        println!("\nLayer 2 (Transport):");
        println!("------------------");
        for (key, value) in &self.layer_2.get_info() {
            println!("{}: {}", key, value);
        }
        
        // Print Layer 3 information if available
        if let Some(layer_3) = &self.layer_3 {
            println!("\nLayer 3 (Application):");
            println!("--------------------");
            for (key, value) in &layer_3.get_info() {
                println!("{}: {}", key, value);
            }
        }
    }
    
}