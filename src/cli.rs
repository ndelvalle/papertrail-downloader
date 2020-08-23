pub fn ask<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            clap::Arg::with_name("start-date")
                .short("s")
                .long("start-date")
                .help("Start date using 'YYYY-MM-DD' format")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("end-date")
                .short("e")
                .long("end-date")
                .help("End date using 'YYYY-MM-DD' format")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("api-token")
                .short("t")
                .long("api-token")
                .help("Papertrail API token")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("output-dir")
                .short("o")
                .long("output-dir")
                .help("Output directory to store downloaded logs")
                .takes_value(true)
                .default_value("./"),
        )
}
