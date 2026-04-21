pub mod the_watcher;

#[cfg(test)]
mod tests {
    use crate::the_watcher::TheWatcher;

    use super::*;
    use the_watcher;

    #[test]
    fn get_process_name() {
        let mut result;
        unsafe {
            result = TheWatcher::get_name_process(12);
        }
        let mut result_string;
        assert_type_eq!(result, windows::core::Result<String>);
    }
}
