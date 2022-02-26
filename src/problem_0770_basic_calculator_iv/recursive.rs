pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;
use std::{iter, mem};

#[derive(Default)]
struct SymbolMap<'a> {
    index_map: HashMap<&'a str, u8>,
    values: Vec<&'a str>,
}

impl<'a> SymbolMap<'a> {
    fn get_index(&mut self, symbol: &'a str) -> u8 {
        if let Some(&index) = self.index_map.get(symbol) {
            index
        } else {
            let index = self.index_map.len() as _;

            self.index_map.insert(symbol, index);
            self.values.push(symbol);

            index
        }
    }

    fn get_value(&mut self, index: u8) -> &'a str {
        self.values[usize::from(index)]
    }
}

type Item = Vec<(u8, u8)>;

impl Solution {
    fn atom<'a>(symbol_map: &mut SymbolMap<'a>, input: &'a str, values: &[i32]) -> Option<(Result<i32, u8>, &'a str)> {
        let end = input
            .bytes()
            .position(|c| !matches!(c, b'-' | b'0'..=b'9'| b'a'..=b'z'))
            .unwrap_or(input.len());

        if end == 0 {
            None
        } else {
            let (left, right) = input.split_at(end);

            if let Ok(value) = left.parse() {
                Some((Ok(value), right))
            } else {
                let index = symbol_map.get_index(left);

                Some((values.get(usize::from(index)).copied().ok_or(index), right))
            }
        }
    }

    fn factor<'a>(symbol_map: &mut SymbolMap<'a>, input: &'a str, values: &[i32]) -> (HashMap<Item, i32>, &'a str) {
        if let Some((atom, input)) = Self::atom(symbol_map, input, values) {
            let factor = match atom {
                Ok(value) => (Vec::new(), value),
                Err(symbol) => (vec![(symbol, 1)], 1),
            };

            (iter::once(factor).collect(), input)
        } else {
            let (result, input) = Self::expression(symbol_map, &input[1..], values);

            (result, &input[1..])
        }
    }

    fn merge_item(left_item: &[(u8, u8)], right_item: &[(u8, u8)], buffer: &mut Vec<(u8, u8)>) {
        let mut left_iterator = left_item.iter().copied();
        let mut right_iterator = right_item.iter().copied();

        'outer: loop {
            if let Some(mut left) = left_iterator.next() {
                loop {
                    if let Some(right) = right_iterator.next() {
                        loop {
                            match left.0.cmp(&right.0) {
                                Ordering::Less => {
                                    buffer.push(left);

                                    if let Some(next_left) = left_iterator.next() {
                                        left = next_left;
                                    } else {
                                        buffer.push(right);
                                        buffer.extend(right_iterator);

                                        break 'outer;
                                    }
                                }
                                Ordering::Equal => {
                                    buffer.push((left.0, left.1 + right.1));

                                    continue 'outer;
                                }
                                Ordering::Greater => {
                                    buffer.push(right);

                                    break;
                                }
                            }
                        }
                    } else {
                        buffer.push(left);
                        buffer.extend(left_iterator);

                        break 'outer;
                    }
                }
            } else {
                buffer.extend(right_iterator);

                break;
            }
        }
    }

    fn term<'a>(symbol_map: &mut SymbolMap<'a>, input: &'a str, values: &[i32]) -> (HashMap<Item, i32>, &'a str) {
        let (mut result, mut input) = Self::factor(symbol_map, input, values);
        let mut item_buffer = Vec::new();
        let mut result_buffer = HashMap::new();

        while let Some(next_input) = input.strip_prefix(" * ") {
            let (factor, next_input) = Self::factor(symbol_map, next_input, values);

            for (left_item, left_count) in &result {
                for (right_item, right_count) in &factor {
                    Self::merge_item(left_item, right_item, &mut item_buffer);

                    let new_count = left_count * right_count;

                    result_buffer
                        .entry(mem::take(&mut item_buffer))
                        .and_modify(|count| *count += new_count)
                        .or_insert(new_count);
                }
            }

            input = next_input;
            mem::swap(&mut result, &mut result_buffer);
            result_buffer.clear();
        }

        (result, input)
    }

    fn expression<'a>(symbol_map: &mut SymbolMap<'a>, input: &'a str, values: &[i32]) -> (HashMap<Item, i32>, &'a str) {
        let (mut result, mut input) = Self::term(symbol_map, input, values);

        loop {
            if let Some(next_input) = input.strip_prefix(" + ") {
                let (term, next_input) = Self::term(symbol_map, next_input, values);

                for (item, new_count) in term {
                    result
                        .entry(item)
                        .and_modify(|count| *count += new_count)
                        .or_insert(new_count);
                }

                input = next_input;
            } else if let Some(next_input) = input.strip_prefix(" - ") {
                let (term, next_input) = Self::term(symbol_map, next_input, values);

                for (item, new_count) in term {
                    result
                        .entry(item)
                        .and_modify(|count| *count -= new_count)
                        .or_insert(-new_count);
                }

                input = next_input;
            } else {
                break;
            };
        }

        (result, input)
    }

    pub fn basic_calculator_iv(expression: String, evalvars: Vec<String>, evalints: Vec<i32>) -> Vec<String> {
        let mut symbol_map = SymbolMap::default();

        for symbol in &evalvars {
            symbol_map.get_index(symbol);
        }

        let mut result = Self::expression(&mut symbol_map, &expression, &evalints)
            .0
            .into_iter()
            .filter_map(|(item, count)| {
                if count == 0 {
                    None
                } else {
                    let mut result = item
                        .into_iter()
                        .map(|(index, count)| (symbol_map.get_value(index), count))
                        .collect::<Vec<_>>();

                    result.sort_unstable_by_key(|&(symbol, _)| symbol);

                    let degrees = result.iter().map(|&(_, count)| count).sum::<u8>();

                    Some((result, count, degrees))
                }
            })
            .collect::<Vec<_>>();

        result.sort_unstable_by(|(lhs_item, _, lhs_degrees), (rhs_item, _, rhs_degrees)| {
            rhs_degrees.cmp(lhs_degrees).then_with(|| {
                let key_fn = |&(symbol, count)| (symbol, Reverse(count));
                let lhs_it = lhs_item.iter().map(key_fn);
                let rhs_it = rhs_item.iter().map(key_fn);

                lhs_it.cmp(rhs_it)
            })
        });

        result
            .into_iter()
            .map(|(item, count, _)| {
                let mut result = count.to_string();

                for (symbol, count) in item {
                    for _ in 0..count {
                        result.push('*');
                        result.push_str(symbol);
                    }
                }

                result
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn basic_calculator_iv(expression: String, evalvars: Vec<String>, evalints: Vec<i32>) -> Vec<String> {
        Self::basic_calculator_iv(expression, evalvars, evalints)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
