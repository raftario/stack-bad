use crate::{
    inst::Inst,
    io::{Stdio, IO},
    parser::parse,
};

pub struct Interpreter<T = Stdio> {
    io: T,
}

impl Interpreter {
    pub const fn new() -> Self {
        Self { io: Stdio }
    }
}

impl<T> Interpreter<T> {
    pub const fn with_io(io: T) -> Self {
        Self { io }
    }
}

impl<T> Interpreter<T>
where
    T: IO,
{
    pub fn run(&mut self, src: &str) {
        let insts = parse(src);
        let mut mem = Mem::new();

        let mut inst_ptr = Ptr::new();
        let mut mem_ptr = Ptr::new();

        while let Some(inst) = insts.get(inst_ptr.0).copied() {
            match inst {
                Inst::Left(n) => mem_ptr.sub(n),
                Inst::Right(n) => mem_ptr.add(n),
                Inst::Add(n) => {
                    let m = mem.get_mut(mem_ptr);
                    *m = m.wrapping_add(n);
                }
                Inst::Sub(n) => {
                    let m = mem.get_mut(mem_ptr);
                    *m = m.wrapping_sub(n);
                }
                Inst::Start => {}
                Inst::End => match mem.get(mem_ptr) {
                    0 => {}
                    _ => {
                        while insts.get(inst_ptr.0).copied().unwrap() != Inst::Start {
                            inst_ptr.dec();
                        }
                    }
                },
                Inst::IO => match mem.get(Ptr(0)) {
                    0 => self.io.write(mem.get(mem_ptr)),
                    1 => *mem.get_mut(mem_ptr) = self.io.read(),
                    _ => {}
                },
            }
            inst_ptr.inc();
        }
    }
}

#[derive(Debug)]
struct Mem(Vec<u8>);

impl Mem {
    const fn new() -> Self {
        Self(Vec::new())
    }

    fn get(&mut self, ptr: Ptr) -> u8 {
        self.ensure_len(ptr.0 + 1);
        self.0.get(ptr.0).copied().unwrap()
    }

    fn get_mut(&mut self, ptr: Ptr) -> &mut u8 {
        self.ensure_len(ptr.0 + 1);
        self.0.get_mut(ptr.0).unwrap()
    }

    fn ensure_len(&mut self, len: usize) {
        if len > self.0.len() {
            self.0.resize(len, 0);
        }
    }
}

#[derive(Clone, Copy)]
struct Ptr(usize);

impl Ptr {
    const fn new() -> Self {
        Self(0)
    }

    fn add(&mut self, n: usize) {
        self.0 = self.0.saturating_add(n);
    }

    fn sub(&mut self, n: usize) {
        self.0 = self.0.saturating_sub(n);
    }

    fn inc(&mut self) {
        self.add(1)
    }

    fn dec(&mut self) {
        self.sub(1)
    }
}
