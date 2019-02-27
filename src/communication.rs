pub enum CommunicationType {
    Usi,
    File,
}
pub struct Communication {
    pub way: CommunicationType
}
impl Communication {
    pub fn new () -> Communication {
        Communication {
            way: CommunicationType::Usi,
        }
    }
    /// Write line.
    pub fn println(&self, line:&str) {
        match (*self).way {
            CommunicationType::Usi => {
                println!("{}", line);
            },
            CommunicationType::File => {
                println!("{}", line);
            },
        }
    }
}
