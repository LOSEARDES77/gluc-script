use clap::Parser;
use std::string::String;

#[derive(Debug)]
enum Arrow {
    DoubleUp,
    SingleUp,
    FortyFiveUp,
    Flat,
    FortyFiveDown,
    SingleDown,
    DoubleDown,
    NotComputable,
    RateOutOfRange,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    url: String,
}

fn parse_url(url: &str) -> String {
    let res;
    let part_1;
    if url.ends_with("/") {
        part_1 = "api/v1/entries/sgv?count=1"
    } else {
        part_1 = "/api/v1/entries/sgv?count=1"
    }
    if url.starts_with("https://") || url.starts_with("http://") {
        res = format!("{}{}", url, part_1);
    } else {
        res = format!("https://{}{}", url, part_1);
    }

    return res;
}
#[tokio::main]
async fn main() {
    let args = Args::parse();

    let url = parse_url(&args.url);
    let resp = reqwest::get(url.to_string())
        .await
        .expect("REASON")
        .text()
        .await
        .unwrap();
    let measurement: u16 = resp.split_whitespace().nth(2).unwrap().parse().unwrap();
    let arrow_type = resp.split_whitespace().nth(3).unwrap().replace("\"", "");
    let arrow;
    let arrow_type = match arrow_type.as_str() {
        "DoubleUp" => Arrow::DoubleUp,
        "SingleUp" => Arrow::SingleUp,
        "FortyFiveUp" => Arrow::FortyFiveUp,
        "Flat" => Arrow::Flat,
        "FortyFiveDown" => Arrow::FortyFiveDown,
        "SingleDown" => Arrow::SingleDown,
        "DoubleDown" => Arrow::DoubleDown,
        "NotComputable" => Arrow::NotComputable,
        "RateOutOfRange" => Arrow::RateOutOfRange,
        _ => panic!("Arrow not found"),
    };
    match arrow_type {
        Arrow::DoubleUp => {
            arrow = "⇈";
        }
        Arrow::SingleUp => {
            arrow = "↑";
        }
        Arrow::FortyFiveUp => {
            arrow = "↗";
        }
        Arrow::Flat => {
            arrow = "→";
        }
        Arrow::FortyFiveDown => {
            arrow = "↘";
        }
        Arrow::SingleDown => {
            arrow = "↓";
        }
        Arrow::DoubleDown => {
            arrow = "⇊";
        }
        Arrow::NotComputable => {
            arrow = "-";
        }
        Arrow::RateOutOfRange => {
            arrow = "-";
        }
    }

    println!("{} {}", arrow, measurement);
}
