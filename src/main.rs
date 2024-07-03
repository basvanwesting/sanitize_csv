use clap::Parser;
use prepare_csv_for_postgresql_copy::run;
use std::{io, process};

/// Convert a stdin CSV to PostgreSQL 15 standards for use in `COPY ... FROM ... WITH FORMAT CSV`
/// The output uses the following standards (which are also the input defaults):
///   DELIMITER ','
///   QUOTE '"'
///   ESCAPE '"'
///   ENCODING 'UTF8'
///
/// Also truncates the number of fields to the exact amount PostgreSQL 15 expects for COPY
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// number of fields (truncates to number or skips row if too little)
    #[arg(short, long)]
    number_of_fields: u8,

    /// Input Delimiter
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    /// Input Quote using
    #[arg(short, long, default_value_t = '"')]
    quote: char,

    /// Input Escape
    #[arg(long, default_value_t = '"')]
    escape: char,

    /// Input encoding
    #[arg(long, default_value = "utf8")]
    encoding: String,
}

fn main() {
    let args = Args::parse();
    let number_of_fields = args.number_of_fields as usize;
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
        encoding,
    ) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("error running example: {}", e);
            process::exit(1);
        }
    }
}
