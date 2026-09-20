//! The history of the Earth and the universe, placed on one timeline.
//!
//! Run with:
//!
//! ```sh
//! cargo run -p hyper-calendar --example deep_time --features deep-time
//! ```

use hyper_calendar::hc_deep_time::{geologic, place_megayears_ago, place_years_ago, universe};

fn place_ma(label: &str, megayears_ago: f64, std_dev: f64) {
    let Ok(placement) = place_megayears_ago(megayears_ago, std_dev) else {
        println!("{label:<34} outside the modelled range");
        return;
    };
    let chain: Vec<&str> = placement
        .geologic
        .iter()
        .flatten()
        .map(|interval| interval.name)
        .collect();
    let chain = if chain.is_empty() {
        "(before the chart begins)".to_string()
    } else {
        chain.join(" > ")
    };
    println!("{label:<34} {megayears_ago:>10.4} Ma   {chain}");
}

fn main() {
    println!(
        "Chart: {} — {}",
        geologic::CHART_VERSION,
        geologic::CHART_CITATION
    );
    println!("Cosmology: {}\n", universe::PARAMETER_SET);

    println!("=== Chronology of the universe ===");
    for epoch in universe::EPOCHS {
        match (epoch.start(), epoch.end()) {
            (Ok(start), Ok(end)) => {
                println!("{:<26} {} → {}", epoch.name, start, end);
            }
            _ => println!("{:<26} (unrepresentable)", epoch.name),
        }
    }

    println!("\n=== Dated cosmic events ===");
    for event in universe::EVENTS {
        if let Ok(value) = event.deep_time() {
            println!("{:<26} {value}", event.name);
        }
    }

    println!("\n=== Geological time scale, eons and eras ===");
    for rank in [geologic::GeologicRank::Eon, geologic::GeologicRank::Era] {
        for interval in geologic::intervals(rank) {
            println!(
                "{:<12} {:<14} {:>9.4} – {:>9.4} Ma",
                format!("{:?}", interval.rank),
                interval.name,
                interval.base_ma,
                interval.top_ma
            );
        }
    }

    println!("\n=== Placing particular moments ===");
    place_ma("End-Cretaceous impact", 66.0, 0.05);
    place_ma("First dinosaurs", 233.0, 1.0);
    place_ma("Great Oxidation Event", 2_400.0, 100.0);
    place_ma("Oldest known rocks", 4_030.0, 30.0);
    place_ma("Formation of the Earth", 4_540.0, 50.0);
    place_ma("Formation of the Solar System", 4_567.3, 0.16);

    println!();
    for (label, years, sigma) in [
        ("Last Glacial Maximum", 21_000.0, 1_000.0),
        ("End of the last ice age", 11_700.0, 100.0),
        ("Invention of writing", 5_300.0, 100.0),
        ("Now", 0.0, 0.0),
    ] {
        match place_years_ago(years, sigma) {
            Ok(placement) => {
                let period = placement.archaeological.map_or("—", |period| period.name);
                let geologic = placement
                    .finest_geologic()
                    .map_or("—", |interval| interval.name);
                println!("{label:<34} {years:>9.0} BP   {geologic:<14} {period}");
            }
            Err(_) => println!("{label:<34} outside the modelled range"),
        }
    }
}
