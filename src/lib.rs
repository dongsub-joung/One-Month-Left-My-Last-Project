pub mod the_watcher;

#[cfg(test)]
mod tests {
    use anyhow::Error;

    use crate::the_watcher::TheWatcher;
    use super::*;

    //  assertion `left == right` failed
    //  left: "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Grisaia Chronos Rebellion\\GCR.exe"
    //  right: "value"
    // #[test]
    // fn test_get_process_name() {
    //     unsafe {
    //         let _result = TheWatcher::get_name_process(24264);
    //         match _result {
    //             Ok(_string) =>{
    //                 assert_eq!(_string, String::from("value"));
    //             },
    //             Err(e) =>{
    //                 eprint!("{}", e);
    //             }
    //         }
    //     }
    // }

    // //  assertion `left == right` failed
    // //  left: "グリザイア クロノスリベリオン"
    // //  right: "value"
    // #[test]
    // fn test_get_title() {
    //     let mut _result= String::new();
    //     unsafe {
    //         _result= TheWatcher::get_current_windows_title_name();
    //     }
    //     assert_eq!(_result, String::from("value"));
    // }
    
}
