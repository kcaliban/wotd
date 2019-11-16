use std::fs::File;
use std::fs::Metadata;
use std::env;
use std::io::prelude::*;
use std::path::Path;
use std::time::SystemTime;
use scraper::{Html, Selector};
use chrono::{DateTime, Local, NaiveDate, Datelike};

fn main() {
    let raw_html = get_html("https://www.merriam-webster.com/word-of-the-day");
    // Parse HTML to scraper
    let scraper_html: scraper::html::Html = Html::parse_document(&raw_html);

    // Get word
    let wotd = extr_wotd(&scraper_html);
    // Get definitions
    let defs = extr_def(&scraper_html);

    // Enable grey color
    print!("\x1b[2m");
    print!("Word: ");
    // Enable underlining, print word, disable underlining
    print!("\x1b[4m{}\x1b[24m\n\n", wotd);
    print!("Definitions:\n");
    // Print definitions
    for def in defs {
        println!("{}", def);
    }
    // Disable grey color
    print!("\x1b[22m");

}

// Check if file last modified today
fn last_modified_today(path: &Path) -> bool {
    let metadata = std::fs::metadata(path).unwrap();
    let mut old_date : NaiveDate;
    let now : NaiveDate = Local::today().naive_local();

    if let Ok(time) = metadata.modified() {
        // Get local date from system time
        old_date = DateTime::<Local>::from(time).date().naive_local();
    } else{
        println!("Could not get time of last modification from file");
        return false;
    }

    if now.year() > old_date.year() || now.month() > old_date.month()
                                    || now.day() > old_date.day() {
        return true;
    }

    false
}

// Fetch HTML
fn get_html(url: &str) -> String {
    // Request HTML
    let mut resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success(),
            "Could not fetch HTML, check your internet connection");

    // Extract HTML
    let raw_html = resp.text().unwrap();

    raw_html
}

// Extract word of the day
fn extr_wotd(scraper_html: &scraper::html::Html) -> String {
    let selector = Selector::parse(".word-and-pronunciation h1").unwrap();

    let wotd_h1 = scraper_html.select(&selector).next().unwrap();
    let wotd_txt = wotd_h1.text().collect::<Vec<_>>();

    format!("{}", wotd_txt[0])
}

// Extract definition
fn extr_def(scraper_html: &scraper::html::Html) -> Vec<String> {
    let selector = Selector::parse(".wod-definition-container > p").unwrap();

    let mut defs : Vec<String> = Vec::new();

    for def in scraper_html.select(&selector) {
        let def_txt = def.text().collect::<Vec<_>>();
        defs.push(format!("{}", def_txt.join("")));
    }

    defs
}
