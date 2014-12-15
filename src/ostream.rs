#![allow(unused_must_use)]
use std::fmt::Show;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Shl;

pub struct OStream<'a,W: 'a> {
    out_stream:  Rc<RefCell<&'a mut W>>
}

impl<'a,W> OStream<'a,W> {
    pub fn new(writer: &'a mut W) -> OStream<'a,W> {
        OStream {
            out_stream: Rc::new(RefCell::new(writer))
        }
    }
}

impl<'a,T,W> Shl<T,OStream<'a,W>> for OStream<'a,W> where W: Writer, T: Show {
    fn shl(&self, output: &T) -> OStream<'a,W> {

        
        write!(self.out_stream.borrow_mut(), "{}", output);
  
        OStream {
            out_stream: self.out_stream.clone()
        }
        
    }
}

#[test]
fn test_out() {
    let mut vec = Vec::with_capacity(100);
    {
        let cout = OStream::new(&mut vec);
        cout << 1i << 2u << 3f64 << "good C++" << endl;
    }
    
    assert_eq!(vec, b"123good C++\n");
}

#[test]
fn test_stdout() {
    let stream = &mut std::io::stdout();
    let cout = OStream::new(stream);
    cout << 1i << 2u << 3f64 << endl;
}

