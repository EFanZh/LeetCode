pub struct Solution {}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();

        for component in path.split('/') {
            match component {
                "" | "." => {}
                ".." => {
                    stack.pop();
                }
                component => stack.push(component),
            }
        }

        let mut result = String::from("/");
        let mut iter = stack.into_iter();

        if let Some(first) = iter.next() {
            result.push_str(first);

            for component in iter {
                result.push_str("/");
                result.push_str(component);
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn simplify_path(path: String) -> String {
        Self::simplify_path(path)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
