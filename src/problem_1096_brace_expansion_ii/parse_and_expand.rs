pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::str::Bytes;

#[allow(variant_size_differences)] // Expected.
enum Atom {
    Char(u8),
    Set(Vec<Vec<Self>>),
}

impl Solution {
    fn parse(iter: &mut Bytes) -> (Vec<Atom>, Option<u8>) {
        let mut result = Vec::new();

        loop {
            match iter.next() {
                Some(c @ b'a'..=b'z') => result.push(Atom::Char(c)),
                Some(b'{') => {
                    let mut elements = Vec::new();

                    loop {
                        let (element, tail) = Self::parse(iter);

                        elements.push(element);

                        if tail != Some(b',') {
                            break;
                        }
                    }

                    result.push(Atom::Set(elements));
                }
                tail => return (result, tail),
            }
        }
    }

    fn expand(element: &[Atom], buffer: &mut String, f: &mut dyn FnMut(&mut String)) {
        if let Some((first, rest)) = element.split_first() {
            match first {
                &Atom::Char(c) => {
                    buffer.push(char::from(c));

                    Self::expand(rest, buffer, f);

                    buffer.pop();
                }
                Atom::Set(elements) => {
                    for element in elements {
                        Self::expand(element, buffer, &mut |buffer| Self::expand(rest, buffer, f));
                    }
                }
            }
        } else {
            f(buffer);
        }
    }

    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let expression = Self::parse(&mut expression.bytes()).0;
        let mut result = Vec::new();

        Self::expand(&expression, &mut String::new(), &mut |buffer| {
            result.push(buffer.clone());
        });

        result.sort_unstable();
        result.dedup();

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn brace_expansion_ii(expression: String) -> Vec<String> {
        Self::brace_expansion_ii(expression)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
