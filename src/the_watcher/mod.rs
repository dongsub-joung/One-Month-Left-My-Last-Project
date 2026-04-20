use anyhow::Result;

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
}

impl TheWatcher {
    pub fn new(pid: i32, output_path: &'static str) -> Self {
        let logging_flag = true;
        let csv_option = false;
        let option = LoggingOptions::ALL;
        // let target= process::new()
        // let data_bus_steam= something;

        Self {
            pid,
            logging_flag,
            output_path,
            csv_option,
            option,
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
        let mut buffer: Vec<usize>= Vec::new();
       
        // @TODO if returned err, try to reconn
        buffer= io::read(data_bus_steam)?;

        if (buffer.capacity) >= 1024000000{
            return Ok(Box::from(buffer));
        }

        Ok(Box::new(Vec::new()))
    }
    async pub fn logging(&mut self, flag: bool, option: LoggingOptions) -> &mut Self {
        self.logging_flag = flag;
        self.option = option;

        
        // logging
        let data= read_data_steam(self.data_bus_steam);
         

        self
    }

    pub fn output_txt_path(&mut self) -> Result<&mut Self> {
        if self.output_path == "" {
            self.output_path = "./"
        }



        // Keep writing data bus steam
        // io::Write
        Ok(self)
    }

    pub fn csv_format_option(&mut self, flag: bool) -> Result<&mut Self> {
        Ok(self)
    }
}
