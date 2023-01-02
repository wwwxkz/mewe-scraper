fn main() {
    let response = reqwest::blocking::get(
        "https://mewe.com/i/wwwxkz"
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);
}
