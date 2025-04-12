use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use build_html::{Container, Html as HtmlBuilder, HtmlContainer, HtmlPage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const MOODLE_LOGIN: &str = "is-33fiot-23-141";
const MOODLE_DATA: &str = "ІС-33 Рибалко Максим, 2 курс";

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyRate {
    pub ccy: String,
    pub base_ccy: String,
    pub buy: String,
    pub sale: String,
}

pub async fn get_currency_rate() -> Result<impl IntoResponse, StatusCode> {
    tracing::info!("Request for getting currency rate");

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.privatbank.ua/p24api/pubinfo?exchange&coursid=5")
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let currency_rate: Vec<CurrencyRate> = response
        .json()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(rate) = currency_rate.first() {
        let html = generate_rate_html(rate);
        tracing::info!("Successfuly got a currency rate");
        return Ok(Html(html));
    }

    tracing::error!("Some error while getting currency rate from API");
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_moodle(Path(moodle): Path<String>) -> impl IntoResponse {
    tracing::info!("Request for getting moodle data for {}", moodle);

    if moodle == MOODLE_LOGIN {
        tracing::info!("Successfuly got data for {}", moodle);

        (StatusCode::OK, Html(format!("<h2>{}</h2>", MOODLE_DATA)))
    } else {
        tracing::error!("No data for {}", moodle);
        (
            StatusCode::NOT_FOUND,
            Html(format!("<h2>No data about {}</h2>", moodle)),
        )
    }
}

fn generate_rate_html(rate: &CurrencyRate) -> String {
    let mut meta_attributes = HashMap::new();
    meta_attributes.insert("charset", "utf-8");

    let html = HtmlPage::new()
        .with_meta(meta_attributes)
        .with_title("Currency rate")
        .with_container(
            Container::new(build_html::ContainerType::Div)
                .with_header(3, format!("ccy: {}", rate.ccy))
                .with_header(3, format!("base ccy: {}", rate.base_ccy))
                .with_header(3, format!("buy: {}", rate.buy))
                .with_header(3, format!("sale: {}", rate.sale)),
        );

    html.to_html_string()
}
