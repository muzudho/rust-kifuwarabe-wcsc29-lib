use communication::Communication;
use conf::kifuwarabe_wcsc29_config::KifuwarabeWcsc29Config;
use conf::kifuwarabe_wcsc29_lib_config::KifuwarabeWcsc29LibConfig;

pub struct Application {
    // Logging.
    pub comm: Communication,
    // Config.
    pub my_conf: KifuwarabeWcsc29LibConfig,
    pub kw29_conf: KifuwarabeWcsc29Config,
}
impl Application {
    pub fn new() -> Self {
        let my_config = KifuwarabeWcsc29LibConfig::load();
        let kw29_config = KifuwarabeWcsc29Config::load(&my_config);
        Application {
            comm: Communication::new(),
            my_conf: my_config,
            kw29_conf: kw29_config,
        }
    }
}
