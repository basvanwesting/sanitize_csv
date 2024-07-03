use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::error::Error;
use std::io::{Read, Write};

pub fn run<R, W>(
    io_reader: R,
    io_writer: W,
    number_of_fields: Option<usize>,
    delimiter: u8,
    quote: u8,
    escape: u8,
    encoding: &str,
) -> Result<(), Box<dyn Error>>
where
    R: Read,
    W: Write,
{
    let mut decode_builder = DecodeReaderBytesBuilder::new();
    init_decode_builder(&mut decode_builder, encoding);
    let io_reader_utf8 = decode_builder.build(io_reader);

    let mut csv_reader_builder = csv::ReaderBuilder::new();
    init_csv_reader_builder(&mut csv_reader_builder, delimiter, quote, escape);
    let mut csv_reader = csv_reader_builder.from_reader(io_reader_utf8);

    let mut csv_writer = csv::WriterBuilder::new()
        .flexible(number_of_fields.is_none())
        .from_writer(io_writer);

    for result in csv_reader.records() {
        match result {
            Ok(mut record) => {
                if let Some(truncate_to) = number_of_fields {
                    if record.len() >= truncate_to {
                        record.truncate(truncate_to);
                        record.trim();
                        csv_writer.write_record(&record)?;
                    } else {
                        // ignore record
                    }
                } else {
                    record.trim();
                    csv_writer.write_record(&record)?;
                }
            }
            Err(err) => {
                //eprintln!("error reading CSV from <stdin>: {}", err),
                return Err(Box::new(err));
            }
        }
    }
    Ok(())
}

fn init_decode_builder(decode_builder: &mut DecodeReaderBytesBuilder, encoding: &str) {
    match encoding {
        "latin1" | "ISO-8859-1" | "ISO_8859_1" | "WINDOWS-1252" | "WINDOWS_1252" => {
            decode_builder
                .bom_sniffing(false)
                .strip_bom(true)
                .encoding(Some(WINDOWS_1252));
        }
        _ => (),
    };
}

fn init_csv_reader_builder(
    csv_reader_builder: &mut csv::ReaderBuilder,
    delimiter: u8,
    quote: u8,
    escape: u8,
) {
    csv_reader_builder
        .has_headers(false)
        .flexible(true)
        .trim(csv::Trim::All)
        .delimiter(delimiter)
        .quote(quote);

    match escape {
        b'"' => csv_reader_builder,
        _ => csv_reader_builder.double_quote(false).escape(Some(escape)),
    };
}
