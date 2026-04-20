use the_watcher::*;

use anyhow::Result;

mod the_watcher;

#[tokio::main]
async fn main() -> Result<()> {
    let pid = 111;
    let output_path = "./dir";

    let mut watcher_a = TheWatcher::new(pid, output_path);
    watcher_a
        .setting_target()
        .logging(true, LoggingOptions::ALL)
        .await
        .output_txt_path()?
        .csv_format_option(true)?;

    Ok(())
}
