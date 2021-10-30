pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let (mut g, mut s) = (g, s);

        g.sort_unstable();
        s.sort_unstable();

        let mut s = s.into_iter();
        let mut result = 0;

        for greed in g {
            loop {
                if let Some(size) = s.next() {
                    if greed <= size {
                        result += 1;

                        break;
                    }
                } else {
                    return result;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        Self::find_content_children(g, s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
