extern crate kifuwarabe_wcsc29_lib;

use kifuwarabe_wcsc29_lib::*;
use kifuwarabe_wcsc29_lib::learn::learning::*;

pub fn main() {

    let leaning = Learning::new();

    leaning.save();
}
