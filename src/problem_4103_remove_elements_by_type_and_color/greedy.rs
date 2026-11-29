pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_remove_all(types: Vec<i32>, colors: Vec<i32>, group_size: Vec<i32>) -> bool {
        let mut stack = Vec::<(i32, i32, i32)>::new();

        types.into_iter().zip(colors).for_each(|(r#type, color)| {
            let group_size = group_size[r#type.cast_unsigned() as usize];

            if group_size != 1 {
                if let Some(top) = stack.last_mut().filter(|top| top.0 == r#type && top.1 == color) {
                    top.2 += 1;

                    if top.2 == group_size {
                        stack.pop();
                    }
                } else {
                    stack.push((r#type, color, 1));
                }
            }
        });

        stack.is_empty()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_remove_all(types: Vec<i32>, colors: Vec<i32>, group_size: Vec<i32>) -> bool {
        Self::can_remove_all(types, colors, group_size)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
