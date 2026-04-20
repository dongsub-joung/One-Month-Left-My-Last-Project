
# This project name is "The Watcher"

I will publish this crates, If I can do it in the end.
Dead end: 14/04/2026 started, after almost 2 month

```rust
use the_watcher::*;
use anyhow::Result;

mod the_watcher;

#[tokio::main]
async fn main() -> Result<()> {

    let mut watcher_a = TheWatcher::new(pid, output_path);
    watcher_a
        .setting_target()
        .logging(true, LoggingOptions::ALL)
        .await
        .output_txt_path()?
        .csv_format_option(true)?;

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
- ALL: keyboard, mouse, network resorce
- KEYBOARD_ONLY 
- MOUSE_ONLY
- NETWORK_ACTIVITY_MODE

### enum LoggingOption

<img width="901" height="641" alt="0303_14-04-26-enum" src="https://github.com/user-attachments/assets/a7883df5-e9c7-4599-a8ca-ac76d32e69c7" />

by Google's Gemini

# fn output_txt_path
- Default path: "./"


# csv_format_option(@TODO)
- true: save a log file as Comma-separated values
- false(Default)
