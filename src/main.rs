use the_watcher::*;

use anyhow::Result;

mod the_watcher;

#[tokio::main]
async fn main() -> Result<()> {
    let pid = 111;
    let output_path = "./dir";

    println!("--------------------------------------------------------------");
    println!("d888888b  .d8b.  d888888b .d8888. db    db db    db  .d8b.");
    println!("`~~88~~' d8' `8b `~~88~~' 88'  YP 88    88 `8b  d8' d8' `8b ");  
    println!("   88    88ooo88    88    `8bo.   88    88  `8bd8'  88ooo88 ");
    println!("   88    88~~~88    88      `Y8b. 88    88    88    88~~~88 ");
    println!("   88    88   88    88    db   8D 88b  d88    88    88   88 ");
    println!("   YP    YP   YP    YP    `8888Y' ~Y8888P'    YP    YP   YP ");
    println!("--------------------------------------------------------------");

    let mut watcher_a = TheWatcher::new(pid, output_path);

    watcher_a
        .setting_target()
        .logging(true, LoggingOptions::ALL)
        .await
        .output_txt_path(true)?
        .csv_format_option(true)?;

    // watcher_a.run_without_logging();

    Ok(())
}
