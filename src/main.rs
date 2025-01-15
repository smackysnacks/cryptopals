#![warn(rust_2018_idioms)]

mod challenge_sets;

use console::style;
use std::sync::mpsc::{channel, TryRecvError};
use std::time::Duration;

fn solve(challenge: &str, solution: fn() -> bool) {
    let challenge = challenge.to_owned();
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_style(indicatif::ProgressStyle::default_spinner().tick_chars("⠋⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "));
    spinner.set_message(format!("Solving {challenge}..."));

    let (tx, rx) = channel();
    std::thread::spawn(move || {
        let passed = solution();
        tx.send(passed).unwrap();
    });

    loop {
        match rx.try_recv() {
            Ok(passed) => {
                if passed {
                    spinner.finish_with_message(format!(
                        "{}... {}",
                        challenge,
                        style("Pass").green()
                    ));
                } else {
                    spinner.finish_with_message(format!(
                        "{}... {}",
                        challenge,
                        style("Fail").red()
                    ));
                }

                break;
            }
            Err(TryRecvError::Disconnected) => {
                spinner.finish_with_message(format!(
                    "{}... {}",
                    challenge,
                    style("Never finished!").red()
                ));

                break;
            }
            Err(TryRecvError::Empty) => {}
        }

        std::thread::sleep(Duration::from_millis(40));
        spinner.tick();
    }
}

fn solve_all() {
    println!("Solving challenge set 1");
    solve("challenge1", challenge_sets::challenge1::solve);
    solve("challenge2", challenge_sets::challenge2::solve);
    solve("challenge3", challenge_sets::challenge3::solve);
    solve("challenge4", challenge_sets::challenge4::solve);
    solve("challenge5", challenge_sets::challenge5::solve);
    solve("challenge6", challenge_sets::challenge6::solve);
    solve("challenge7", challenge_sets::challenge7::solve);
    solve("challenge8", challenge_sets::challenge8::solve);

    println!("Solving challenge set 2");
    solve("challenge9", challenge_sets::challenge9::solve);
    solve("challenge10", challenge_sets::challenge10::solve);
    solve("challenge11", challenge_sets::challenge11::solve);
    solve("challenge12", challenge_sets::challenge12::solve);
}

fn main() {
    solve_all();
}
