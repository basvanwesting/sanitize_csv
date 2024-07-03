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
    /// Optional number of fields (truncates to number or skips row if too little)
    /// Preferred for use in PostgreSQL context
    #[arg(short, long)]
    number_of_fields: Option<usize>,

    /// Input Delimiter
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    /// Input Quote using
    #[arg(short, long, default_value_t = '"')]
    quote: char,

    /// Input Escape
    #[arg(long, default_value_t = '"')]
    escape: char,

    /// Input forced encoding label (https://encoding.spec.whatwg.org/#concept-encoding-get)
    /// strips BOM
    #[arg(long)]
    encoding: Option<String>,
}

fn main() {
    let args = Args::parse();
    let number_of_fields = args.number_of_fields;
    let delimiter = args.delimiter as u8;
    let quote = args.quote as u8;
    let escape = args.escape as u8;
    let encoding = &args.encoding;

    let stdin = io::stdin();
    let reader = stdin.lock();

    match run(
        reader,
        io::stdout(),
        number_of_fields,
        delimiter,
        quote,
        escape,
        encoding.as_deref(),
    ) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("error running example: {}", e);
            process::exit(1);
        }
    }
}
