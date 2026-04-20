use anyhow::Result;

use std::fs::File;
use std::io::prelude::*;

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
    // data_bus_steam: Someting,
    buffered_data: BufferedData,
}

struct BufferedData{
    // date: Date
    data: Vec<usize>,
    // sender_email: &' static str
}
impl BufferedData{
    pub fn new() -> Self{
        let data: Vec<usize>= Vec::new();
        Self { data }
    }
    pub fn from(&mut self, v_data: Vec<usize>) -> &mut Self{
        self.data= v_data;
        self
    }
    pub fn unwrap_data(self)-> Vec<usize>{
        self.data
    }
}

impl TheWatcher {
    pub fn new(pid: i32, output_path: &'static str) -> Self {
        let logging_flag = true;
        let csv_option = false;
        let option = LoggingOptions::ALL;
        // let target= process::new()
        // let data_bus_steam= something;
        
        let buffered_data= BufferedData::new();

        Self {
            pid,
            logging_flag,
            output_path,
            csv_option,
            option,
            buffered_data,
        } // target, data_bus_steam
    }

    pub fn setting_target(&mut self) -> &mut Self {
        // let target= process::new()

        // hook a target
        // self.target= SomeTool::hook(pid);
        // self.data_bus_steam= something;

        self
    }

    async fn read_data_steam(data_bus_steam: steam) -> Result<Box<Vec<usize>>>{
        let mut unwrapped_data: Vec<usize>= Vec::new();

        // !TODO if returned err, try to reconn
        {
            let mut buffer_result: Vec<usize>= Vec::new();
            // !TODO define buffer_result max size 1024000000 ~ +5000000)
            buffer_result= io::read(data_bus_steam)?;

            match buffer_result{
                Ok(data) =>{
                    unwrapped_data= *(data.unwrap());
                },
                Err(e) =>{
                    panic!("can't get the data bus steam");
                }
            }
        }

        if (unwrapped_data.capacity) >= 1024000000{
            return Ok(Box::from(unwrapped_data));
        }

        Ok(Box::new(Vec::new()))
    }
    async pub fn logging(&mut self, flag: bool, option: LoggingOptions) -> &mut Self {
        self.logging_flag = flag;
        self.option = option;

        // !TODO 
        // If this program save a data as file automatically, 
        // i can write my code more consistently(buffer clean, and then watching again).
        // But its not a malware. Just in educational purpose.
       
        // logging
        let data= read_data_steam(self.data_bus_steam); 
        // !TODO unwrap data
        self.buffered_data= BufferedData::from(self.buffered_data, data);

        self
    }

    pub fn output_txt_path(&mut self) -> Result<&mut Self> {
        let path= &self.output_path;
        let full_path= format!("{}/output.txt", path);

        let mut file= File::create(full_path)?;
        
        file.write_all(BufferedData::unwrap_data(self.buffered_Data)
            .to_string()
            .as_bytes()
        )?;

        Ok(self)
    }

    pub fn csv_format_option(&mut self, flag: bool) -> Result<&mut Self> {
        Ok(self)
    }
}
