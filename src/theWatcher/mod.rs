use std::{error::Error, io, process::Stdio};

pub enum LoggingOptions {
    ALL,
    KEYBOARD_ONLY,
    MOUSE_ONLY,
    NETWORK_ACTIVITY_MODE
}


struct TheWatcher{
    pid: i32,
    logging_flag: bool,
    output_path: &'static str,
    csv_option: bool,
    // target: process::Something ,
    // data_bus_steam: Someting,
}

impl TheWatcher{
    pub fn new(pid: i32, output_path: &'static str) -> Self{
        let logging_flag= true;
        let csv_option= false;
        // let target= process::new()
        // let data_bus_steam= something;

        Self{ pid, logging_flag, output_path, csv_option }
    }

    fn setting_target(&mut self) -> &mut Self{
        // hook a target
        // self.target= SomeTool::hook(pid);

        self
    }

    fn logging(&mut self, 
                flag: bool, 
                option: TheWatcher::LoggingOptions) 
                -> &mut Self{
                    
        // logging
        // @TODO

        self.logging_flag= flag;
        self.option= option;
        
        self
    }

    fn output_txt_path(&mut self) -> Result<&mut Self, Error>{
        if self.output_path == "" {
            self.output_path= "./"
        }

        // Keep writing data bus steam 
        // io::Write

        Ok(self)
    }
}