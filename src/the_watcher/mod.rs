use std::{error::Error, io, process::Stdio};

#[allow(non_camel_case_types)]
pub enum LoggingOptions {
    ALL,
    KEYBOARD_ONLY,
    MOUSE_ONLY,
    NETWORK_ACTIVITY_MODE
}


pub struct TheWatcher{
    pub pid: i32,
    pub logging_flag: bool,
    pub output_path: &'static str,
    pub csv_option: bool,
    pub option: LoggingOptions,
}

impl TheWatcher{
    pub fn new(pid: i32, output_path: &'static str) -> Self{
        let logging_flag= true;
        let csv_option= false;
        let option = LoggingOptions::ALL;

        Self{ pid, logging_flag, output_path, csv_option, option }
    }

    pub fn setting_target(&mut self) -> &mut Self{
        self
    }

    pub fn logging(&mut self, 
                flag: bool, 
                option: LoggingOptions) 
                -> &mut Self{
                    
        self.logging_flag= flag;
        self.option= option;
        
        self
    }

    pub fn output_txt_path(&mut self) -> Result<&mut Self, Box<dyn Error>>{
        if self.output_path == "" {
            self.output_path= "./"
        }

        Ok(self)
    }
}