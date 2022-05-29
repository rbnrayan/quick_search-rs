use std::{
    process::exit,
    env,
};
use serde_json::{self, Value};
use quick_search::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: search <search_term>");
        exit(1);
    }
    let search_term = args[1].clone();

    if search_term.is_empty() {
        println!("[error]: can't search an empty string.");
        exit(1);
    }
    let url = format_input_to_url(search_term);

    let data = match make_request(&url) {
        Ok(data) => data,
        Err(e) => {
            println!("[error]: {}", e);
            exit(1);
        }
    };

    let json_data: Value = match serde_json::from_slice(data.as_ref()) {
        Ok(data) => data,
        Err(e) => {
            println!("[error]: Failed to convert the data to json.\n\t{}", e);
            exit(1);
        }
    };

    if json_data["Abstract"].to_string()
        .trim_matches('"')
        .is_empty() 
    {
        if json_data["RelatedTopics"].as_array().unwrap().is_empty() {
            println!("No result and no related topics.");
            exit(0);
        }

        println!("Ambigous search see the related topics:");
        for i in 0..4 {
            if json_data["RelatedTopics"][i]["FirstURL"].is_null() { break; }
            println!(
                "[{}] {}",
                i + 1,
                json_data["RelatedTopics"][i]["FirstURL"].to_string().trim_matches('"'),
            );
        }
        return;
    }
    println!(
        "{}\n\nSee more: {}",
        json_data["Abstract"].to_string().trim_matches('"'),
        json_data["AbstractURL"].to_string().trim_matches('"')
    );
}
