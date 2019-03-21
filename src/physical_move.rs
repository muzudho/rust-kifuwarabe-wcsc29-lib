use address::*;

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

    pub fn create_turn() -> PhysicalMove {
        PhysicalMove {
            address: None,
            sky_turn: true,
            sky_rotate: false,
        }
    }
}
