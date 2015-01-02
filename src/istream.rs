use std::str::{FromStr,from_str};
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Shr;
use std::default::Default;
use std::io::{RefReader,ByRefReader};

pub struct IStream<R> where R: Reader {
    istream: Rc<RefCell<R>>
}

pub trait ToIStream<'a> where Self: ByRefReader + Reader {
    fn to_istream(&'a mut self) -> IStream<RefReader<'a,Self>> {
        IStream {
            istream: Rc::new(RefCell::new(self.by_ref()))
        }
    }
}

impl<'a,B> ToIStream<'a> for B where B: ByRefReader + Reader {}

pub trait AsIStream where Self: Reader {
    fn as_istream(self) -> IStream<Self> {
        IStream {
            istream: Rc::new(RefCell::new(self))
        }
    }
}

impl<R> AsIStream for R where R: Reader {}

impl<R> Clone for IStream<R> where R: Reader {
    fn clone(&self) -> IStream<R> {
        IStream {
            istream: self.istream.clone()
        }
    }
}
        
impl<'b,F,R> Shr<&'b mut F,IStream<R>> for IStream<R> where R: Reader, F: FromStr + Default {
    fn shr(mut self, output: &mut F) -> IStream<R> {
        
        let mut buf = String::new(); // a string buffer
        
        loop {
            if let Ok(byte) = self.istream.borrow_mut().read_byte() {
                if byte == '\u{A}' as u8 || byte == '\u{20}' as u8 {
                    break
                } else {
                    buf.push(byte as char);
                }
            } else {
                break
            }
        }
        
        *output = FromStr::from_str(buf[]).unwrap_or_default();
        IStream {
            istream: self.istream.clone()
        }
    }
}

#[test]
fn test_buf() {
    use std::io::stdout;
    use super::ostream::{AsOStream,endl};
    use super::istream::{AsIStream};
    
    let vin = b"1 2 3 4".as_istream();
    let cout = stdout().as_ostream();
    
    for _ in range(0i,5) {
        let mut d = 0i;
        vin.clone() >> &mut d;
        cout.clone() << d << endl;
    }

    
}

                
