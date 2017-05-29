#![feature(test)]

extern crate test;
extern crate bf;
extern crate brainfuck;

pub use test::Bencher;

static FACTOR_SRC: &'static str =
    include_str!("../bf-src/factor.bf");

static HELLO_SRC: &'static str =
    include_str!("../bf-src/hello-world.bf");

#[cfg(feature = "long")]
static MANDELBROT_SRC: &'static str =
    include_str!("../bf-src/mandelbrot.bf");

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
    use brainfuck::tape::Mod256ArrayTape;

    let mut reader = input;
    let mut writer = Vec::new();
    let program = Program::parse(program).unwrap();
    {
        let mut interpreter = Interpreter::<Mod256ArrayTape>::new(program, &mut reader, &mut writer);
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

macro_rules! bench_bfs {
    ($mod_name:ident, $program:expr, $input:expr) =>
    {
        #[cfg(test)]
        mod $mod_name {
            use super::*;

            #[bench]
            fn bf_interpret(bench: &mut Bencher) {
                bench.iter(|| {
                    bf_interpret_run($program, $input)
                });
            }

            #[bench]
            fn bf_jit(bench: &mut Bencher) {
                bench.iter(|| {
                    bf_jit_run($program, $input)
                });
            }

            #[bench]
            fn brainfuck(bench: &mut Bencher) {
                bench.iter(|| {
                    brainfuck_run($program, $input)
                });
            }
        }
    }
}

bench_bfs!(empty_program, "", b"");
bench_bfs!(hello, HELLO_SRC, b"");
bench_bfs!(factor_1_000_000, FACTOR_SRC, b"1000000\n");

#[cfg(feature = "long")]
bench_bfs!(factor_179_424_691, FACTOR_SRC, b"179424691\n");

#[cfg(feature = "long")]
bench_bfs!(mandelbrot, MANDELBROT_SRC, b"");

