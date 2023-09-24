mod notifier;

use crate::notifier::TelegramNotifier;
use scraper::{Html, Selector};
use std::thread::sleep;
use std::time::Duration;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;

const LINK: &str = r#"https://market.lun.ua/uk/search?currency=UAH&floor_max=4&geo_id=1&is_without_fee=false&price_max=10000&price_sqm_currency=UAH&section_id=2&sort=relevance&sub_geo_id=31117&sub_geo_id=31904"#;
const SELECTOR: &str = r#".feed-layout__item-holder"#;

type NumberOfAds = usize;

fn main() {
    dotenv::dotenv().unwrap();

    let log_file = rolling::daily("./logs", "app");
    tracing_subscriber::fmt()
        .json()
        .with_writer(log_file.and(std::io::stdout))
        .init();

    tracing::info!("starting app");

    let tg_notifier = TelegramNotifier::new(
        std::env::var("TG_BOT_TOKEN").unwrap(),
        std::env::var("TG_CHAT_ID").unwrap(),
    );

    tg_notifier.send_message("test message").unwrap();

    let two_hours = Duration::from_secs(2 * 60 * 60);
    let mut number_of_ads = get_number_of_ads();
    sleep(two_hours);

    loop {
        let new_number_of_ads = get_number_of_ads();

        if new_number_of_ads > number_of_ads {
            tracing::info!("new ads appeared");
            tg_notifier
                .send_message("New ads appeared!")
                .expect("failed to send tg message");
            number_of_ads = new_number_of_ads;
        } else {
            number_of_ads = new_number_of_ads;
        }

        sleep(two_hours);
    }
}

fn get_number_of_ads() -> NumberOfAds {
    tracing::info!("requesting lun");
    let response = reqwest::blocking::get(LINK)
        .map_err(|err| {
            tracing::error!("failed to request lun: {:?}", err);
            err
        })
        .expect("failed to request lun");

    let html_content = response.text().unwrap();

    let document = Html::parse_document(&html_content);

    // subtract 2 items as they're not ads
    let number_of_ads = document.select(&Selector::parse(SELECTOR).unwrap()).count() - 2;
    tracing::info!("successfully retrieved {} ads", number_of_ads);

    number_of_ads
}
