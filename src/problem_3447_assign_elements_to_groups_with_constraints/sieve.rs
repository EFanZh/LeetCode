pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn assign_elements(groups: Vec<i32>, elements: Vec<i32>) -> Vec<i32> {
        let mut groups = groups;
        let max_group = groups.iter().fold(0, |max, group| group.cast_unsigned().max(max));
        let mut indices = vec![-1; max_group as usize + 1].into_boxed_slice();

        (0..).zip(elements).for_each(|(i, element)| {
            let element = element.cast_unsigned() as usize;
            let mut iter = indices.iter_mut().skip(element).step_by(element);

            if let Some(index @ -1) = iter.next() {
                *index = i;

                iter.for_each(|index| {
                    if *index == -1 {
                        *index = i;
                    }
                });
            }
        });

        for group in &mut groups {
            *group = indices[group.cast_unsigned() as usize];
        }

        groups
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn assign_elements(groups: Vec<i32>, elements: Vec<i32>) -> Vec<i32> {
        Self::assign_elements(groups, elements)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
