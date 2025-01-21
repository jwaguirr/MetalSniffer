use std::collections::HashMap;

pub trait PacketInfo {
    fn get_info(&self) -> HashMap<String, String>;
}