use clap::{Arg, Command, ArgAction};
// use systemd::daemon::{self, notify};

fn parse_argv() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("miraclecast")
        .version("1.0")
        .about("Remote-display Management-daemon")
        .arg(
            Arg::new("log-level")
                .long("log-level")
                .help("Maximum level for log messages")
                .num_args(1)
                .value_name("lvl"),
        )
        .arg(
            Arg::new("log-time")
                .long("log-time")
                .help("Prefix log-messages with timestamp")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("log-date-time")
                .long("log-date-time")
                .help("Prefix log-messages with date time")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

        // println!("Arguments: {:?}", matches);

    if let Some(log_level) = matches.get_one::<String>("log-level") {
        let log_level: i32 = log_level.parse()?;
        set_log_level(log_level);
    }

    if matches.get_flag("log-time") {
        enable_log_time();
    }

    if matches.get_flag("log-date-time") {
        enable_log_date_time();
    }

    Ok(())

}

fn set_log_level(log_level: i32) {
    println!("Log level set to: {}", log_level);
    // TBD
}

fn enable_log_time() {
    println!("Log messages will be prefixed with timestamp");
    // TBD
}

fn enable_log_date_time() {
    println!("Log messages will be prefixed with date time");
    // TBD
}

fn main() {
    let _ = parse_argv();

    daemon::notify(false, &[Status("Starting up...")]).unwrap();

    daemon::notify(false, &[Ready, Status("Running...")]).unwrap();

    daemon::notify(false, &[Status("Shutting down..."), Stopping]).unwrap();
}