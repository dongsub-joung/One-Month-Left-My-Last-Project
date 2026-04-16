# This project name is "The Watcher"

I will publish this crates, If I can do it in the end.

```rust
use theWatcher::TheWatcher;

fn main(){
  let watcherA= TheWatcher::new();
  let _= watcherA.setting_target("dota2.exe")
                 .logging(true, TheWatcher::LoggingOptions::ALL)
                 .output_path("./log.txt")
                 .format_option(true);
}
```

# LoggingOptions
- ALL: keyboard, mouse, network resorce

# formatOption
- true: save a log file as Comma-separated values
