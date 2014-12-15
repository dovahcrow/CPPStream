extern crate cppStream;
use cppStream::endl;
use cppStream::ostream::OStream;
use std::io::stdout;

fn main() {
    let mut out = stdout();
    let cout = OStream::new(&mut out);
    cout << 1i << 2f64 << "goodday!" << vec![1i,2i,3i,4i] << endl;
}
