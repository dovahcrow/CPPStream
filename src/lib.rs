#![feature(slicing_syntax)]
#![allow(unused_mut)]
#![feature(old_orphan_check)]
#![feature(associated_types)]
pub use istream::{cin,istream,AsIStream,ToIStream};
pub use ostream::{cout,endl,ostream,ToOStream,AsOStream};
pub use iostream::{iostream,ToIOStream,RefStream,ByRefStream,AsIOStream};

pub mod istream;
pub mod ostream;
pub mod iostream;
#[test]
fn a_test() {
    struct A;
    let a = box A;
    let b = box *a;
    let _ = a;
    let _ = b;
}
