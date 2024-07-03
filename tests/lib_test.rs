#[cfg(test)]
mod lib_tests {
    use prepare_csv_for_postgresql_copy::run;
    use std::fs::File;
    use std::io::BufReader;

    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }

    #[test]
    fn run_ascii_semicolon_valid() {
        let f = File::open("tests/fixtures/ascii_semicolon_valid.csv").unwrap();
        let mut reader = BufReader::new(f);
        let mut writer = Vec::new();

        let result = run(&mut reader, &mut writer, 3, b';', b'"', b'"', "ascii");
        assert!(result.is_ok());

        assert_eq!(writer.len(), 24);

        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "foo,bar,baz\nabc,def,ghi\n");
    }

    #[test]
    fn run_utf8_semicolon_valid() {
        let f = File::open("tests/fixtures/utf8_semicolon_valid.csv").unwrap();
        let mut reader = BufReader::new(f);
        let mut writer = Vec::new();

        let result = run(&mut reader, &mut writer, 3, b';', b'"', b'"', "utf8");
        assert!(result.is_ok());

        assert_eq!(writer.len(), 25);

        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "foo,bar,baz\nabc,déf,ghi\n");
    }

    #[test]
    fn run_latin1_semicolon_valid() {
        let f = File::open("tests/fixtures/latin1_semicolon_valid.csv").unwrap();
        let mut reader = BufReader::new(f);
        let mut writer = Vec::new();

        let result = run(&mut reader, &mut writer, 3, b';', b'"', b'"', "latin1");
        assert!(result.is_ok());

        assert_eq!(writer.len(), 25);

        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "foo,bar,baz\nabc,déf,ghi\n");
    }

    #[test]
    fn run_latin1_semicolon_as_utf8_invalid() {
        let f = File::open("tests/fixtures/latin1_semicolon_valid.csv").unwrap();
        let mut reader = BufReader::new(f);
        let mut writer = Vec::new();

        let result = run(&mut reader, &mut writer, 3, b';', b'"', b'"', "utf8");
        assert!(result.is_err());

        let e = result.err().unwrap();
        assert!(e.is::<csv::Error>());

        let e_message = e.to_string();
        assert!(e_message.contains("CSV parse error"));
        assert!(e_message.contains("invalid UTF-8"));
    }

    #[test]
    fn run_utf8_semicolon_as_latin1_valid() {
        let f = File::open("tests/fixtures/utf8_semicolon_valid.csv").unwrap();
        let mut reader = BufReader::new(f);
        let mut writer = Vec::new();

        let result = run(&mut reader, &mut writer, 3, b';', b'"', b'"', "latin1");
        assert!(result.is_ok());

        assert_eq!(writer.len(), 27);

        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "foo,bar,baz\nabc,dÃ©f,ghi\n");
    }

    #[test]
    fn run_ascii_semicolon_flexible_valid_2_fields() {
        let f = File::open("tests/fixtures/ascii_semicolon_flexible_valid.csv").unwrap();
        let mut reader = BufReader::new(f);
        let mut writer = Vec::new();

        let result = run(&mut reader, &mut writer, 2, b';', b'"', b'"', "ascii");
        assert!(result.is_ok());

        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "foo,bar\nabc,def\nabc,def\n");
    }

    #[test]
    fn run_ascii_semicolon_flexible_valid_3_fields() {
        let f = File::open("tests/fixtures/ascii_semicolon_flexible_valid.csv").unwrap();
        let mut reader = BufReader::new(f);
        let mut writer = Vec::new();

        let result = run(&mut reader, &mut writer, 3, b';', b'"', b'"', "ascii");
        assert!(result.is_ok());

        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "foo,bar,baz\nabc,def,ghi\n");
    }

    #[test]
    fn run_ascii_semicolon_flexible_valid_4_fields() {
        let f = File::open("tests/fixtures/ascii_semicolon_flexible_valid.csv").unwrap();
        let mut reader = BufReader::new(f);
        let mut writer = Vec::new();

        let result = run(&mut reader, &mut writer, 4, b';', b'"', b'"', "ascii");
        assert!(result.is_ok());

        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "abc,def,ghi,jkl\n");
    }

    #[test]
    fn run_ascii_semicolon_double_quote_valid() {
        let f = File::open("tests/fixtures/ascii_semicolon_double_quote_valid.csv").unwrap();
        let mut reader = BufReader::new(f);
        let mut writer = Vec::new();

        let result = run(&mut reader, &mut writer, 3, b';', b'"', b'"', "ascii");
        assert!(result.is_ok());

        let output = String::from_utf8(writer).unwrap();
        println!("{}", output);
        assert_eq!(output, "foo,bar,baz\nabc,\"def;g\"\"h\"\"i\",jkl\n");
    }

    #[test]
    fn run_ascii_semicolon_null_quote_valid() {
        let f = File::open("tests/fixtures/ascii_semicolon_null_quote_valid.csv").unwrap();
        let mut reader = BufReader::new(f);
        let mut writer = Vec::new();

        let result = run(&mut reader, &mut writer, 3, b';', b'\0', b'\\', "ascii");
        assert!(result.is_ok());

        let output = String::from_utf8(writer).unwrap();
        println!("{}", output);
        assert_eq!(output, "foo,bar,baz\nabc,\"def;g\"\"h\"\"i\",jkl\n");
    }
}
