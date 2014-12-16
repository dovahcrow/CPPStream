use std::str::{FromStr,from_str};
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Shr;
use std::mem::transmute;
use std::default::Default;

pub struct BorrowedIStream<'a,R: 'a> where R: Reader {
    istream: Rc<RefCell<&'a mut R>>
}

pub struct OwnedIStream<R> where R: Reader {
    istream: Rc<RefCell<R>>
}

impl<'a,F,R> Shr<F,BorrowedIStream<'a,R>> for BorrowedIStream<'a,R> where R: Reader, F: FromStr + Default {
    fn shr(&self, output: &F) -> BorrowedIStream<'a,R> {
        
        let uout: &mut F = unsafe {transmute(output)}; // use unsafe here hence the shr trait use immutable right operand, need fix.
        
        let mut reader = self.istream.borrow_mut();
        
        let mut buf = String::new(); // a string buffer
        
        loop {
            if let Ok(byte) = reader.read_byte() {
                if byte == '\u{A}' as u8 || byte == '\u{20}' as u8 {
                    break
                } else {
                    buf.push(byte as char);
                }
            } else {
                break
            }
        }
        *uout = FromStr::from_str(buf[]).unwrap_or_default();
        BorrowedIStream {
            istream: self.istream.clone()
        }
    }
}

impl<F,R> Shr<F,OwnedIStream<R>> for OwnedIStream<R> where R: Reader, F: FromStr + Default {
    fn shr(&self, output: &F) -> OwnedIStream<R> {
        
        let uout: &mut F = unsafe {transmute(output)}; // use unsafe here hence the shr trait use immutable right operand, need fix.
        
        let mut reader = self.istream.borrow_mut();
        
        let mut buf = String::new(); // a string buffer
        
        loop {
            if let Ok(byte) = reader.read_byte() {
                if byte == '\u{A}' as u8 || byte == '\u{20}' as u8 {
                    break
                } else {
                    buf.push(byte as char);
                }
            } else {
                break
            }
        }
        *uout = FromStr::from_str(buf[]).unwrap_or_default();
        OwnedIStream {
            istream: self.istream.clone()
        }
    }
}

pub trait ToIStream<'a,R> where R: Reader {
    fn to_istream(&'a mut self) -> BorrowedIStream<'a,R>;
}

impl<'a,R> ToIStream<'a,R> for R where R: Reader {
    fn to_istream(&'a mut self) -> BorrowedIStream<'a,R> {
        BorrowedIStream {
            istream: Rc::new(RefCell::new(self))
        }
    }
}

pub trait AsIStream<R> where R: Reader {
    fn as_istream(mut self) -> OwnedIStream<R>;
}

impl<R> AsIStream<R> for R where R: Reader {
    fn as_istream(mut self) -> OwnedIStream<R> {
        OwnedIStream {
            istream: Rc::new(RefCell::new(self))
        }
    }
}

#[test]
fn test_buf() {
    use std::io::stdout;
    use super::ostream::{AsOStream, endl};
    
    let mut b = b"1 2 3 4 5";
    let cin = b.to_istream();
    let d = 0u;
    for _ in range(0i,5) {
        cin >> d;
        stdout().as_ostream() << d << endl;
    }
    
}

                
