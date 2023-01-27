use indicatif::ProgressBar;
use std::{thread, time};

fn main() {
  let total = 100;
  let bar = ProgressBar::new(total);
  for _ in 0..total {
      bar.inc(1);

      let five_hundred_millis = time::Duration::from_millis(500);

      thread::sleep(five_hundred_millis);
  }
  bar.finish();
}
