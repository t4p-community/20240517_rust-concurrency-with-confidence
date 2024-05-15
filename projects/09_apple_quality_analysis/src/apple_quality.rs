use csv::ReaderBuilder;
use prettytable::{format, Cell, Row, Table};
use std::error::Error;
use std::fs::File;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppleQuality {
    #[serde(rename = "A_id")]
    pub id: i32,
    #[serde(rename = "Size")]
    pub size: f64,
    #[serde(rename = "Weight")]
    pub weight: f64,
    #[serde(rename = "Sweetness")]
    pub sweetness: f64,
    #[serde(rename = "Crunchiness")]
    pub crunchiness: f64,
    #[serde(rename = "Juiciness")]
    pub juiciness: f64,
    #[serde(rename = "Ripeness")]
    pub ripeness: f64,
    #[serde(rename = "Acidity")]
    pub acidity: f64,
    #[serde(rename = "Quality")]
    pub quality: String,
}

pub fn read_apple_quality_csv(file_path: &str) -> Result<Vec<AppleQuality>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    let apple_quality: Vec<AppleQuality> = rdr.deserialize().collect::<Result<_, csv::Error>>()?;
    Ok(apple_quality)
}

pub fn summarize_apples(apples: &Vec<&AppleQuality>) -> Vec<f64> {
    let mut summary = vec![0.0; 7]; // We have 7 numeric fields to summarize
    for apple in apples.iter() {
        summary[0] += apple.size;
        summary[1] += apple.weight;
        summary[2] += apple.sweetness;
        summary[3] += apple.crunchiness;
        summary[4] += apple.juiciness;
        summary[5] += apple.ripeness;
        summary[6] += apple.acidity;
    }

    // Converting totals to averages
    for val in &mut summary {
        *val /= apples.len() as f64;
    }

    summary
}

pub fn display_comparative_summary_in_table(good_summary: &[f64], bad_summary: &[f64]) {
    let attributes = [
        "Size",
        "Weight",
        "Sweetness",
        "Crunchiness",
        "Juiciness",
        "Ripeness",
        "Acidity",
    ];

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    // Adding the header row with attributes and types of apple for comparison
    table.add_row(row!["Attribute", "Good Apples", "Bad Apples"]);

    // Iterating through attributes and appending respective summaries
    for (i, &attr) in attributes.iter().enumerate() {
        table.add_row(Row::new(vec![
            Cell::new(attr),
            Cell::new(&format!("{:.2}", good_summary[i])),
            Cell::new(&format!("{:.2}", bad_summary[i])),
        ]));
    }

    // Printing the table
    table.printstd();
}
