use clap::Parser;
use recipe::Recipe;
use tokio::io::AsyncReadExt;
mod recipe;

#[derive(clap::Parser)]
struct Opts {
    #[clap(long, short)]
    recipe_file: String,
}

async fn cli() {
    let opts = Opts::parse();
    let mut file = tokio::fs::File::open(&opts.recipe_file)
        .await
        .unwrap_or_else(|_| panic!("File {} expected to be", opts.recipe_file));
    let mut file_contents: String = "".into();
    file.read_to_string(&mut file_contents).await.unwrap();
    let recipe: Recipe = serde_yaml::from_str(&file_contents).unwrap();
    let facts = recipe.get_nutrition_facts();

    println!("Facts: {}\n{}", opts.recipe_file, facts);
}

#[tokio::main]
async fn main() {
    cli().await;
}
