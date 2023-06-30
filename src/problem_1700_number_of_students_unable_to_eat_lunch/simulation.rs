pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut counts = [0, 0];

        for student in students {
            counts[(student as u32 as usize) & 1] += 1;
        }

        for sandwich in sandwiches {
            let sandwich = sandwich as u32 as usize & 1;
            let count = &mut counts[sandwich];

            if *count == 0 {
                return counts[1 - sandwich];
            }

            *count -= 1;
        }

        0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        Self::count_students(students, sandwiches)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
