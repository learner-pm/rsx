use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub struct ProgressSpinner {
    spinner: ProgressBar,
}

impl ProgressSpinner {
    // 初始化一个新的进度条（spinner）
    pub fn new(message: &str, total_files: u64) -> Self {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::with_template("{spinner:.dim} {msg} [{elapsed_precise}]")
                .unwrap()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
        );
        spinner.set_message(message.to_string());
        spinner.set_length(total_files);
        spinner.enable_steady_tick(Duration::from_millis(100));

        Self { spinner }
    }

    // 更新进度条的消息
    pub fn update_message(&self, message: String) {
        self.spinner.set_message(message);
    }

    // 停止进度条并输出最终消息
    pub fn finish(&self, final_message: String) {
        self.spinner.finish_with_message(final_message);
    }
}
