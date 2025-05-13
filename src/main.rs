// Declare the modules we created
mod models;
mod utils;

// Import necessary items
use std::fs;
use utils::parser::parse_plu_text; // Import the parser function

fn main() {
    println!("Attempting to parse PLU data...");

    // Define the path to the data file relative to the project root
    let file_path = "plu_code/src/additional/plu.txt";

    // Read the file content
    let plu_text = match fs::read_to_string(file_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            eprintln!(
                "Please ensure the file exists at the correct location relative to where you run `cargo run`."
            );
            // Check if running from workspace root or plu_code directory
            let alternative_path = "src/additional/plu.txt";
            match fs::read_to_string(alternative_path) {
                Ok(text) => {
                    eprintln!("Trying alternative path '{}'", alternative_path);
                    text
                }
                Err(e2) => {
                    eprintln!("Error reading file '{}': {}", alternative_path, e2);
                    std::process::exit(1); // Exit if file can't be read
                }
            }
        }
    };

    // Call the parser function
    match parse_plu_text(&plu_text) {
        Ok(collection) => {
            println!("Successfully parsed {} PLU items.", collection.items.len());

            // --- Example Usage ---

            // 1. Print the first 3 items (optional)
            println!("\n--- First 3 Parsed Items ---");
            for item in collection.items.iter().take(3) {
                println!("{:?}", item);
            }

            // 2. Find all Apples
            let apples: Vec<_> = collection
                .items
                .iter()
                .filter(|item| {
                    item.category_path
                        .first()
                        .map_or(false, |cat| cat == "Apple")
                })
                .collect();
            println!("\n--- Found {} Apple Varieties ---", apples.len());
            if let Some(first_apple) = apples.first() {
                println!("First Apple Found: {:?}", first_apple);
            }

            // 3. Find item by a specific PLU code
            let plu_to_find = 4098; // Akane, small
            println!("\n--- Searching for PLU {} ---", plu_to_find);
            if let Some(found_item) = collection
                .items
                .iter()
                .find(|item| item.plu_codes.contains(&plu_to_find))
            {
                println!("Found item: {:?}", found_item);
            } else {
                println!("No item found for PLU {}", plu_to_find);
            }

            // 4. Serialize the whole collection to JSON (optional)
            // match serde_json::to_string_pretty(&collection) {
            //    Ok(json) => {
            //        println!("\n--- JSON Output (sample) ---");
            //        // Print only a part of it to avoid flooding the console
            //        let sample_len = std::cmp::min(json.len(), 500);
            //        println!("{}\n...", &json[..sample_len]);
            //        // You could write this 'json' string to a file instead
            //        // fs::write("plu_output.json", json).expect("Unable to write JSON file");
            //        // println!("Full JSON output written to plu_output.json");
            //    }
            //    Err(e) => eprintln!("Failed to serialize to JSON: {}", e),
            // }
        }
        Err(e) => {
            eprintln!("\nError parsing PLU data: {}", e);
        }
    }
}
