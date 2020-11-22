use brainfuck_maker::grammar::*;
use brainfuck_maker::interpreter::*;
use brainfuck_maker::parser::*;
use clap::Clap;
use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::result::Result;

#[derive(Clap, Debug)]
#[clap(name = "brainfuck-maker", version = "0.1.0", author = "guricerin")]
struct Opts {
    #[clap(short, long, name = "brainfuck src code file")]
    code_path: PathBuf,
    #[clap(short, long, name = "brainfuck grammar file")]
    grammar_path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let code = fs::read_to_string(opts.code_path)?;
    //println!("content: \n{}", &program);
    let mut parser = Parser::new(&code);
    if let Some(grammar_path) = opts.grammar_path {
        let grammar = Grammar::new(
            "ふるえるぞハート!",
            "燃えつきるほどヒート!!",
            "オラ",
            "無駄",
            "ァ!",
            "やれやれだぜ",
            "おまえの次のセリフは「",
            "」という!",
        );
        parser.replace(&grammar);
    }
    let tokens = parser.parse();
    let mut interpreter = Interpreter::new(30000);
    interpreter.run(&tokens)?;

    Ok(())
}
