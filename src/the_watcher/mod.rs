use anyhow::Result;

use std::fs::File;
use std::io::prelude::*;
use windows::{
    core::*, 
    Data::Xml::Dom::*,
    Win32::Foundatuiion::*,
    Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};

#[allow(non_camel_case_types)]
pub enum LoggingOptions {
    ALL,
    KEYBOARD_ONLY,
    MOUSE_ONLY,
    NETWORK_ACTIVITY_MODE,
}

pub struct TheWatcher {
    pid: i32,
    logging_flag: bool,
    output_path: &'static str,
    csv_option: bool,
    option: LoggingOptions,
    // target: process::Something ,
    // data_bus_stream: Someting,
    buffered_data: BufferedData,
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
        let mut data= buffered_Data.data.clone();
        data.extend(_data);
        
        Self { data }
    }
    pub fn unwrap_data(self) -> Vec<usize> {
        self.data
    }
}

impl TheWatcher {
    pub fn new(pid: i32, output_path: &'static str) -> Self {
        let logging_flag = true;
        let csv_option = false;
        let option = LoggingOptions::ALL;
        // let target= process::new()
        // let data_bus_stream= something;

        let buffered_data = BufferedData::new();

        Self {
            pid,
            logging_flag,
            output_path,
            csv_option,
            option,
            buffered_data,
        } // target, data_bus_stream
    }
    
    pub fn setting_target(&mut self) -> &mut Self {
        let setted_target= cfg_select!{
            windows => {
                // If AI can drop some codes like this logic,
                //  malware do not need anymore :)
                //  just conect PC, and then drop that code remotely.

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
            // !TODO define buffer_result max size 1024000000 ~ +5000000)
            buffer_result = tokio::io(data_bus_stream)?;

            match buffer_result {
                Ok(data) => {
                    unwrapped_data= buffer_result;
                    break;
                }
                Err(e) => {
                    reutnr Err("can't get data bus stream".into());
                }
            }
        }

        if unwrapped_data.capcity() >= CAPACITY_LINE {
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
    pub async fn logging(&mut self, flag: bool, option: LoggingOptions) -> &mut Self {
        self.logging_flag = flag;
        self.option = option;

        // @TODO
        // If this program save a data as file automatically,
        // i can write my code more consistently(buffer clean, and then keep watching again).
        // But its not a malware. Just in educational purpose.

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

                let unwrapped_data = BufferedData::unwarp_data(self.buffered_data);
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
