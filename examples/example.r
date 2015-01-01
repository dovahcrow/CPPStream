#![cfg(not(test))]
extern crate cppStream;
use cppStream::ostream::{endl, ToOStream, AsOStream};
use cppStream::istream::{ToIStream, AsIStream};
use std::io::{stdin,stdout};

fn main() {
    let mut out = Vec::with_capacity(1000);
    {
        let cout = out.to_ostream();  // borrow the out
        cout << 1i << 2f64 << "goodday!" << vec![1i,2i,3i,4i] << endl;
    }
    println!("{}", out);
    
    let cout = stdout().as_ostream(); // move the out
    cout << 1i << 2f64 << "goodday!" << vec![1i,2i,3i,4i] << endl;

    let cin = stdin().as_istream();
    let mut d = String::new();
    let mut f = 0i;
    cin >> d >> f;
    cout << d << endl << f << endl;
}
