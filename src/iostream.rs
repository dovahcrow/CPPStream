#![allow(unused_must_use)]
use std::fmt::Show;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::{Shl,Shr};
use std::io::Stream;
use std::io::{Writer,Reader};
use std::io::IoResult;
use std::str::{FromStr,from_str};
use std::default::Default;

pub struct RefStream<'a,S:'a> where S: Stream {
    inner: &'a mut S
}

impl<'a, S: Stream> Reader for RefStream<'a, S> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> { self.inner.read(buf) }
}

impl<'a, S: Stream> Writer for RefStream<'a, S> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> IoResult<()> { self.inner.write(buf) }

    #[inline]
    fn flush(&mut self) -> IoResult<()> { self.inner.flush() }
}

pub trait ByRefStream where Self: Stream {
    #[inline]
    fn by_ref<'a>(&'a mut self) -> RefStream<'a, Self>;
}

impl<S> ByRefStream for S where S: Stream {
    fn by_ref<'a>(&'a mut self) -> RefStream<'a,S> {
        RefStream {
            inner: self
        }
    }
}


pub struct IOStream<S> where S: Stream {
    iostream: Rc<RefCell<S>>,
}

impl<S> IOStream<S> where S: Stream {
    pub fn new(stream: S) -> IOStream<S> {
        IOStream {
            iostream: Rc::new(RefCell::new(stream))
        }
    }
}

pub trait ToIOStream<'a> where Self: ByRefStream + Stream {
    fn to_iostream(&'a mut self) -> IOStream<RefStream<'a,Self>>;
}

impl<'a,B> ToIOStream<'a> for B where B: ByRefStream + Stream {
    fn to_iostream(&'a mut self) -> IOStream<RefStream<'a,B>> {
        let a = self.by_ref();
        IOStream::new(a)
    }
}

pub trait AsIOStream where Self: Stream {
    fn as_iostream(self) -> IOStream<Self> {
        let a = self;
        IOStream::new(a)
    }
}

impl<S> AsIOStream for S where S: Stream {}

impl<S,T> Shl<T,IOStream<S>> for IOStream<S> where S: Stream, T: Show {
    fn shl(self, output: T) -> IOStream<S> {
        {
            let mut writer = self.iostream.borrow_mut();
            write!(writer, "{}", output);
            writer.flush();
        }
        self.clone()
    }
}

impl<'b,F,S> Shr<&'b mut F,IOStream<S>> for IOStream<S> where S: Stream, F: FromStr + Default {
    fn shr(mut self, output: &mut F) -> IOStream<S> {
        
        let mut buf = String::new(); // a string buffer
        
        loop {
            if let Ok(byte) = self.iostream.borrow_mut().read_byte() {
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
        self.clone()
    }
}

impl<S> Clone for IOStream<S> where S: Stream {
    fn clone(&self) -> IOStream<S> {
        IOStream {
            iostream: self.iostream.clone()
        }
    }
}

pub fn iostream<S>(stream: S) -> IOStream<S> where S: Stream {
    IOStream::new(stream)
}

#[test]
fn test_iostream() {
    use super::{endl,cout};
    use std::io::TcpStream;
    
    let io = iostream(TcpStream::connect("127.0.0.1:7077"));
    io.clone() << "1" << "2" << endl;
    let mut a = 0i;
    io.clone() >> &mut a;
    cout() << a << endl;
}


