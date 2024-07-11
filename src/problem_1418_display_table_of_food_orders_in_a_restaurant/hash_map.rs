pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashMap, HashSet};
use std::iter;

impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut tables = Vec::new();
        let mut foods = HashSet::new();

        for order in &orders {
            let [_, table, food]: &[_; 3] = order.as_slice().try_into().unwrap();
            let food = food.as_str();

            foods.insert(food);

            let table = table.parse::<usize>().unwrap() - 1;
            let extra = (table + 1).saturating_sub(tables.len());

            tables.extend(iter::repeat_with(HashMap::new).take(extra));

            let slot = &mut tables[table];

            slot.entry(food).and_modify(|count| *count += 1).or_insert(1_u16);
        }

        let mut foods = foods.into_iter().collect::<Vec<_>>();

        foods.sort_unstable();

        let mut result = Vec::new();

        result.push(
            iter::once("Table")
                .chain(foods.iter().copied())
                .map(str::to_string)
                .collect(),
        );

        for (i, table) in (1_u16..).zip(tables.into_iter()) {
            if !table.is_empty() {
                result.push(
                    iter::once(i)
                        .chain(foods.iter().map(|&food| table.get(food).copied().unwrap_or(0)))
                        .map(|x| x.to_string())
                        .collect(),
                );
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        Self::display_table(orders)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
