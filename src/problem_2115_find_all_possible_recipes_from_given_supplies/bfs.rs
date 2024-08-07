pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::{iter, mem};

impl Solution {
    fn intern<'a>(word_to_id: &mut HashMap<&'a str, u16>, word: &'a str) -> u16 {
        let candidate = word_to_id.len() as _;

        match word_to_id.entry(word) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                entry.insert(candidate);

                candidate
            }
        }
    }

    pub fn find_all_recipes(recipes: Vec<String>, ingredients: Vec<Vec<String>>, supplies: Vec<String>) -> Vec<String> {
        let mut word_to_id = (0..).zip(&recipes).map(|(id, recipe)| (recipe.as_str(), id)).collect();

        let mut in_degrees = ingredients
            .iter()
            .map(|ingredients| ingredients.len() as u8)
            .collect::<Box<_>>();

        let mut graph = Vec::<Vec<u16>>::new();

        for (recipe, ingredients) in (0..).zip(&ingredients) {
            for ingredient in ingredients {
                let ingredient = usize::from(Self::intern(&mut word_to_id, ingredient));

                if let Some(nexts) = graph.get_mut(ingredient) {
                    nexts.push(recipe);
                } else {
                    let extra = ingredient - graph.len();

                    graph.extend(iter::repeat_with(Vec::new).take(extra));
                    graph.push(vec![recipe]);
                }
            }
        }

        let mut result = Vec::new();

        let mut queue = supplies
            .iter()
            .filter_map(|supply| word_to_id.get(supply.as_str()).copied())
            .collect::<VecDeque<_>>();

        drop(word_to_id);

        let mut recipes = recipes;

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let ingredient = usize::from(queue.pop_front().unwrap());

                for &recipe in &graph[ingredient] {
                    let recipe = usize::from(recipe);
                    let in_degree = &mut in_degrees[recipe];

                    if *in_degree == 1 {
                        result.push(mem::take(&mut recipes[recipe]));
                        queue.push_back(recipe as _);
                    } else {
                        *in_degree -= 1;
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_all_recipes(recipes: Vec<String>, ingredients: Vec<Vec<String>>, supplies: Vec<String>) -> Vec<String> {
        Self::find_all_recipes(recipes, ingredients, supplies)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
