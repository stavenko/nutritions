use std::{error::Error, path::PathBuf};

use clap::Parser;
use recipe::Recipe;
mod recipe;

#[derive(clap::Parser)]
struct Opts {
    #[clap(long, short)]
    recipe_file: PathBuf,
}

async fn cli() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let recipe = Recipe::read_from_file(&opts.recipe_file).await?;
    let facts = recipe.get_nutrition_facts().await?;

    println!("Facts: {}\n{}", opts.recipe_file.to_string_lossy(), facts);
    Ok(())
}

#[tokio::main]
async fn main() {
    cli().await.unwrap();
}
