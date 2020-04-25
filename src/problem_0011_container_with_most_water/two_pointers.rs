pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let (mut head, rest) = height.split_first().unwrap();
        let (mut tail, mut body) = rest.split_last().unwrap();
        let mut length = rest.len() as i32;

        loop {
            if head < tail {
                result = result.max(length * head);

                if let Some((new_head, new_body)) = body.split_first() {
                    head = new_head;
                    body = new_body;
                } else {
                    break;
                }
            } else {
                result = result.max(length * tail);

                if let Some((new_tail, new_body)) = body.split_last() {
                    tail = new_tail;
                    body = new_body;
                } else {
                    break;
                }
            }

            length -= 1;
        }

        result
    }
}

impl super::Solution for Solution {
    fn max_area(height: Vec<i32>) -> i32 {
        Self::max_area(height)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
