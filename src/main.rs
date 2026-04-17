// mod the_watcher;
// use the_watcher::TheWatcher;
mod the_Watcher;

fn main() {
    println!("Hello, world!");
    let _watcher = the_Watcher::TheWatcher::new(123, "./output.txt");
}