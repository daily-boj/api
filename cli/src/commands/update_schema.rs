use crate::arguments::UpdateSchema;
use colored::*;
use dailyboj_api::generate_schema;

pub fn update_schema(_args: UpdateSchema) {
    println!("Update daily-boj database schema...");
    for res in generate_schema("./resources/schema") {
        match res {
            Ok(path) => {
                println!(
                    "{}",
                    format!("success: {}", path.as_os_str().to_string_lossy()).green()
                );
            }
            Err(e) => {
                println!("{}", format!("failure: {}", e).green());
            }
        }
    }
}
