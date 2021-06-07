#![warn(rust_2018_idioms)]

mod challenge_sets;

use console::style;
use std::sync::mpsc::{channel, TryRecvError};
use std::time::Duration;

fn solve_with_spinner(challenge: &str, solution: fn() -> bool) {
    let challenge = challenge.to_owned();
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_style(indicatif::ProgressStyle::default_spinner().tick_chars("⠋⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "));
    spinner.set_message(format!("Solving {}...", challenge));

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
    solve_with_spinner("challenge1", challenge_sets::challenge1::solve);
    solve_with_spinner("challenge2", challenge_sets::challenge2::solve);
    solve_with_spinner("challenge3", challenge_sets::challenge3::solve);
    solve_with_spinner("challenge4", challenge_sets::challenge4::solve);
    solve_with_spinner("challenge5", challenge_sets::challenge5::solve);
    solve_with_spinner("challenge6", challenge_sets::challenge6::solve);
    solve_with_spinner("challenge7", challenge_sets::challenge7::solve);
    solve_with_spinner("challenge8", challenge_sets::challenge8::solve);

    println!("Solving challenge set 2");
    solve_with_spinner("challenge9", challenge_sets::challenge9::solve);
    solve_with_spinner("challenge10", challenge_sets::challenge10::solve);
    solve_with_spinner("challenge11", challenge_sets::challenge11::solve);
}

fn main() {
    solve_all();
}
