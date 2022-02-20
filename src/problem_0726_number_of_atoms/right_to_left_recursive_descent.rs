pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    fn number(mut input: &[u8]) -> (u32, &[u8]) {
        let mut result = 0;
        let mut base = 1;

        while let Some((last @ b'0'..=b'9', rest)) = input.split_last() {
            result += u32::from(last - b'0') * base;
            base *= 10;

            input = rest;
        }

        if result == 0 {
            result = 1;
        }

        (result, input)
    }

    fn atom(input: &[u8]) -> Option<(&[u8], &[u8])> {
        let i = input.iter().rposition(|c| !c.is_ascii_lowercase())?;

        input[i].is_ascii_uppercase().then(|| {
            let (rest, result) = input.split_at(i);

            (result, rest)
        })
    }

    fn group<'a>(mut input: &'a [u8], factor: u32, counts: &mut HashMap<&'a [u8], u32>) -> Option<&'a [u8]> {
        input = input.split_last().filter(|(&c, _)| c == b')')?.1;
        input = Self::formula(input, factor, counts).unwrap();

        Some(input.split_last().unwrap().1)
    }

    fn item<'a>(input: &'a [u8], factor: u32, counts: &mut HashMap<&'a [u8], u32>) -> Option<&'a [u8]> {
        match Self::atom(input) {
            None => Self::group(input, factor, counts),
            Some((name, next_input)) => {
                counts.entry(name).and_modify(|c| *c += factor).or_insert(factor);
                Some(next_input)
            }
        }
    }

    fn formula<'a>(mut input: &'a [u8], factor: u32, counts: &mut HashMap<&'a [u8], u32>) -> Option<&'a [u8]> {
        loop {
            let (count, next_input) = Self::number(input);

            if let Some(next_input) = Self::item(next_input, count * factor, counts) {
                input = next_input;
            } else {
                return Some(next_input);
            }
        }
    }

    pub fn count_of_atoms(formula: String) -> String {
        use std::io::Write;

        let mut counts = HashMap::new();

        Self::formula(formula.as_bytes(), 1, &mut counts).unwrap();

        let mut counts = counts.into_iter().collect::<Vec<_>>();

        counts.sort_unstable_by_key(|&(name, _)| name);

        let mut result = Vec::new();

        for (name, count) in counts {
            result.extend(name);

            if count > 1 {
                write!(&mut result, "{}", count).unwrap();
            }
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_of_atoms(formula: String) -> String {
        Self::count_of_atoms(formula)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
