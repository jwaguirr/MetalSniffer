use base64::Engine;
use pcap::{Capture, Device};
use pcap::BpfProgram;
use pnet::packet::ethernet::EthernetPacket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use tauri::{AppHandle, Emitter, State};
mod utils;
use utils::custom_packet::{CustomPacket, MyPacket};
use chrono::{DateTime, Local, NaiveDateTime, Timelike};
use std::collections::HashMap;
use std::sync::Mutex;
use base64::{engine::general_purpose::STANDARD};

use std::mem;

// Modify CaptureState to include filter
#[derive(Default)]
struct CaptureState {
    running: Arc<AtomicBool>,
    filter: Arc<Mutex<String>>, // Add filter storage
}

fn format_timeval(tv: libc::timeval) -> String {
    // Extract seconds and microseconds
    let local_time: DateTime<Local> = DateTime::from_timestamp(tv.tv_sec, (tv.tv_usec * 1000) as u32)
    .expect("Invalid timestamp")
    .into();

// Format the DateTime into the desired string format
    local_time.format("%m/%d/%Y %H:%M:%S").to_string()
}

// Add new command to set filter
#[tauri::command]
fn set_packet_filter(filter: String, state: State<CaptureState>) -> Result<(), String> {
    // Validate filter syntax before storing
    // Create a temporary capture to test the filter
    let device = Device::lookup()
        .map_err(|e| format!("Device lookup error: {}", e))?
        .ok_or("No device found")?;
    
    let cap_builder = Capture::from_device(device)
        .map_err(|e| format!("Capture error: {}", e))?;
    
    let mut test_capture = cap_builder
        .open()
        .map_err(|e| format!("Open capture error: {}", e))?;
    
    // Test if filter is valid
    test_capture
        .filter(&filter, true)
        .map_err(|e| format!("Invalid filter syntax: {}", e))?;
    
    // If we get here, filter is valid - store it
    if let Ok(mut current_filter) = state.filter.lock() {
        *current_filter = filter;
        Ok(())
    } else {
        Err("Failed to set filter".to_string())
    }
}

// Modify start_packet_capture to use the stored filter
#[tauri::command]
fn start_packet_capture(app: AppHandle, state: State<CaptureState>) {
    let running = state.running.clone();
    let filter = state.filter.clone(); // Clone filter reference
    running.store(true, Ordering::SeqCst);

    thread::spawn(move || {
        println!("Starting packet capture...");
        let device = match Device::lookup() {
            Ok(Some(device)) => device,
            _ => {
                eprintln!("Failed to find a network device.");
                return;
            }
        };

        let cap_builder = Capture::from_device(device)
            .unwrap()
            .immediate_mode(true)
            .snaplen(65535)
            .promisc(true);

        let mut capture = match cap_builder.open() {
            Ok(capture) => capture,
            Err(e) => {
                eprintln!("Failed to open capture: {:?}", e);
                return;
            }
        };

        // Apply the stored filter if one exists
        if let Ok(current_filter) = filter.lock() {
            if !current_filter.is_empty() {
                if let Err(e) = capture.filter(&current_filter, true) {
                    eprintln!("Failed to set filter: {:?}", e);
                    return;
                }
            }
        }

        // Rest of your capture loop remains the same
        while running.load(Ordering::SeqCst) {
                if let Ok(new_packet) = capture.next_packet() {
                    if let Some(ethernet_packet) = EthernetPacket::new(&new_packet.data) {
                        match MyPacket::new(&ethernet_packet) {
                            Ok(custom_packet) => {
                                let unix_timestamp: libc::timeval = new_packet.header.ts; // Your timestamp
                                let time: String = format_timeval(unix_timestamp);
                                let mut packet_info: HashMap<String, HashMap<String, String>> = custom_packet.get_packet_info();
                                let encode = STANDARD.encode(&new_packet.data);
                                packet_info.insert("raw_data".to_string(), HashMap::from([
                                    ("data".to_string(), encode),
                                    ("header".to_string(), format!("{:?}", new_packet.header))
                                ]));
                                packet_info.insert("timestamp".to_string(), HashMap::from([("timestamp".to_string(), time)]));
                                if let Err(e) = app.emit("packet-captured", packet_info) {
                                    eprintln!("Failed to emit packet event: {:?}", e);
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to process packet: {:?}", e);
                            }
                        }
                    }
            }
        }

        println!("Packet capture stopped.");
    });
}

#[tauri::command]
fn stop_packet_capture(state: State<CaptureState>) {
    state.running.store(false, Ordering::SeqCst); // Set the flag to false
    println!("Stopping packet capture...");
}

#[tauri::command]
fn filter_packets(packets: Vec<HashMap<String, HashMap<String, String>>>, filter: String) -> Result<Vec<HashMap<String, HashMap<String, String>>>, String> {
    // Create a temporary capture for filter compilation
    println!("Filter: {:?}", filter);
    let device = Device::lookup()
        .map_err(|e| format!("Device lookup error: {}", e))?
        .ok_or("No device found")?;
    
    let cap_builder = Capture::from_device(device)
        .map_err(|e| format!("Capture error: {}", e))?;
    
    let mut capture = cap_builder
        .open()
        .map_err(|e| format!("Open capture error: {}", e))?;

    // Compile the filter using the capture
    let program = capture.compile(filter.as_str(), true)
        .map_err(|e| format!("Failed to compile filter: {}", e))?;
    
    // Filter packets using stored raw data
    let filtered_packets: Vec<HashMap<String, HashMap<String, String>>> = packets
        .into_iter()
        .filter(|packet| {
            if let Some(raw_data) = packet.get("raw_data") {
                if let Some(data_str) = raw_data.get("data") {
                    // Decode base64 raw data
                    if let Ok(packet_data) = STANDARD.decode(data_str) {
                        // Apply the filter program to the packet data
                        program.filter(&packet_data)
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        })
        .collect();

    Ok(filtered_packets)
}
// Update the run function to include the new command
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(CaptureState::default())
        .invoke_handler(tauri::generate_handler![
            start_packet_capture, 
            stop_packet_capture,
            set_packet_filter,  // Add the new command
            filter_packets
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}