use clap::Parser;
use sanitize_csv::run;
use std::{io, process};

/// Convert a stdin CSV to PostgreSQL 15 standards for use in `COPY ... FROM ... WITH FORMAT CSV`
/// The output uses the following standards (which are also the input defaults):
///   DELIMITER ','
///   QUOTE '"'
///   ESCAPE '"'
///   ENCODING 'UTF8'
///
/// Has optional number of fields to allow use in non-PostgreSQL contexts
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// input delimiter
    #[arg(short = 'd', long, default_value_t = ',')]
    input_delimiter: char,

    /// output delimiter
    #[arg(short = 'D', long, default_value_t = ',')]
    output_delimiter: char,

    /// Input Quote using
    #[arg(short, long, default_value_t = '"')]
    quote: char,

    /// Input Escape
    #[arg(short, long, default_value_t = '"')]
    escape: char,

    /// Force input encoding label (https://encoding.spec.whatwg.org/#concept-encoding-get).
    /// Ignores and strips BOM
    #[arg(short, long)]
    force_encoding: Option<String>,

    /// Optional number of fields (truncates to number or skips row if too little).
    /// Advised to use in PostgreSQL context
    #[arg(short, long)]
    number_of_fields: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let number_of_fields = args.number_of_fields;
    let input_delimiter = args.input_delimiter as u8;
    let output_delimiter = args.output_delimiter as u8;
    let quote = args.quote as u8;
    let escape = args.escape as u8;
    let force_encoding = &args.force_encoding;

    let stdin = io::stdin();
    let reader = stdin.lock();

    match run(
        reader,
        io::stdout(),
        number_of_fields,
        input_delimiter,
        output_delimiter,
        quote,
        escape,
        force_encoding.as_deref(),
    ) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("error running example: {}", e);
            process::exit(1);
        }
    }
}
