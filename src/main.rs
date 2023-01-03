use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

fn default_item_count() -> u8 {
    1
}

#[derive(Debug, Deserialize, Hash, Eq, PartialEq)]
struct Item {
    item: String,
    #[serde(default = "default_item_count")]
    count: u8,
}

#[derive(Debug, Deserialize, Hash, Eq, PartialEq)]
struct Tag {
    tag: String,
}

#[derive(Debug, Deserialize, Hash, Eq, PartialEq)]
#[serde(untagged)]
enum ItemOrTag {
    Item(Item),
    Tag(Tag),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ItemOrItemName {
    Item(Item),
    ItemName(String),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Ingredient {
    ItemOrTag(ItemOrTag),
    IngredientVec(Vec<Ingredient>),
}

#[derive(Debug, Deserialize)]
struct Recipe {
    #[serde(rename = "type")]
    type_: String,
    key: Option<HashMap<String, Ingredient>>,
    ingredient: Option<Ingredient>,
    ingredients: Option<Vec<Ingredient>>,
    pattern: Option<Vec<String>>,
    result: Option<ItemOrItemName>,
}

fn main() {
    let recipe_files = fs::read_dir("./src/minecraft-data/minecraft/recipes")
        .expect("Could not find recipes directory");

    let recipes: Vec<Recipe> = recipe_files
        .map(|recipe_file| {
            let recipe_file = recipe_file.expect("why would this fail");
            let path = recipe_file.path();
            let contents = fs::read_to_string(&path).expect(
                format!(
                    "Could not find {}",
                    recipe_file.file_name().to_str().expect("what")
                )
                .as_str(),
            );
            serde_json::from_str::<Recipe>(contents.as_str()).expect("Could not parse")
        })
        .collect();

    println!("{}", recipes.len());
}
