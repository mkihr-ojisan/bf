use std::io::Read;

use clap::Parser;

//mod assembler;
mod compiler;
mod instruction;
mod optimizer;
mod parser;
mod vm;

#[derive(Parser)]
struct Args {
    #[clap(short = 'O', long, default_value = "0", value_parser = 0..=2)]
    optimize_level: i64,
    #[clap(long)]
    print_optimized: bool,
    #[clap(long)]
    trace: bool,
    filename: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let input = if let Some(filename) = args.filename {
        std::fs::read_to_string(filename)?
    } else {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input)?;
        input
    };

    let program = parser::parse(&input)?;
    let optimized = optimizer::optimize(&program, args.optimize_level);

    if args.print_optimized {
        dbg!(&optimized);
    }

    let compiled = compiler::compile(&optimized);
    vm::run(&compiled, args.trace);

    Ok(())
}
