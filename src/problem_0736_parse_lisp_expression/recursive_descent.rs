pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    fn symbol(input: &str) -> Option<(&str, &str)> {
        let end = input
            .find(|c: char| matches!(c, '(' | ')' | ' '))
            .unwrap_or_else(|| input.len());

        if end == 0 {
            None
        } else {
            Some(input.split_at(end))
        }
    }

    fn application<'a>(
        input: &'a str,
        context: &mut HashMap<&'a str, i32>,
        bindings_buffer: &mut Vec<(&'a str, Option<i32>)>,
    ) -> Option<(i32, &'a str)> {
        let input = input.strip_prefix('(')?;
        let (key, mut input) = Self::symbol(input).unwrap();

        if matches!(key, "add" | "mult") {
            let (lhs, input) = Self::expression(&input[1..], context, bindings_buffer).unwrap();
            let (rhs, input) = Self::expression(&input[1..], context, bindings_buffer).unwrap();

            Some((if key == "add" { lhs + rhs } else { lhs * rhs }, &input[1..]))
        } else {
            let buffer_base = bindings_buffer.len();

            let result = loop {
                let next_input = &input[1..];

                if let Some((symbol, next_input)) = Self::symbol(next_input) {
                    if let Some((value, next_input)) = Self::expression(&next_input[1..], context, bindings_buffer) {
                        bindings_buffer.push((symbol, context.insert(symbol, value)));
                        input = next_input;
                    } else {
                        input = next_input;

                        break symbol.parse().unwrap_or_else(|_| context[symbol]);
                    }
                } else {
                    let (value, next_input) = Self::application(next_input, context, bindings_buffer).unwrap();

                    input = next_input;

                    break value;
                }
            };

            for (key, value) in bindings_buffer.drain(buffer_base..) {
                if let Some(value) = value {
                    context.insert(key, value);
                } else {
                    context.remove(key);
                }
            }

            Some((result, &input[1..]))
        }
    }

    fn expression<'a>(
        input: &'a str,
        context: &mut HashMap<&'a str, i32>,
        bindings_buffer: &mut Vec<(&'a str, Option<i32>)>,
    ) -> Option<(i32, &'a str)> {
        if let Some((symbol, input)) = Self::symbol(input) {
            Some((symbol.parse().unwrap_or_else(|_| context[symbol]), input))
        } else {
            Self::application(input, context, bindings_buffer)
        }
    }

    pub fn evaluate(expression: String) -> i32 {
        Self::expression(&expression, &mut HashMap::new(), &mut Vec::new())
            .unwrap()
            .0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn evaluate(expression: String) -> i32 {
        Self::evaluate(expression)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
