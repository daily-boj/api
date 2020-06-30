use anyhow;
use dailyboj_api::generate;

fn main() -> anyhow::Result<()> {
    println!("Generating daily-boj api...");
    generate("./generated")?;
    println!("Success!");
    Ok(())
}
