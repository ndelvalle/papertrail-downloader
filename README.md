# papertrail-downloader

papertrail-downloader is a tool to download Papertrail archive logs in parallel.


## Install

### Cargo

```
cargo install papertrail-downloader
```

### Manually

Download the latest [released binary](https://github.com/ndelvalle/papertrail-downloader/releases)
and add executable permissions:

```
wget -O papertrail-downloader "https://github.com/ndelvalle/papertrail-downloader/releases/download/v0.1.1/papertrail-downloader-x86-64-linux"
chmod +x papertrail-downloader
```

## Use

### Command line interface

```
papertrail-downloader 0.1.0
ndelvalle <nicolas.delvalle@gmail.com>
Papertrail parallel log archives downloader

USAGE:
    papertrail-downloader [OPTIONS] --api-token <api-token> --end-date <end-date> --start-date <start-date>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --api-token <api-token>      Papertrail API token
    -e, --end-date <end-date>        End date using 'YYYY-MM-DD' format
    -o, --output-dir <output-dir>    Output directory to store downloaded logs [default: ./]
    -s, --start-date <start-date>    Start date using 'YYYY-MM-DD' format
```
