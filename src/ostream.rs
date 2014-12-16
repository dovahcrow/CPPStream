#![allow(unused_must_use)]
use std::fmt::Show;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Shl;

pub struct BorrowedOStream<'a,W: 'a> where W: Writer {
    ostream:  Rc<RefCell<&'a mut W>>
}

pub struct OwnedOStream<W> where W: Writer {
    ostream: Rc<RefCell<W>>,
}

impl<'a,T,W> Shl<T,BorrowedOStream<'a,W>> for BorrowedOStream<'a,W> where W: Writer, T: Show {
    fn shl(&self, output: &T) -> BorrowedOStream<'a,W> {

        
        write!(self.ostream.borrow_mut(), "{}", output);
  
        BorrowedOStream {
            ostream: self.ostream.clone()
        }
        
    }
}

impl<W,T> Shl<T,OwnedOStream<W>> for OwnedOStream<W> where W: Writer, T: Show {
    fn shl(&self, output: &T) -> OwnedOStream<W> {
        write!(self.ostream.borrow_mut(), "{}", output);
        OwnedOStream {
            ostream: self.ostream.clone(),
        }
    }
}

pub trait AsOStream<W> {
    fn as_ostream(mut self) -> OwnedOStream<W>;
}

impl<T> AsOStream<T> for T where T: Writer {
    fn as_ostream(mut self) -> OwnedOStream<T> {
        OwnedOStream {
            ostream: Rc::new(RefCell::new(self))
        }
    }
}

pub trait ToOStream<'a,W> where W: Writer {
    fn to_ostream(&'a mut self) -> BorrowedOStream<'a,W>;
}

impl<'a,T> ToOStream<'a,T> for T where T: Writer {
    fn to_ostream(&'a mut self) -> BorrowedOStream<'a,T> {
        BorrowedOStream {
            ostream: Rc::new(RefCell::new(self))
        }
    }
}

#[allow(non_upper_case_globals)]
pub const endl: char = '\n';
    
#[test]
fn test_out() {
    let mut vec = Vec::with_capacity(100);
    {
        let vecout = vec.to_ostream();
        vecout << 1i << 2u << 3f64 << "good C++" << endl;
    }
    
    assert_eq!(vec, b"123good C++\n");
}

#[test]
fn test_stdout() {
    use std;
    let cout = std::io::stdout().as_ostream();
    cout << 1i << 2u << 3f64 << endl;
}

