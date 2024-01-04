pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let mut seats = seats;
        let mut students = students;

        seats.sort_unstable();
        students.sort_unstable();

        let mut result = 0;

        for (seat, student) in seats.into_iter().zip(students) {
            result += seat.abs_diff(student);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        Self::min_moves_to_seat(seats, students)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
