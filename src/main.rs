use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::Path;
use std::io::Write;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input template file (e.g., NNN.frag or NNN_math.js)
    input_file: String,
}

#[derive(Debug)]
struct EachBlock {
    values: Vec<String>,
    start: usize,
    end: usize,
}

fn parse_each_blocks(content: &str) -> Result<Vec<EachBlock>> {
    let re = Regex::new(r"EACH<([^>]+)>").unwrap();
    let mut blocks = Vec::new();

    for cap in re.captures_iter(content) {
        let values: Vec<String> = cap[1]
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        let range = cap.get(0).unwrap();
        blocks.push(EachBlock {
            values,
            start: range.start(),
            end: range.end(),
        });
    }

    Ok(blocks)
}

fn generate_combinations(blocks: &[EachBlock]) -> Vec<Vec<String>> {
    if blocks.is_empty() {
        return vec![vec![]];
    }

    let mut result = Vec::new();
    let first = &blocks[0];
    let rest = &blocks[1..];
    let rest_combinations = generate_combinations(rest);

    for value in &first.values {
        for combination in &rest_combinations {
            let mut new_combination = vec![value.clone()];
            new_combination.extend(combination.clone());
            result.push(new_combination);
        }
    }

    result
}

fn replace_each_blocks(content: &str, values: &[String], blocks: &[EachBlock]) -> String {
    let mut result = content.to_string();
    
    // Replace blocks in reverse order to maintain correct indices
    for (i, block) in blocks.iter().enumerate().rev() {
        result.replace_range(block.start..block.end, &values[i]);
    }

    result
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input_path = Path::new(&args.input_file);
    
    // Read the input file
    let content = fs::read_to_string(input_path)
        .with_context(|| format!("Failed to read file: {}", args.input_file))?;

    // Parse EACH blocks
    let blocks = parse_each_blocks(&content)?;
    
    // Generate all combinations
    let combinations = generate_combinations(&blocks);
    
    // Create log file
    let log_file = "combinations.log";
    let mut log = fs::File::create(log_file)
        .with_context(|| format!("Failed to create log file: {}", log_file))?;

    // Process each combination
    for (index, combination) in combinations.iter().enumerate() {
        // Generate output filename
        let input_filename = input_path.file_name().unwrap().to_str().unwrap();
        let output_filename = format!("{:03}_{}", index, input_filename.replace("NNN", ""));
        
        // Replace EACH blocks with current combination values
        let output_content = replace_each_blocks(&content, combination, &blocks);
        
        // Write output file
        fs::write(&output_filename, output_content)
            .with_context(|| format!("Failed to write file: {}", output_filename))?;
        
        // Log the combination
        writeln!(
            log,
            "{} -> {} {}",
            input_filename,
            output_filename,
            combination.join(", ")
        )?;
    }

    println!("Generated {} combinations. See {} for details.", combinations.len(), log_file);
    Ok(())
}
