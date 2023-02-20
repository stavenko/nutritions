use core::fmt;
use enum_iterator::Sequence;
use serde::Deserialize;
use std::{collections::HashMap, fmt::Write};

#[derive(Deserialize)]
pub struct Recipe {
    products: Vec<Product>,
    dish: Dish,
}

#[derive(Deserialize)]
pub struct Dish {
    ingredients: Vec<Ingredient>,
    weight: Option<f64>,
}

#[derive(Deserialize)]
pub struct Ingredient {
    product: String,
    amount: f64,
}

#[derive(Deserialize)]
pub struct Product {
    name: String,
    facts: HashMap<Nutrition, f64>,
}

#[derive(Deserialize, PartialEq, Eq, Hash, Sequence, Debug, Clone, Copy)]
pub enum Nutrition {
    Energy,
    Proteins,
    Fats,
    Carbohydrates,
}

#[derive(Default, Debug, PartialEq)]
pub struct NutritionFacts(HashMap<Nutrition, f64>);

impl fmt::Display for NutritionFacts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in enum_iterator::all::<Nutrition>() {
            if let Some(value) = self.0.get(&item) {
                write!(f, "{:?}:  {:.2}\n", item, value)?;
            }
        }
        Ok(())
    }
}

impl Recipe {
    pub fn get_nutrition_facts(&self) -> NutritionFacts {
        let mut totals_for_dish: HashMap<Nutrition, f64> = HashMap::new();
        let mut total_ingredients_weight = 0.0;
        for ingredient in &self.dish.ingredients {
            if let Some(product) = self.products.iter().find(|p| p.name == ingredient.product) {
                total_ingredients_weight += ingredient.amount;
                for (nutrient, amount) in &product.facts {
                    let this_amount = amount/100.0 * ingredient.amount;
                    println!("add {} {:?} {:?} {}g = {}",product.name, nutrient, amount/100.0, ingredient.amount, this_amount);
                    totals_for_dish
                        .entry(*nutrient)
                        .and_modify(|v| *v += this_amount)
                        .or_insert(this_amount);
                }
            } else {
                panic!(
                    "Cannot find ingredient in recipe: {} possible products: {}",
                    ingredient.product,
                    self.products.iter()
                    .map(|p| p.name.clone())
                    .collect::<Vec<_>>()
                    .join(", ")
                )
            }
        }
        println!("Totals for raw ingredients {:?} {}", totals_for_dish, total_ingredients_weight);

        let ing_coef = self.dish.weight.map(|dish_weight| total_ingredients_weight / dish_weight).unwrap_or(1.0);
        println!("{}", ing_coef);

        println!("whole result dish {:?}",    totals_for_dish
                .iter()
                .map(|(k, a)| (k, a * ing_coef))
                .collect::<Vec<_>>());
        let weight_to_hundred = self.dish.weight.map(|dish_weight| 100.0 / dish_weight).unwrap_or(100.0 / total_ingredients_weight);

        NutritionFacts(
            totals_for_dish
                .into_iter()
                .map(|(k, a)| (k, a * ing_coef * weight_to_hundred))
                .collect(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::{Ingredient, Nutrition, NutritionFacts, Product, Recipe};

    #[test]
    fn calculate1() {
        let oil = Product {
            name: "Oil".into(),
            facts: [(Nutrition::Energy, 1000.0)].into_iter().collect(),
        };
        let recipe = Recipe {
            dish: super::Dish {
                ingredients: vec![Ingredient {
                    product: "Oil".into(),
                    amount: 10.0,
                }],
                weight: 20.0,
            },
            products: vec![oil],
        };

        let facts = recipe.get_nutrition_facts();

        assert_eq!(
            facts,
            NutritionFacts([(Nutrition::Energy, 500.0)].into_iter().collect())
        )
    }
    #[test]
    #[should_panic(expected= "Cannot find ingredient in recipe: cabbage possible products: Oil, Milk")]
    fn fail_not_found() {
        let milk = Product {
            name: "Milk".into(),
            facts: [(Nutrition::Energy, 1000.0)].into_iter().collect(),
        };
        let oil = Product {
            name: "Oil".into(),
            facts: [(Nutrition::Energy, 1000.0)].into_iter().collect(),
        };
        let recipe = Recipe {
            dish: super::Dish {
                ingredients: vec![Ingredient {
                    product: "cabbage".into(),
                    amount: 10.0,
                }],
                weight: 20.0,
            },
            products: vec![oil, milk],
        };

        let facts = recipe.get_nutrition_facts();

        assert_eq!(
            facts,
            NutritionFacts([(Nutrition::Energy, 500.0)].into_iter().collect())
        )
    }
}
