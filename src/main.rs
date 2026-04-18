use anyhow::Result;

mod the_watcher;
// mod the_Watcher;

use the_watcher::*;

fn main() -> Result<()>{
  let pid= 111;
  let output_path= "./dir";

  let mut watcher_a= TheWatcher::new(pid, output_path);
  watcher_a.setting_target()
          .logging(true, LoggingOptions::ALL)
          .output_txt_path()?
          .csv_format_option(true)?;

  Ok(())
}