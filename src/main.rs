use deku_redo::{NewDekuRead, Test};
use deku::DekuRead;
use bitvec::prelude::Msb0;
use bitvec::view::BitView;

fn main() {
    let bytes = [0x00, 0x03];
    let b = bytes.view_bits::<Msb0>();
    let test = Test::read_new(b, ());
    println!("{test:?}");

    let bytes = [0x00, 0x03];
    let b = bytes.view_bits::<Msb0>();
    let test = Test::read(b, ());
    println!("{test:?}");
}
