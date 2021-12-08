use reqwest;
use scraper::{Html, Selector};

pub async fn get_once_results() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://www.juegosonce.es/resultados-cupon-diario")
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&response);
    let cupon = Selector::parse("span.numerocupon").unwrap();

    Ok(document
        .select(&cupon)
        .last()
        .unwrap()
        .text()
        .collect::<String>())
}
