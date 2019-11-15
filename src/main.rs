use scraper::{Html, Selector};

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
