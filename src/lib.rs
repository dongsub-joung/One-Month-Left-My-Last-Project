pub mod the_watcher;

#[cfg(test)]
mod tests {
    use anyhow::Error;

    use crate::the_watcher::TheWatcher;
    use super::*;

//  thread 'tests::test_get_process_name' (15388) panicked at src\lib.rs:16:21:
//  assertion `left == right` failed
//  left: "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Grisaia Chronos Rebellion\\GCR.exe"
//  right: "value"
//  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//  test tests::test_get_process_name ... FAILED
    #[test]
    fn test_get_process_name() {
        unsafe {
            let _result = TheWatcher::get_name_process(24264);
            match _result {
                Ok(_string) =>{
                    assert_eq!(_string, String::from("value"));
                },
                Err(e) =>{
                    eprint!("{}", e);
                }
            }
        }
    }
}
