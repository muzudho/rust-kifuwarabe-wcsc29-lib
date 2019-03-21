use address::*;
use board::*;

/// Vector に入れるときコピーする。
#[derive(Clone, Copy, PartialEq)]
pub struct PhysicalMove {
    pub address: Option<Address>,
    pub sky_turn: bool,
    pub sky_rotate: bool,
}
impl PhysicalMove {
    pub fn create_by_address(address:Address) -> PhysicalMove {
        PhysicalMove {
            address: Some(address),
            sky_turn: false,
            sky_rotate: false,
        }
    }

    pub fn turn_over() -> PhysicalMove {
        PhysicalMove {
            address: None,
            sky_turn: true,
            sky_rotate: false,
        }
    }

    pub fn rotate() -> PhysicalMove {
        PhysicalMove {
            address: None,
            sky_turn: false,
            sky_rotate: true,
        }
    }

    pub fn print(&self, board_size:&BoardSize) -> String {
        match self.address {
            Some(address) => {
                address.print(board_size)
            },
            None => {
                if self.sky_turn {
                    "+".to_string()
                } else if self.sky_rotate {
                    "-".to_string()
                } else {
                    panic!("Unexpected physical move print.")
                }
            },
        }
    }
}
