use chrono::NaiveDate;
use chrono::NaiveDateTime;
use futures::stream::StreamExt;
use indicatif::ProgressBar;
use reqwest::Client;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

mod cli;

const PAPERTRAIL_URL: &str = "https://papertrailapp.com/api/v1";
// Papertrail has a TCP limit of 10 new connections per second per source IP
// to a log destination (port).
// https://www.papertrail.com/blog/introducing-syslog-ratelimits/
const PAPERTRAIL_PARALLEL_REQUESTS: usize = 10;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::ask().get_matches();

    let output_folder = matches
        .value_of("output-folder")
        .map(|path| shellexpand::tilde(&path).into_owned())
        .unwrap(); // Safe to unwrap value, has clap default.

    let token = matches
        .value_of("api-token")
        .unwrap() // Safe to unwrap value, has clap default.
        .to_owned();

    let start_date = matches
        .value_of("start-date")
        .map(|date| {
            NaiveDate::parse_from_str(date, "%Y-%m-%d")
                .map(|date| date.and_hms(00, 00, 00))
                .expect("Failed to parse start date")
        })
        .unwrap();

    let end_date = matches
        .value_of("end-date")
        .map(|date| {
            NaiveDate::parse_from_str(date, "%Y-%m-%d")
                .map(|date| date.and_hms(00, 00, 00))
                .expect("Failed to parse end date")
        })
        .unwrap();

    let total_logs = (end_date.timestamp() - start_date.timestamp()) / 60 / 60;
    let progress_bar = Arc::new(Mutex::new(ProgressBar::new(total_logs as u64)));
    let client = Client::builder().build().unwrap();
    let output = Arc::new(output_folder);

    let mut date = start_date.clone();
    let mut tasks = vec![];

    while date < end_date {
        let token = token.clone();
        let hour = to_hour(date);
        let url = create_url(hour);
        let output = output.clone();
        let client = client.clone();
        let progress_bar = progress_bar.clone();

        let task = async move {
            let mut stream = client
                .get(&url)
                .header("X-Papertrail-Token", token)
                .send()
                .await
                .expect("Failed to make the request")
                .bytes_stream();

            let path = format!("{}/{}.tsv.gz", output, date.clone());
            let mut file = File::create(path).await.expect("Failed to create file");

            while let Some(item) = stream.next().await {
                let chunk = item.unwrap();
                file.write_all(&chunk).await.unwrap();
            }

            let progress_bar = progress_bar.lock().unwrap();
            progress_bar.inc(1);
        };

        tasks.push(task);
        date = date + chrono::Duration::hours(1);
    }

    futures::stream::iter(tasks)
        .buffer_unordered(PAPERTRAIL_PARALLEL_REQUESTS)
        .collect::<()>()
        .await;

    let progress_bar = progress_bar.lock().unwrap();
    progress_bar.finish();
    Ok(())
}

fn create_url(date: String) -> String {
    format!("{}/archives/{}/download", PAPERTRAIL_URL, date)
}

fn to_hour(date: NaiveDateTime) -> String {
    date.format("%Y-%m-%d-%H").to_string()
}
