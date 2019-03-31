pub struct RpmoveFile {
    pub line: String,
}
impl RpmoveFile {
    pub fn default() -> RpmoveFile {
        RpmoveFile {
            line: String::new(),
        }
    }
}