use std::str::{FromStr,from_str};
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Shr;
use std::mem::transmute;
use std::default::Default;
use std::borrow::BorrowFromMut;
use std::ops::DerefMut;

//pub trait BorrowFromMut<Sized? Owned> for Sized? {
//    fn borrow_from_mut(owned: &mut Owned) -> &mut Self;
//}

//trait CanRead<R> where R: Reader {
//    fn

pub struct IStream<'a,'b,R:'a,Q:'b,Sized? D:'b> where R: Reader, D: BorrowFromMut<Q>, &'b mut D: DerefMut<R> {
    istream: Rc<RefCell<Q>>
}

pub trait ToIStream<'a,'b,R,Q,D> where R: Reader, D: BorrowFromMut<Q>, &'b mut D: DerefMut<R> {
    fn to_istream(self) -> IStream<'a,'b,R,Q,D>;
}

impl<'a,'b,R,Q,D> ToIStream<'a,'b,R,Q,D> for Q where R: Reader, D: BorrowFromMut<Q>, &'b mut D: DerefMut<R> {
    fn to_istream(self) -> IStream<'a,'b,R,Q,D> {
        IStream {
            istream: Rc::new(RefCell::new(self))
        }
    }
}

impl<'a,'b,R,Q,D> Clone for IStream<'a,'b,R,Q,D> where R: Reader, D: BorrowFromMut<Q>, &'b mut D: DerefMut<R> {
    fn clone(&self) -> IStream<'a,'b,R,Q,D> {
        IStream {
            istream: self.istream.clone()
        }
    }
}
        
impl<'a,'b,'c,F,R,Q,D> Shr<&'b mut F,IStream<'a,'a,R,Q,D>> for IStream<'a,'a,R,Q,D> where R: Reader, F: FromStr + Default, D: BorrowFromMut<Q>, &'a mut D: DerefMut<R> {
    fn shr(mut self, output: &mut F) -> IStream<'a,'b,R,Q,D> {
        let tmp = &mut *self.istream.borrow_mut();
        let mut reader: &mut D = BorrowFromMut::borrow_from_mut(tmp);
        let mut real_reader: &DerefMut<R> = &reader;
        
        let mut buf = String::new(); // a string buffer
        
        loop {
            if let Ok(byte) = (*real_reader.deref_mut()).read_byte() {
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
    use std::io::{stdio,stdin};
    use super::istream::IStream;
    let mut cin = stdin();
    let vin: IStream<_,&mut stdio::StdinReader,stdio::StdinReader> = (&mut cin).to_istream();
    let mut d = 0u;
    for _ in range(0i,5) {
        vin.clone() >> &mut d;
    }

    
}

                
