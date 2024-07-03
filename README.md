Convert a stdin CSV to PostgreSQL 15 standards for use in `COPY ... FROM ... WITH FORMAT CSV` \
The output uses the following standards (which are also the input defaults): \ 
  DELIMITER ',' \ 
  QUOTE '"' \ 
  ESCAPE '"' \
  ENCODING 'UTF8'

```
Usage: sanitize_csv [OPTIONS]

Options:
  -d, --delimiter <DELIMITER>
          Input Delimiter [default: ,]
  -q, --quote <QUOTE>
          Input Quote using [default: "]
  -e, --escape <ESCAPE>
          Input Escape [default: "]
  -f, --force-encoding <FORCE_ENCODING>
          Force input encoding label (https://encoding.spec.whatwg.org/#concept-encoding-get). Ignores and strips BOM
  -n, --number-of-fields <NUMBER_OF_FIELDS>
          Optional number of fields (truncates to number or skips row if too little). Advised to use in PostgreSQL context
  -h, --help
          Print help
  -V, --version
          Print version
```
