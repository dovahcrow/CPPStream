#![feature(default_type_params)]
#![feature(slicing_syntax)]
#![feature(associated_types)]
#![allow(unused_mut)]
pub use istream::{AsIStream,ToIStream};
pub use ostream::{endl,ToOStream,AsOStream};
pub mod istream;
pub mod ostream;

