

use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};

pub struct Progress {
    pub count: u64,
    pub bar: ProgressBar,
}

impl Progress {
    pub fn build(count: u64) -> Self {
        let bar = ProgressBar::with_draw_target(Some(count), ProgressDrawTarget::stderr());

        bar.set_style(
            ProgressStyle::with_template(
                "{elapsed_precise:.white.dim} {wide_bar:.magenta} {bytes}/{total_bytes} ({bytes_per_sec}, {eta})",
            )
            .unwrap()
            .progress_chars("=-/|\\_"),
        );

        Self { count, bar }
    }

    pub fn finish(&self) {
        self.bar.finish();
    }

    pub fn upgrade(&self, delta: u64) {
        self.bar.inc(delta);
    }
}

impl Drop for Progress {
    fn drop(&mut self) {
        self.bar.finish();
    }
}