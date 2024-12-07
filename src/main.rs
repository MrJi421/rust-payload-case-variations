use std::{io, fs::File, io::Write, time::Instant};

fn main() {
    println!("Enter the payload script:");
    let mut payload = String::new();
    io::stdin().read_line(&mut payload).expect("Failed to read line");
    let payload = payload.trim(); // Remove any extra spaces or newline

    println!("Do you want the payloads encoded (yes/no)[feature is under developement.]?");
    let mut encoded = String::new();
    io::stdin().read_line(&mut encoded).expect("Failed to read line");
    let encoded = encoded.trim().to_lowercase();

    println!("Enter the output file name (always save as .txt):");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).expect("Failed to read line");
    let filename = filename.trim();
    
    let output_file = format!("{}.txt", filename);
    let mut file = File::create(&output_file).expect("Could not create file");

    let variations = generate_case_variations(payload);
    let total_variations = variations.len();
    
    let start_time = Instant::now();
    
    for (index, variation) in variations.iter().enumerate() {
        // Write the variation to the file
        writeln!(file, "{}", variation).expect("Failed to write to file");

        // Show progress
        let elapsed_time = start_time.elapsed().as_secs_f64();
        let estimated_time_remaining = elapsed_time / (index as f64 + 1.0) * (total_variations as f64 - index as f64);

        println!(
            "Progress: {:.2}% | Estimated Time Remaining: {:.2} seconds.",
            (index as f64 + 1.0) / total_variations as f64 * 100.0,
            estimated_time_remaining
        );
    }

    println!("Number of Payloads: {}", total_variations);
    println!("Finished writing payloads to {}", output_file);
}

fn generate_case_variations(payload: &str) -> Vec<String> {
    let mut variations = Vec::new();

    // Generate all case combinations for a given string
    fn generate_case_combinations(input: &str) -> Vec<String> {
        let mut combinations = vec!["".to_string()];
        
        for ch in input.chars() {
            let mut new_combinations = Vec::new();
            for combination in &combinations {
                // Toggle between lowercase and uppercase for each character
                new_combinations.push(combination.clone() + &ch.to_lowercase().to_string());
                new_combinations.push(combination.clone() + &ch.to_uppercase().to_string());
            }
            combinations = new_combinations;
        }
        
        combinations
    }

    // Generate case combinations for <script> and </script>
    let open_tag_variations = generate_case_combinations("script");
    let close_tag_variations = generate_case_combinations("script");
    
    // Combine variations of the opening and closing tags
    for open_tag in open_tag_variations {
        for close_tag in close_tag_variations.clone() {
            let variation = payload
                .replace("<script>", &format!("<{}>", open_tag))
                .replace("</script>", &format!("</{}>", close_tag));
            variations.push(variation);
        }
    }

    variations
}
