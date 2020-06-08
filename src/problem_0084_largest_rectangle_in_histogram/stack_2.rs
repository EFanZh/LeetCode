pub struct Solution {}

use std::mem;

struct NonEmptyStack<T> {
    base: Vec<T>,
    top: T,
}

impl<T> NonEmptyStack<T> {
    fn new(top: T) -> Self {
        Self { base: Vec::new(), top }
    }

    fn push(&mut self, value: T) {
        self.base.push(mem::replace(&mut self.top, value));
    }

    fn pop(&mut self) -> Option<T> {
        self.base.pop().map(|new_top| mem::replace(&mut self.top, new_top))
    }
}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut stack = NonEmptyStack::new((-1, 0));

        for item in (0..).zip(heights) {
            loop {
                if item.1 <= stack.top.1 {
                    if let Some((_, height)) = stack.pop() {
                        result = result.max((item.0 - stack.top.0 - 1) * height);
                    } else {
                        result = result.max(item.0 * stack.top.1);
                        stack.top = item;

                        break;
                    }
                } else {
                    stack.push(item);

                    break;
                }
            }
        }

        let right = stack.top.0;

        while let Some(top) = stack.pop() {
            result = result.max((right - stack.top.0) * top.1);
        }

        result
    }
}

impl super::Solution for Solution {
    fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        Self::largest_rectangle_area(heights)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
