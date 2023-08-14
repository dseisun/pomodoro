use std::time::Duration;

use indicatif::ProgressBar;

fn main() {
    let bar = ProgressBar::new(1000);
for _ in 0..1000 {
    bar.inc(1);
    std::thread::sleep(Duration::from_millis(100))
}
bar.finish();
}