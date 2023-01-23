use indicatif::ProgressBar;
use std::{thread, time};

fn main() {
  let bar = ProgressBar::new(100);
  for _ in 0..1000 {
      bar.inc(1);

      let five_hundred_millis = time::Duration::from_millis(500);

      thread::sleep(five_hundred_millis);
  }
  bar.finish();
}