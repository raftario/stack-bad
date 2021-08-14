use std::iter::Peekable;

use num_traits::Unsigned;

use crate::inst::Inst;

pub fn parse(src: &str) -> Vec<Inst> {
    let mut iter = src.as_bytes().iter().copied().peekable();
    let mut insts = Vec::new();
    while let Some(b) = iter.next() {
        insts.push(match b {
            b's' => Inst::Sub(take::<u8, _>(b's', &mut iter) + 1),
            b't' => Inst::Right(take::<usize, _>(b't', &mut iter) + 1),
            b'a' => Inst::Add(take::<u8, _>(b's', &mut iter) + 1),
            b'c' => Inst::IO,
            b'k' => Inst::Left(take::<usize, _>(b't', &mut iter) + 1),
            b'b' => Inst::Start,
            b'd' => Inst::End,
            _ => continue,
        });
    }
    insts
}

fn take<T, I>(inst: u8, iter: &mut Peekable<I>) -> T
where
    T: Unsigned,
    I: Iterator<Item = u8>,
{
    let mut count = T::zero();
    while let Some(b) = iter.peek().copied() {
        if b == inst {
            count = count + T::one();
            iter.next();
        } else {
            break;
        }
    }
    count
}
