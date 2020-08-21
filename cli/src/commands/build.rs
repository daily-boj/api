use crate::arguments::Build;
use colored::*;
use dailyboj_api::{generate, generate_schema};

pub fn build(_args: Build) {
    println!("Generating daily-boj database schema...");
    generate_schema("./resources/schema");
    println!("Generating daily-boj api...");
    let res = generate("./generated");
    for route in res {
        let success = route.success;
        let failure = route.errors.len();
        let whole = success + failure;
        println!(
            "  {} {}",
            format!("/{}", route.name),
            if failure == 0 {
                "success".green().underline().to_string()
            } else if success == 0 {
                "failure".red().underline().to_string()
            } else {
                format!(
                    "{} / {} ({})",
                    format!("success {}", success).green(),
                    format!("failure {}", failure).red(),
                    format!("all {}", whole).yellow()
                )
            }
        );
    }
}
