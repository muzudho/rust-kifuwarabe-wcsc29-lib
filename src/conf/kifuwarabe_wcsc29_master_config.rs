use conf::kifuwarabe_wcsc29_exe_config::*;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use studio::application::Application;

/// 統一設定ファイル。
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct KifuwarabeWcsc29MasterConfig {
    pub kifuwarabe_wcsc29_opt: String,
    pub converter_var_lib: String,

    pub training: String,
    pub learning: String,
    pub book: String,
    pub tapes_fragments: String,
}
impl KifuwarabeWcsc29MasterConfig {
    /// 設定ファイル読込。
    pub fn load(my_app_conf: &KifuwarabeWcsc29ExeConfig) -> KifuwarabeWcsc29MasterConfig {
        let kw29_path = &my_app_conf.kifuwarabe_wcsc29_master_config_path;
        let mut kw29_file = match File::open(kw29_path) {
            Ok(x) => x,
            Err(err) => panic!("File open error. {:?}", err), // ログ取らない。
        };

        let mut contents = String::new();
        match kw29_file.read_to_string(&mut contents) {
            Ok(x) => x,
            Err(err) => panic!("File open error. {:?}", err), // ログ取らない。
        };

        match serde_json::from_str(&contents) {
            Ok(x) => x,
            Err(err) => panic!("Unexpected wcsc29 config: {}", err), // ログ取らない。
        }
    }

    /// 実行ファイルへのパス。
    pub fn get_kifuwarabe_wcsc29_exe(&self, app: &Application) -> String {
        Path::new(&self.kifuwarabe_wcsc29_opt)
            .join("kifuwarabe-wcsc29.exe")
            .to_str()
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. get_kifuwarabe_wcsc29_exe.")))
            .to_string()
    }

    /// 棋譜ファイルを置くディレクトリーへのパス。
    pub fn get_input_directory_of_converter(&self, app: &Application) -> String {
        Path::new(&self.converter_var_lib)
            .join("input")
            .to_str()
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. get_input_directory_of_converter.")))
            .to_string()
    }

    /// 展開したファイルを置くディレクトリーへのパス。
    pub fn get_expanded_directory_of_converter(&self, app: &Application) -> String {
        Path::new(&self.converter_var_lib)
            .join("expanded")
            .to_str()
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. get_expanded_directory_of_converter.")))
            .to_string()
    }

    /// エンコードしたファイルを置くディレクトリーへのパス。
    pub fn get_encoded_directory_of_converter(&self, app: &Application) -> String {
        Path::new(&self.converter_var_lib)
            .join("encoded")
            .to_str()
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. get_encoded_directory_of_converter.")))
            .to_string()
    }

    /// エラーしたファイルを置くディレクトリーへのパス。
    pub fn get_error_directory_of_converter(&self, app: &Application) -> String {
        Path::new(&self.converter_var_lib)
            .join("error")
            .to_str()
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. get_error_directory_of_converter.")))
            .to_string()
    }

    /// 棋譜変換したファイルを置くディレクトリーへのパス。
    pub fn get_converted_directory_of_converter(&self, app: &Application) -> String {
        Path::new(&self.converter_var_lib)
            .join("converted")
            .to_str()
            .unwrap_or_else(|| {
                panic!(app
                    .comm
                    .panic("Fail. get_converted_directory_of_converter."))
            })
            .to_string()
    }

    /// １つのファイルに棋譜を詰め込んだファイルを置くディレクトリーへのパス。
    pub fn get_jammed_directory_of_converter(&self, app: &Application) -> String {
        Path::new(&self.converter_var_lib)
            .join("jammed")
            .to_str()
            .unwrap_or_else(|| panic!(app.comm.panic("Fail. get_jammed_directory_of_converter.")))
            .to_string()
    }
}
