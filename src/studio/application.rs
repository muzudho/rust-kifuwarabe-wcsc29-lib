use chrono::Utc;
use conf::kifuwarabe_wcsc29_app_config::KifuwarabeWcsc29AppConfig;
use conf::kifuwarabe_wcsc29_master_config::KifuwarabeWcsc29MasterConfig;
use std::path::PathBuf;
use studio::communication::Communication;

pub struct Application {
    // Logging.
    pub comm: Communication,
    // Config.
    pub my_conf: KifuwarabeWcsc29AppConfig,
    pub kw29_conf: KifuwarabeWcsc29MasterConfig,
    // デバッグ出力のフラグ。usiの邪魔になる出力がいっぱい出るぜ☆（*＾～＾*）
    pub kifuwarabe_flag: bool,
}
impl Application {
    pub fn new() -> Self {
        let my_config = KifuwarabeWcsc29AppConfig::load();
        let kw29_config = KifuwarabeWcsc29MasterConfig::load(&my_config);

        // logger, logging, log file.
        let mut path = PathBuf::from(&my_config.logging.directory);
        path.push(&format!(
            "{}-{}{}",
            &my_config.logging.file_base_name,
            Utc::now().format("%Y%m%d-%H%M(%S)").to_string(),
            &my_config.logging.file_extension
        ));
        let path_name = &path.to_str().unwrap_or_else(|| panic!("Fail. path_name.")); // ログ取れない。無限ループ防止で簡素なpanic。
                                                                                      // print!("#Log path_name: '{}'.", path_name);

        // 最初は、デバッグ・フラグをＯｎにして開始☆（＾～＾）標準出力はせず、ログには出す☆（＾～＾）
        Application {
            comm: Communication::from_file(path_name),
            my_conf: my_config,
            kw29_conf: kw29_config,
            kifuwarabe_flag: true,
        }
    }

    pub fn is_debug(&self) -> bool {
        self.kifuwarabe_flag
    }
}
impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
