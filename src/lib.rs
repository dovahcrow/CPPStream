#![allow(unused_mut)]
#![feature(old_orphan_check)]
#![feature(box_syntax)]

pub use istream::{cin,istream,AsIStream,ToIStream};
pub use ostream::{cout,endl,ostream,ToOStream,AsOStream};
pub use iostream::{iostream,ToIOStream,RefStream,ByRefStream,AsIOStream};

pub mod istream;
pub mod ostream;
pub mod iostream;

