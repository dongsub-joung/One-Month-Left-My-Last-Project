
struct TheWatcher{
    pid: i32,
    logging_flag: bool,
    output_path: &'static str,
    csv_option: bool,
    // target: process::
}

impl TheWatcher{
    pub fn new(pid: i32) -> Self{
        let logging_flag= true;
        let output_path= "";
        let csv_option= false;
        // let target= process::new()
         
        Self{ pid, logging_flag, output_path, csv_option }
    }

    fn setting_target(&mut self) -> &mut Self{
        // hook a target

    }
}