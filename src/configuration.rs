use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Config {
    input: Box<Path>,
    output: Box<Path>,
    steps: Option<u32>,
    dump_freq: Option<u32>,
}

/**
 * TODO:
 * `--steps=<i32>`
 * `--dump_freq=<i32>`
 */
impl Config {
    pub fn new(args: Vec<String>) -> Self {
        let mut args = args.into_iter();

        // The first argument is the executable name, as supplied by
        // `std::env::args()`.
        args.next();

        let input = args
            .next()
            .expect("missing input path: usage: game-of-life <input> <output>");
        let output = args
            .next()
            .expect("missing output path: usage: game-of-life <input> <output>");

        Self {
            input: PathBuf::from(input).into_boxed_path(),
            output: PathBuf::from(output).into_boxed_path(),
            steps: Some(10),
            dump_freq: None,
        }
    }
}
