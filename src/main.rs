mod models;
mod parser;
mod round;
mod stats;

use models::QuakeAction;
use parser::QuakeLogLine;
use round::QuakeRound;
use stats::RoundStats;

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};

fn get_log_path() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("* Usage: {} <path_to_quake_log>", args[0]);
        std::process::exit(1);
    }

    args[1].clone()
}

fn write_stats_to_file(stats: &[RoundStats]) -> Result<(), std::io::Error> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("./quake-report.json")?;
    serde_json::to_writer_pretty(file, stats)?;
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let log_path = get_log_path();
    println!("Reading log file at {}", log_path);
    let file = File::open(log_path)?;
    let reader = BufReader::new(file);

    let mut stats: Vec<RoundStats> = Vec::new();
    let mut round: QuakeRound = QuakeRound::new(1);

    println!("Processing log file...");
    for line in reader.lines() {
        let line = line?;

        if let Some(quake_line) = QuakeLogLine::from_str(&line) {
            match quake_line.get_action() {
                QuakeAction::ShutdownGame | QuakeAction::InitGame => {
                    // needed to handle case where Shutdowngame isn't logged
                    if round.is_started() {
                        stats.push(round.into());
                        round = QuakeRound::new(stats.len() as u32 + 1);
                    }
                    if *quake_line.get_action() == QuakeAction::InitGame {
                        round.start_round();
                    }
                }
                QuakeAction::ClientUserInfoChanged => {
                    round.process_client_user_info_changed(&quake_line);
                }
                QuakeAction::Kill => {
                    round.process_kill(&quake_line);
                }
                _ => continue,
            };
        };
    }

    // case where the last round doesn't end with ShutdownGame
    if round.is_started() {
        stats.push(round.into());
    }

    write_stats_to_file(&stats)?;
    println!("Report generated at: ./quake-report.json");
    Ok(())
}
