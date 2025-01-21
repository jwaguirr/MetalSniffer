use std::collections::HashMap;

use super::packet_info::PacketInfo;

#[derive(Debug, Clone)]
pub enum LayerThree {
    DNS(DNS),
    MDNS(MDNS),
    HTTP(HTTP),
    HTTPS(HTTPS),
    Raw(Raw)
}

#[derive(Debug, Clone)]
pub struct DNS {
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct MDNS {
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct HTTP {
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct HTTPS {
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Raw {
    pub payload: Vec<u8>,
}

// Implement for LayerThree protocols
impl PacketInfo for LayerThree {
    fn get_info(&self) -> HashMap<String, String> {
        match self {
            LayerThree::DNS(dns) => {
                let mut info = HashMap::new();
                info.insert("Protocol".into(), "DNS".into());
                info.insert("Payload Length".into(), dns.payload.len().to_string());
                info
            }
            LayerThree::MDNS(mdns) => {
                let mut info = HashMap::new();
                info.insert("Protocol".into(), "MDNS".into());
                info.insert("Payload Length".into(), mdns.payload.len().to_string());
                info
            }
            LayerThree::HTTP(http) => {
                let mut info = HashMap::new();
                info.insert("Protocol".into(), "HTTP".into());
                info.insert("Payload Length".into(), http.payload.len().to_string());
                info
            }
            LayerThree::HTTPS(https) => {
                let mut info = HashMap::new();
                info.insert("Protocol".into(), "HTTPS".into());
                info.insert("Payload Length".into(), https.payload.len().to_string());
                info
            }
            LayerThree::Raw(raw) => {
                let mut info = HashMap::new();
                info.insert("Protocol".into(), "Raw/Unknown".into());
                info.insert("Payload Length".into(), raw.payload.len().to_string());
                info
            }
        }
    }
}
