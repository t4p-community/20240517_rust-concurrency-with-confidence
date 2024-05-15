#[macro_use]
extern crate prettytable;

mod apple_quality;

use crate::apple_quality::{
    display_comparative_summary_in_table, read_apple_quality_csv, summarize_apples, AppleQuality,
};
use std::sync::Arc;
use std::thread;

fn main() {
    let file_path = "../data/apple_quality.csv";
    let apple_quality = read_apple_quality_csv(file_path).unwrap();
    let apple_quality = Arc::new(apple_quality);

    let apple_quality_good = Arc::clone(&apple_quality);
    let handle_good = thread::spawn(move || {
        let good_apples: Vec<&AppleQuality> = apple_quality_good
            .iter()
            .filter(|x| x.quality == "good")
            .collect();

        // the return value from the closure is the value that will be
        // returned from the thread when called join
        summarize_apples(&good_apples)
    });

    let apple_quality_bad = Arc::clone(&apple_quality);
    let handle_bad = thread::spawn(move || {
        let bad_apples: Vec<&AppleQuality> = apple_quality_bad
            .iter()
            .filter(|x| x.quality == "bad")
            .collect();

        // the return value from the closure is the value that will be
        // returned from the thread when called join
        summarize_apples(&bad_apples)
    });

    // wait for threads to finish and collect summaries
    let good_summary = handle_good.join().unwrap();
    let bad_summary = handle_bad.join().unwrap();

    // using the updated function to print a comparative summary
    display_comparative_summary_in_table(&good_summary, &bad_summary);
}
