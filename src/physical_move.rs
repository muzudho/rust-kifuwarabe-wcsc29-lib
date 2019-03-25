use address::*;
use position::*;

/// Vector に入れるときコピーする。
#[derive(Clone, Copy, PartialEq)]
pub struct PhysicalMove {
    pub address: Option<Address>,
    pub sky_turn: bool,
    pub sky_rotate: bool,
    pub phase_change: bool,
}
impl PhysicalMove {
    pub fn create_by_address(address:Address) -> PhysicalMove {
        PhysicalMove {
            address: Some(address),
            sky_turn: false,
            sky_rotate: false,
            phase_change: false,
        }
    }

    pub fn turn_over() -> PhysicalMove {
        PhysicalMove {
            address: None,
            sky_turn: true,
            sky_rotate: false,
            phase_change: false,
        }
    }

    pub fn rotate() -> PhysicalMove {
        PhysicalMove {
            address: None,
            sky_turn: false,
            sky_rotate: true,
            phase_change: false,
        }
    }

    pub fn change_phase() -> PhysicalMove {
        PhysicalMove {
            address: None,
            sky_turn: false,
            sky_rotate: false,
            phase_change: true,
        }
    }

    pub fn to_sign(&self, board_size:BoardSize, ply:&mut i16) -> String {
        match self.address {
            Some(address) => {
                address.to_physical_sign(board_size)
            },
            None => {
                if self.sky_turn {
                    "+".to_string()
                } else if self.sky_rotate {
                    "-".to_string()
                } else if self.phase_change {
                    // TODO 手数が出てきた方が嬉しいので [2] といった数で挟みたい。
                    *ply += 1;
                    format!("[{}]", ply)
                    // "|".to_string()
                } else {
                    panic!("Unexpected physical move print.")
                }
            },
        }
    }
}
