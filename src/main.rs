use std::{fs, path::PathBuf, process::ExitCode};

use c_compiler::{Token, lexer::Lexer, parse};
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Run the lexer, but stop before parsing
    #[arg(long)]
    lex: bool,
    /// Run the lexer and parser, but stop before assembly generation
    #[arg(long)]
    parse: bool,
    /// Perform lexing, parsing, and assembly generation, but stop before code emission
    #[arg(long)]
    codegen: bool,

    /// Emit assembly, but not assemble or link it
    #[arg(short = 'S')]
    s: bool,

    src_path: PathBuf,
}

fn main() -> Result<(), ExitCode> {
    let args = Args::parse();
    let src = fs::read_to_string(args.src_path).expect("Failed to read the source file");

    if args.lex {
        let lexer = Lexer::new(&src);
        let tokens = lexer.lex_all();
        let mut has_lexing_errors = false;
        for token in &tokens {
            if let Token::Error(msg) = token {
                eprintln!("Lexing Error: {}", msg);
                has_lexing_errors = true;
            }
        }

        // Always print the tokens for debugging, regardless of errors
        println!("Tokens: {:?}", tokens);

        if has_lexing_errors {
            // Return a non-zero exit code to signal failure
            return Err(ExitCode::from(1));
        }
    } else if args.parse {
        let program = parse(&src);
        println!("{:?}", program);
    }

    Ok(())
}
