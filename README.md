# salog
cli using rust

## help
Usage: salog.exe [OPTIONS] <--input-file <INPUT_FILE>|--input-url <INPUT_URL>|--input-es-index <INPUT_ES_INDEX>>

Options:
- `-F, --input-file <INPUT_FILE>`: INPUT COMMAND - input logs from a file
- `-U, --input-url <INPUT_URL>`: INPUT COMMAND - input logs from a URL
- `-E, --input-es-index <INPUT_ES_INDEX>`: INPUT COMMAND - input logs from an Elasticsearch index
- `-r, --reverse`: Reverse before limiting log messages
- `--level <LEVEL>`: Filter logs by level
- `-j, --json`: Return logs as JSON text
- `-t, --truncate`: Remove all existing logs before saving
- `-f, --save-to-file <SAVE_TO_FILE>`: Save logs to a file
- `-c, --count`: Return logs as JSON text
- `-s, --summary`: Return logs as JSON text
- `-e, --save-to-es-index <SAVE_TO_ES_INDEX>`: Save logs to an Elasticsearch index
- `-l, --limit <LIMIT>`: Limit the number of logs (take only the first n)
- `--date-filter <DATE_FILTER>`
- `-p, --pretty-json`: Show pretty JSON output
- `-v, --verbose`: Show verbose JSON output
- `-h, --help`: Print help
- `-V, --version`: Print version
