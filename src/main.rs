use chrono::NaiveDate;
use chrono::NaiveDateTime;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use reqwest::blocking::Client as ReqwestClient;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;

// Papertrail documentation about permanent log archives.
// https://www.papertrail.com/help/permanent-log-archives/

mod cli;

const PAPERTRAIL_URL: &str = "https://papertrailapp.com/api/v1";
// Papertrail has a TCP limit of 10 new connections per second per source IP
// to a log destination (port).
// https://www.papertrail.com/blog/introducing-syslog-ratelimits/
const PAPERTRAIL_PARALLEL_REQUESTS: usize = 10;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::ask().get_matches();

    let token = matches
        .value_of("token")
        .expect("API token is a required attribute")
        .to_owned();

    let output_dir = matches
        .value_of("output-dir")
        .map(|path| shellexpand::tilde(&path).into_owned())
        .unwrap(); // Safe to unwrap value, has clap default.

    let start_date = matches
        .value_of("start-date")
        .map(|date| {
            NaiveDate::parse_from_str(date, "%Y-%m-%d")
                .map(|date| date.and_hms(00, 00, 00))
                .expect("Failed to parse start date")
        })
        .unwrap(); // Safe to unwrap value, has clap required attribute.

    let end_date = matches
        .value_of("end-date")
        .map(|date| {
            NaiveDate::parse_from_str(date, "%Y-%m-%d")
                .map(|date| date.and_hms(00, 00, 00))
                .expect("Failed to parse end date")
        })
        .unwrap(); // Safe to unwrap value, has clap required attribute.

    create_dir_all(&output_dir).expect("Failed to create output directory");

    let logs_count = (end_date.timestamp() - start_date.timestamp()) / 60 / 60;

    let progress_bar = ProgressBar::new(logs_count as u64);
    progress_bar.set_style(ProgressStyle::default_bar().template(
        "{spinner} [{elapsed_precise}] [{wide_bar}] {pos}/{len} ({per_sec}, ETA: {eta})",
    ));

    let reqwest_client = reqwest::blocking::Client::new();
    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(PAPERTRAIL_PARALLEL_REQUESTS)
        .build()
        .expect("Failed to create thread pool");

    let dates = get_dates(start_date, end_date);

    thread_pool.install(|| {
        dates
            .par_iter()
            .inspect(|_| progress_bar.inc(1))
            .for_each(|date| download_log(&reqwest_client, &token, date, &output_dir));
    });

    progress_bar.finish();
    Ok(())
}

fn download_log(
    reqwest_client: &ReqwestClient,
    token: &String,
    date: &NaiveDateTime,
    output_dir: &String,
) {
    let ymdh = to_ymdh(date);
    let url = create_url(&ymdh);

    let response = reqwest_client
        .get(&url)
        .header("X-Papertrail-Token", token)
        .send()
        .expect("Failed to create the HTTP request");

    if !response.status().is_success() {
        let error = response.text().expect("Failed to read response error body");
        eprintln!(
            "Failed to download log for date {}. Error: {}.\n",
            &date, error
        );
        return;
    }

    let bytes = response
        .bytes()
        .expect("Failed to read bytes from HTTP response");

    let path = format!("{}/{}.tsv.gz", output_dir, date.clone());
    let mut file = File::create(path).expect("Failed to create log file");

    file.write_all(bytes.as_ref())
        .expect("Failed to write log bytes to file");
}

fn create_url(date: &String) -> String {
    format!("{}/archives/{}/download", PAPERTRAIL_URL, date)
}

fn to_ymdh(date: &NaiveDateTime) -> String {
    date.format("%Y-%m-%d-%H").to_string()
}

fn get_dates(start_date: NaiveDateTime, end_date: NaiveDateTime) -> Vec<NaiveDateTime> {
    let mut date = start_date;
    let mut dates = vec![];

    while date < end_date {
        dates.push(date);
        date += chrono::Duration::hours(1);
    }

    dates
}
