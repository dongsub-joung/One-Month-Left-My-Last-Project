use core::panic;
use pktmon::{
    Capture,
    filter::{PktMonFilter, TransportProtocol},
};
use std::*;
use windows::{Win32::System::Threading::*, core::*};

#[allow(non_camel_case_types)]
pub enum LoggingOptions {
    // <Windows tail title>: receiver IP / DNS address
    NETWORK_ACTIVITY_MODE,
    // keylogger
    KEYBOARD_ONLY,
    // mouselogger
    MOUSE_ONLY,
    // All with multi-threading
    ALL,
}

struct Data<T> {
    timestamp: std::time::SystemTime,
    inner_data: T,
}

struct NetworkPacketData {
    // @TODO Data<T> will have Packet type
    packet_data: Data<Vec<usize>>,
    // sender_email: &' static str
}

trait BufferedData {
    type DataType;

    fn new() -> Self;
    // fn from(buffered_data: NetworkPacketData, _data: Vec<usize>) -> Self;
    fn unwrap_data(self) -> Data<Vec<usize>>;
    fn borrowing(&self) -> &Self;
}

impl BufferedData for NetworkPacketData {
    // @TODO will have Packet type
    type DataType = Vec<usize>;

    fn new() -> Self {
        let timestamp = std::time::SystemTime::now();
        let inner_data: Vec<usize> = Vec::new();

        let packet_data = Data {
            timestamp,
            inner_data,
        };

        Self { packet_data }
    }
    // fn from(buffered_data: &Self, _data: Vec<usize>) -> Self {
    //     let mut packet_data = buffered_data.packet_data;
    //     packet_data.inner_data.extend(_data);

    //     Self { packet_data }
    // }
    fn unwrap_data(self) -> Data<Vec<usize>> {
        self.packet_data
    }
    fn borrowing(&self) -> &NetworkPacketData {
        &self
    }
    // fn from(buffered_data: &Self, _data: Vec<usize>) -> Self {
    //     let mut packet_data = buffered_data.packet_data;
    //     packet_data.inner_data.extend(_data);

    //     Self { packet_data }
    // }
}

// @TODO
// struct KeyboardData {}
// impl BufferedData for KeyboardData {}
// struct MouseData {}
// impl BufferedData for MouseData {}

pub struct TheWatcher {
    pid: u32, // type.c_ulong
    logging_flag: bool,
    output_path: &'static str,
    csv_option: bool,
    option: LoggingOptions,
    buffered_data: NetworkPacketData,
    target: (String, String),
    // data_bus_stream: Someting,
}

impl TheWatcher {
    pub fn new(pid: u32, output_path: &'static str) -> Self {
        let logging_flag = true;
        let csv_option = Default::default();
        let option = LoggingOptions::NETWORK_ACTIVITY_MODE;
        let buffered_data = BufferedData::new();
        let target = (Default::default(), Default::default());
        // let data_bus_stream= something;

        Self {
            pid,
            logging_flag,
            output_path,
            csv_option,
            option,
            buffered_data,
            target,
            // data_bus_stream,
        }
    }

    //  If AI can drop some codes like this logic,
    //  malware do not need anymore :)
    //  just conect PC, and then drop that code remotely.
    pub fn setting_target(&mut self) -> &mut Self {
        #[cfg(windows)]
        {
            // unit test done - fn get_name_process
            unsafe fn get_name_process(pid: u32) -> windows::core::Result<String> {
                unsafe {
                    let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid)?;

                    const BUFFER_MAX_SIZE: usize = 2028;
                    let mut buffer = [0u16; BUFFER_MAX_SIZE];
                    let mut buffer_size = buffer.len() as u32;

                    QueryFullProcessImageNameW(
                        handle,
                        windows::Win32::System::Threading::PROCESS_NAME_WIN32,
                        PWSTR(buffer.as_mut_ptr()),
                        &mut buffer_size,
                    )?;

                    Ok(String::from_utf16_lossy(&buffer[..buffer_size as usize]))
                }
            }

            fn filter_absolut_path(raw_path: String) -> (String, String) {
                let mut v_strs: Vec<&str> = raw_path[..raw_path.len()].split(r#"\\"#).collect();

                let exe_name = v_strs.pop().expect("fail to unwrap at exe_name");
                let program_name = v_strs.pop().expect("fail to unwrap at program_name");
                (program_name.to_string(), exe_name.to_string())
            }

            let process_full_name: String;
            unsafe {
                process_full_name =
                    get_name_process(self.pid).expect("fail to fn get name to process");
            };

            self.target = filter_absolut_path(process_full_name);
        }

        #[cfg(not(windows))]
        {
            // @TODO hook a daemon
        }

        self
    }

    fn filtering_data(_stream_data: &Vec<usize>) -> String {
        let mut filtered_string = String::new();

        filtered_string
    }

    // fn packet_captureing(exe_name: (String, String)) {
    //     #[cfg(windows)]
    //     {
    //         let exe_name = exe_name.clone();

    //         fn set_interface(exe_name: &(String, String)) {
    //             let v_interfaces = pnet::datalink::interfaces();
    //             let _interface = v_interfaces
    //                 .iter()
    //                 .find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty());

    //             let mut receiver = match _interface {
    //                 Some(interface) => {
    //                     println!("Found default interface with [{}]", interface.name);

    //                     let setted_channel = pnet_datalink::channel(interface, Default::default());
    //                     let (_, rx) = match setted_channel {
    //                         Ok(pnet_datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
    //                         Ok(_) => panic!("nothing in value"),
    //                         Err(e) => panic!("Err: {} from the datalink channel", e),
    //                     };

    //                     rx
    //                 }
    //                 None => panic!("fail to packet_captureing on getting receiver"),
    //             };

    //             loop {
    //                 match receiver.next() {
    //                     Ok(packet) => {
    //                         if let Some(ethernet_packet) =
    //                             pnet::packet::ethernet::EthernetPacket::new(packet)
    //                         {
    //                             let converted_wire_format =
    //                                 pnet::packet::FromPacket::from_packet(&ethernet_packet);

    //                             println!(
    //                                 "<Active: {}/{}> destination: {} | ethertype: {}",
    //                                 exe_name.0,
    //                                 exe_name.1,
    //                                 converted_wire_format.destination,
    //                                 converted_wire_format.ethertype,
    //                             );
    //                         }
    //                     }
    //                     Err(e) => {
    //                         println!("Err while reading: {}", e);
    //                     }
    //                 }
    //             }
    //         }

    //         set_interface(&exe_name);
    //     }
    //     #[cfg(not(windows))]
    //     {
    //         // @TODO
    //     }
    // }

    pub fn packet_captureing_without_nmap(exe_name: (String, String)) {
        let mut capture_barve = Capture::new().unwrap();
        let mut capture_chrome = Capture::new().unwrap();
        let mut capture_firefox = Capture::new().unwrap();
        
        let exe_name1= exe_name.clone();
        let exe_name2= exe_name.clone();
        let exe_name3= exe_name.clone();

        std::thread::spawn(move || {
            capture_barve
                .add_filter(PktMonFilter {
                    name: "UDP Filter".to_string(),
                    transport_protocol: Some(TransportProtocol::UDP),
                    port: 5353.into(), // brave

                    ..PktMonFilter::default()
                })
                .unwrap();

            capture_barve.start().unwrap();

            let packet = capture_barve.next_packet().unwrap();

            println!(
                "<Active: {}/{}> payload: {:?}",
                exe_name1.0, exe_name1.1, packet.payload,
            );
        });

        
        std::thread::spawn(move || {
            capture_chrome
                .add_filter(PktMonFilter {
                    name: "UDP Filter".to_string(),
                    transport_protocol: Some(TransportProtocol::UDP),
                    port: 443.into(), // chrome, but Default Ports: While 80 and 443 are standard

                    ..PktMonFilter::default()
                })
                .unwrap();

            capture_chrome.start().unwrap();

            let packet = capture_chrome.next_packet().unwrap();
            println!(
                "<Active: {}/{}> payload: {:?}",
                exe_name2.0, exe_name2.1, packet.payload,
            );
        });

        std::thread::spawn(move || {
            capture_firefox
                .add_filter(PktMonFilter {
                    name: "UDP Filter".to_string(),
                    transport_protocol: Some(TransportProtocol::UDP),
                    port: 443 .into(), // firefox

                    ..PktMonFilter::default()
                })
                .unwrap();

            capture_firefox.start().unwrap();

            let packet = capture_firefox.next_packet().unwrap();
            println!(
                "<Active: {}/{}> payload: {:?}",
                exe_name3.0, exe_name3.1, packet.payload,
            );
        });
    }

    pub async fn logging(&mut self, flag: bool, option: LoggingOptions) -> &mut Self {
        self.logging_flag = flag;
        self.option = option;

        let exe_name = self.target.clone();

        // @TODO fillter
        // packets <- thread 3
        match &self.option {
            LoggingOptions::NETWORK_ACTIVITY_MODE => {
                std::thread::spawn(move || {
                    Self::packet_captureing_without_nmap(exe_name);
                });
            }
            _ => {}
        }

        // @TODO
        // self.buffered_data = BufferedData::from(self.buffered_data, stream_data);

        self
    }
}

// pub fn output_txt_path(&mut self, flag: bool) -> Result<&mut Self> {
//     let path = &self.output_path;
//     let full_path = format!("{}/output.txt", path);

//     match flag {
//         true => {
//             let mut file = File::create(full_path)?;

//             let unwrapped_data = BufferedData::unwrap_data(self.buffered_data);
//             let filtered_data = filtering_data(unwrapped_data);

//             file.write_all(filtered_data.as_bytes())?;
//         }
//         false => {
//             // @TODO send data to sender as email
//         }
//     }
//     Ok(self)
// }

// pub fn csv_format_option(&mut self, flag: bool) -> Result<&mut Self> {
//     Ok(self)
// }

// async fn read_data_stream(data_bus_stream: stream) -> Result<Vec<usize>, Box<dyn Error>> {
//     static CAPACITY_LINE: usize= 1024000000;
//     let mut unwrapped_data: Vec<usize> = Vec::with_capacity(CAPACITY_LINE);

//     // !TODO if returned err, try to reconn
//     loop{
//         let mut buffer_result: Vec<usize> = Vec::new();

//         // @TODO https://docs.rs/tokio/latest/tokio/io/trait.AsyncRead.html
//         let pointer: core::task::Pin<&mut Self>;
//         buffer_result = AsyncRead::poll_read(
//             pointer,
//             data_bus_stream,
//             buffer_result,
//         )?;

//         // !TODO define buffer_result max size 1024000000 ~ +5000000)
//         match buffer_result {
//             Ok(data) => {
//                 unwrapped_data= data;
//                 break;
//             }
//             Err(e) => {
//                 return Err("can't get data bus stream".into());
//             }
//         }
//     }

//     if unwrapped_data.capacity() >= CAPACITY_LINE {
//         return Ok(unwrapped_data);
//     } else if unwrapped_data.is_empty(){
//         return Err("No data collected".into());
//     }

//     Ok(unwrapped_data)
// }
