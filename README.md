
# This project name is "The Watcher"

I will publish this crates, If I can do it in the end.
Dead end: 14/04/2026 started, after almost 2 month

```rust
use theWatcher::TheWatcher;

fn main(){
  let watcherA= TheWatcher::new()
                .setting_target("notepad.exe")
                .logging(true, TheWatcher::LoggingOptions::ALL)
                .output_path("./log.txt")
                .format_option(true);
}
```

# LoggingOptions
- ALL: keyboard, mouse, network resorce
- KEYBOARD_ONLY 
- MOUSE_ONLY
- NETWORK_ACTIVITY_MODE

## enum LoggingOption

<img width="901" height="641" alt="0303_14-04-26-enum" src="https://github.com/user-attachments/assets/a7883df5-e9c7-4599-a8ca-ac76d32e69c7" />
by Google's Gemini

# formatOption
- true: save a log file as Comma-separated values
