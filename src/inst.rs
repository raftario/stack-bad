#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Inst {
    Left(usize),
    Right(usize),
    Add(u8),
    Sub(u8),
    Start,
    End,
    IO,
}
