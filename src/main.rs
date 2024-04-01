use std::io::Read;

use clap::Parser;

mod assembler;
mod compiler;
mod instruction;
mod optimizer;
mod parser;
mod runtime;

#[derive(Parser)]
struct Args {
    #[clap(short = 'O', long, default_value = "2", value_parser = 0..=2)]
    optimize_level: i64,
    #[clap(long)]
    print_optimized: bool,
    #[clap(long)]
    trace: bool,
    #[clap(long, default_value = "true")]
    native_compile: bool,
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

    if args.native_compile {
        let compiled = compiler::x86_64::compile(&optimized);
        runtime::native::run(&compiled);
    } else {
        let compiled = compiler::vm::compile(&optimized);
        runtime::vm::run(&compiled, args.trace);
    }

    Ok(())
}
