use std::str::{FromStr};
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Shr;
use std::default::Default;
use std::io::{RefReader,ByRefReader};
use std::io::stdio::StdinReader;
use std::io::stdin;

pub struct IStream<R> where R: Reader {
    istream: Rc<RefCell<R>>
}

impl<R> IStream<R> where R: Reader {
    pub fn new(reader: R) -> IStream<R> {
        IStream {
            istream: Rc::new(RefCell::new(reader))
        }
    }
}

pub trait ToIStream<'a> where Self: ByRefReader + Reader + Sized {
    fn to_istream(&'a mut self) -> IStream<RefReader<'a,Self>> {
        IStream::new(self.by_ref())
    }
}

impl<'a,B> ToIStream<'a> for B where B: ByRefReader + Reader + Sized {}

pub trait AsIStream where Self: Reader + Sized {
    fn as_istream(self) -> IStream<Self> {
        IStream::new(self)
    }
}

impl<R> AsIStream for R where R: Reader + Sized {}

impl<R> Clone for IStream<R> where R: Reader {
    fn clone(&self) -> IStream<R> {
        IStream {
            istream: self.istream.clone()
        }
    }
}
        
impl<'b,F,R> Shr<&'b mut F> for IStream<R> where R: Reader, F: FromStr + Default {
    type Output = IStream<R>;
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
        
        *output = FromStr::from_str(&buf[]).unwrap_or_default();
        IStream {
            istream: self.istream.clone()
        }
    }
}

pub fn istream<R>(reader: R) -> IStream<R> where R: Reader {
    IStream::new(reader)
}

pub fn cin() -> IStream<StdinReader> {
    IStream::new(stdin())
}

#[test]
fn test_buf() {
    use std::io::stdout;
    use super::ostream::{AsOStream,endl};
    use super::istream::{AsIStream};
    
    let vin = b"1 2 3 4".as_istream();
    let cout = stdout().as_ostream();
    
    for _ in range(0is,5) {
        let mut d = 0is;
        vin.clone() >> &mut d;
        cout.clone() << d << endl;
    }

    
}

                
