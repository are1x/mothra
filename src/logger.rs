// src/logger.rs
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub struct LoggerConfig {
    /// "rendering" ターゲットのログレベル
    pub rendering_level: LevelFilter,
    /// デフォルトのログレベル
    pub default_level: LevelFilter,
    /// ログをファイルに出力する場合のファイルパス（None なら標準出力のみ）
    pub file_output: Option<String>,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            rendering_level: LevelFilter::Debug,
            default_level: LevelFilter::Warn,
            file_output: None,
        }
    }
}

/// LoggerConfig を用いたロガーの初期化
pub fn init_logger_with_config(config: LoggerConfig) {
    let mut builder = Builder::new();
    builder
        .filter(Some("rendering"), config.rendering_level)
        .filter(None, config.default_level);
    if let Some(file_path) = config.file_output {
        // ファイル出力のために、Log::builder() に書き込み先を設定する
        let file = std::fs::File::create(file_path).expect("Unable to create log file");
        builder.target(env_logger::Target::Pipe(Box::new(file)));
    }
    builder.init();
}
