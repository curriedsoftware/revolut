mod codegen;
mod naming;
mod parser;
mod types;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "revolut-codegen", about = "Generate Rust types from OpenAPI specs")]
struct Args {
    /// Path to the OpenAPI YAML spec file
    #[arg(long)]
    spec: PathBuf,

    /// Output directory for generated code
    #[arg(long)]
    output: PathBuf,

    /// API name (business, merchant, open_banking, crypto_ramp)
    #[arg(long)]
    api: String,
}

fn main() {
    let args = Args::parse();

    eprintln!(
        "Generating {} types from {}...",
        args.api,
        args.spec.display()
    );

    let schemas = parser::parse_spec(
        args.spec
            .to_str()
            .expect("Invalid spec path"),
    );

    eprintln!("Parsed {} schemas", schemas.len());

    let code = codegen::generate(&schemas);

    // Ensure output directory exists
    std::fs::create_dir_all(&args.output).expect("Failed to create output directory");

    let output_file = args.output.join("mod.rs");
    std::fs::write(&output_file, code).expect("Failed to write output file");

    eprintln!("Generated {}", output_file.display());
}
