#![feature(test)]

extern crate test;
extern crate bf;
extern crate brainfuck;

pub use test::Bencher;

static FACTOR_SRC: &'static str =
    include_str!("../bf-src/factor.bf");

static MANDELBROT_SRC: &'static str =
    include_str!("../bf-src/mandelbrot.bf");

static HELLO_SRC: &'static str =
    include_str!("../bf-src/hello-world.bf");

fn bf_interpret_run(program: &str, input: &[u8]) -> Vec<u8> {
    use bf::traits::Interpretable;

    let program = bf::ast::parse_program(program.as_bytes()).unwrap();
    let program = bf::rle::compile(&program);
    let program = bf::peephole::compile(&program);

    program.interpret_memory(None, input).unwrap()
}

#[cfg(feature = "bf-jit")]
fn bf_jit_run(program: &str, input: &[u8]) -> Vec<u8> {
    use bf::traits::Interpretable;

    let program = bf::ast::parse_program(program.as_bytes()).unwrap();
    let program = bf::rle::compile(&program);
    let program = bf::peephole::compile(&program);
    let program = bf::jit::compile(&program, true);

    program.interpret_memory(None, input).unwrap()
}

fn brainfuck_run(program: &str, input: &[u8]) -> Vec<u8> {
    use brainfuck::program::Program;
    use brainfuck::Interpreter;
    use brainfuck::tape::VecTape;

    let mut reader = input;
    let mut writer = Vec::new();
    let program = Program::parse(program).unwrap();
    {
        let mut interpreter = Interpreter::<VecTape>::new(program, &mut reader, &mut writer);
        interpreter.run().unwrap();
    }
    writer
}

#[cfg(test)]
mod it_works {
    use std::str;
    use super::*;

    const FACTOR_INPUT: &'static [u8] = b"1000000\n";
    const FACTOR_RESULT: &'static str = "1000000: 2 2 2 2 2 2 5 5 5 5 5 5\n";

    fn assert_factor(result: Vec<u8>) {
        assert_eq!(str::from_utf8(&result).unwrap(), FACTOR_RESULT);
    }

    #[test]
    fn bf_interpret_factor_1_000_000() {
        assert_factor(bf_interpret_run(FACTOR_SRC, FACTOR_INPUT));
    }

    #[test]
    fn bf_jit_factor_1_000_000() {
        assert_factor(bf_jit_run(FACTOR_SRC, FACTOR_INPUT));
    }

    #[test]
    fn brainfuck_factor_1_000_000() {
        assert_factor(brainfuck_run(FACTOR_SRC, FACTOR_INPUT));
    }

    const HELLO_RESULT: &'static str = "Hello, World!";

    fn assert_hello(result: Vec<u8>) {
        assert_eq!(str::from_utf8(&result).unwrap(), HELLO_RESULT);
    }

    #[test]
    fn bf_interpret_hello() {
        assert_hello(bf_interpret_run(HELLO_SRC, b""));
    }

    #[test]
    fn bf_jit_hello() {
        assert_hello(bf_jit_run(HELLO_SRC, b""));
    }

    #[test]
    fn brainfuck_hello() {
        assert_hello(brainfuck_run(HELLO_SRC, b""));
    }
}

#[cfg(test)]
mod empty_program_bench {
    use super::*;

    #[bench]
    fn bf_interpret(bench: &mut Bencher) {
        bench.iter(|| {
            bf_interpret_run("", b"")
        });
    }

    #[bench]
    fn bf_jit(bench: &mut Bencher) {
        bench.iter(|| {
            bf_jit_run("", b"")
        });
    }

    #[bench]
    fn brainfuck(bench: &mut Bencher) {
        bench.iter(|| {
            brainfuck_run("", b"")
        });
    }
}
#[cfg(test)]
mod hello_bench {
    use super::*;

    #[bench]
    fn bf_interpret(bench: &mut Bencher) {
        bench.iter(|| {
            bf_interpret_run(HELLO_SRC, b"")
        });
    }

    #[bench]
    fn bf_jit(bench: &mut Bencher) {
        bench.iter(|| {
            bf_jit_run(HELLO_SRC, b"")
        });
    }

    #[bench]
    fn brainfuck(bench: &mut Bencher) {
        bench.iter(|| {
            brainfuck_run(HELLO_SRC, b"")
        });
    }
}

#[cfg(test)]
mod factor_1_000_000 {
    use super::*;

    const FACTOR_INPUT: &'static [u8] = b"1000000\n";

    #[bench]
    fn bf_interpret(bench: &mut Bencher) {
        bench.iter(|| {
            bf_interpret_run(FACTOR_SRC, FACTOR_INPUT)
        });
    }

    #[bench]
    fn bf_jit(bench: &mut Bencher) {
        bench.iter(|| {
            bf_jit_run(FACTOR_SRC, FACTOR_INPUT)
        });
    }

    #[bench]
    fn brainfuck(bench: &mut Bencher) {
        bench.iter(|| {
            brainfuck_run(FACTOR_SRC, FACTOR_INPUT)
        });
    }
}

#[cfg(all(test, feature = "long"))]
mod factor_179_424_691 {
    use super::*;

    const FACTOR_INPUT: &'static [u8] = b"179424691\n";

    #[bench]
    fn bf_interpret(bench: &mut Bencher) {
        bench.iter(|| {
            bf_interpret_run(FACTOR_SRC, FACTOR_INPUT)
        });
    }

    #[bench]
    fn bf_jit(bench: &mut Bencher) {
        bench.iter(|| {
            bf_jit_run(FACTOR_SRC, FACTOR_INPUT)
        });
    }

    #[bench]
    fn brainfuck(bench: &mut Bencher) {
        bench.iter(|| {
            brainfuck_run(FACTOR_SRC, FACTOR_INPUT)
        });
    }
}

#[cfg(all(test, feature = "long"))]
mod mandelbrot_bench {
    use super::*;

    #[bench]
    fn bf_interpret(bench: &mut Bencher) {
        bench.iter(|| {
            bf_interpret_run(MANDELBROT_SRC, b"")
        });
    }

    #[bench]
    fn bf_jit(bench: &mut Bencher) {
        bench.iter(|| {
            bf_jit_run(MANDELBROT_SRC, b"")
        });
    }

    #[bench]
    fn brainfuck(bench: &mut Bencher) {
        bench.iter(|| {
            brainfuck_run(MANDELBROT_SRC, b"")
        });
    }
}

