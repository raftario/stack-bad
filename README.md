# stack bad

stack bad is a brainfuck inspired, Turing complete language whose syntax can be boiled down to "stack bad"

It operates on a theoretically unbounded region of contiguous memory, representable as an array of unsigned 8 bit integers. It uses two pointers, one instruction pointer and one memory pointer, both initialised to 0. It uses a set of 7 instructions. Every input character which isn't an instruction is ignored.

- `s` - Decrement the value at the memory pointer, wrapping to 255 on underflow.
- `t` - Increment the memory pointer, which theoretically is unbounded and cannot overflow.
- `a` - Increment the value at the memory pointer, wrapping to 0 on overflow.
- `c` - If the value at memory address `0` is 0, write the value at the memory pointer to output. If the value at memory address `0` is 1, read the value at the memory pointer from output (0 for EOF). Otherwise, do nothing.
- `k` - Decrement the memory pointer, saturating at 0.
- `b` - Loop start, used in conjunction with loop end.
- `d` - Loop end. If value at the memory pointer is 0, continue execution. Otherwise, move the instruction pointer back to the matching loop start.

## Usage

This repository contains the code for `sbi`, a very simple stack bad interpreter. You can use it by running `cargo run -- <FILE>`.

## Examples

- [`stack_bad`](examples/stack_bad.sb) - Hello world examples which prints "stack bad"
- [`echo`](examples/echo.sb) - Pipes stdin to stdout
