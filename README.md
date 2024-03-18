# salog
cli using rust

## help
Usage: salog.exe [OPTIONS] <--input-file <INPUT_FILE>|--input-url <INPUT_URL>|--input-es-index <INPUT_ES_INDEX>>
Options:
  -F, --input-file <INPUT_FILE>              INPUT COMMAND : input logs from file
  -U, --input-url <INPUT_URL>                INPUT COMMAND : input logs from url
  -E, --input-es-index <INPUT_ES_INDEX>      INPUT COMMAND : input logs from elastic search index
  -r, --reverse                              reverse before limit logs message
      --level <LEVEL>                        filter by level
  -j, --json                                 return logs as json text
  -t, --truncate                             remove all existing logs before save
  -f, --save-to-file <SAVE_TO_FILE>          save logs to file
  -c, --count                                return logs as json text
  -s, --summary                              return logs as json text
  -e, --save-to-es-index <SAVE_TO_ES_INDEX>  save logs to elastic search index
  -l, --limit <LIMIT>                        limit number, only take the n limit from first
      --date-filter <DATE_FILTER>
  -p, --pretty-json                          show pretty in json output
  -v, --verbose                              show pretty in json output
  -h, --help                                 Print help
  -V, --version                              Print version
