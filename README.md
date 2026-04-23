
# This project name is "The Watcher"

I will publish this crates, If I can do it in the end.
Dead end: 14/04/2026 started, after almost 2 month

```rust
use the_watcher::*;
use anyhow::Result;

mod the_watcher;

#[tokio::main]
async fn main() -> Result<()> {

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
        .logging(true, LoggingOptions::NETWORK_ACTIVITY_MODE)
        .await
        .output_txt_path()?
        .csv_format_option(true)?;
    
    watcher_a::run();

    Ok(())
}
```

# fn setting_target

step-by-step
- 1st: use PID
- 2nd: fillter as process name 

# fn logging

- true(Default)/false

## LoggingOptions
- NETWORK_ACTIVITY_MODE (Default)
- KEYBOARD_ONLY 
- MOUSE_ONLY
- ALL: keyboard, mouse, network resorce

### enum LoggingOption

<img width="901" height="641" alt="0303_14-04-26-enum" src="https://github.com/user-attachments/assets/a7883df5-e9c7-4599-a8ca-ac76d32e69c7" />

by Google's Gemini

# fn output_txt_path
- Default path: "./"


# csv_format_option(@TODO)
- true: save a log file as Comma-separated values
- false(Default)
