#![cfg(not(test))]
extern crate cppStream;
use cppStream::*;
use std::io::{stdin,stdout};
use std::io::TcpStream;

fn main() {
    let mut out = Vec::with_capacity(1000);
    {
        let cout = out.to_ostream();  // borrow the out
        cout << 1is << 2f64 << "goodday!" << vec![1is,2is,3is,4is] << endl;
    }
    println!("{:?}", out);
    {
        let cout = stdout().as_ostream(); // move the out
        cout.clone() << 1is << 2f64 << "goodday!" << vec![1is,2is,3is,4is] << endl;
    }

    {
        let cout = cout();  // another way
        cout.clone() << "another!" << endl;
        
        let cin = stdin().as_istream();
        let mut d = String::new();
        let mut f = 0is;
        cout.clone() << "please input a string: ";
        cin.clone() >> &mut d;
        cout.clone() << "pleas input a int: ";
        cin.clone() >> &mut f;
        cout << "string is: `" << d << "`, and number is: `" << f << "`" << endl;
    }
    
    let io = iostream(TcpStream::connect("127.0.0.1:7077"));  // bi-direction pipe
    io.clone() << "1" << "2" << endl;
    let mut a = 0is;
    io.clone() >> &mut a;
    cout() << "recv from tcp! : " << a << endl; //directly use cout

}
