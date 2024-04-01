use std::io::Read;

use clap::Parser;
use optimizer::Optimization;

mod assembler;
mod compiler;
mod instruction;
mod optimizer;
mod parser;
mod runtime;

#[derive(Parser)]
struct Args {
    /// Enable specified optimizations
    #[clap(short, long, value_delimiter = ',')]
    optimize: Vec<Optimization>,
    /// Print optimized intermediate representation
    #[clap(long)]
    print_optimized: bool,
    /// Trace execution. Only works if not using native code generation
    #[clap(long)]
    trace: bool,
    /// Enable native code generation
    #[clap(long, default_value = "false")]
    native_codegen: bool,
    /// Input file. If not provided, reads from stdin
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
    let optimized = optimizer::optimize(program, &args.optimize);

    if args.print_optimized {
        dbg!(&optimized);
    }

    if args.native_codegen {
        let compiled = compiler::x86_64::compile(&optimized);
        runtime::native::run(&compiled);
    } else {
        let compiled = compiler::vm::compile(&optimized);
        runtime::vm::run(&compiled, args.trace);
    }

    Ok(())
}
