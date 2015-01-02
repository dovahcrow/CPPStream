#![feature(slicing_syntax)]
#![allow(unused_mut)]
pub use istream::{cin,istream,AsIStream,ToIStream};
pub use ostream::{cout,endl,ostream,ToOStream,AsOStream};
pub use iostream::{iostream,ToIOStream,RefStream,ByRefStream,AsIOStream};

pub mod istream;
pub mod ostream;
pub mod iostream;
