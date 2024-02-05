use std::env::args;
use std::process;
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

#[tokio::main]
async fn main() {
    let args = args().collect::<Vec<String>>(); // Get command line arguments
    if args.len() != 2 {
        println!("Usage: {} <nightscout-url>", args[0]);
        process::exit(1);
    }
    let url;
    {
        let aditional;
        if args[1].ends_with("/") { aditional = "api/v1/entries/sgv?count=1" } else { aditional = "/api/v1/entries/sgv?count=1" }
        if args[1].starts_with("https://") || args[1].starts_with("http://") {
            url = format!("{}{}", args[1], aditional);
        } else {
            url = format!("https://{}{}", args[1], aditional);
        }
    }
    let resp = reqwest::get(url.to_string()).await.expect("REASON").text().await.unwrap();
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
        Arrow::DoubleUp => {arrow = "⇈";},
        Arrow::SingleUp => {arrow = "↑";},
        Arrow::FortyFiveUp => {arrow = "↗";},
        Arrow::Flat => {arrow = "→";},
        Arrow::FortyFiveDown => { arrow = "↘";}
        Arrow::SingleDown => {arrow = "↓";}
        Arrow::DoubleDown => {arrow = "⇊";}
        Arrow::NotComputable => {arrow = "-";}
        Arrow::RateOutOfRange => {arrow = "-";}
    }

    println!("{} {}", arrow, measurement);

}
