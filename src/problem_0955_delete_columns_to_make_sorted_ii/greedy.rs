pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn analyze(diffs: &[Box<[Ordering]>], i: usize) -> Ordering {
        let mut result = Ordering::Less;

        for row in diffs {
            match row[i] {
                Ordering::Less => {}
                Ordering::Equal => result = Ordering::Equal,
                Ordering::Greater => return Ordering::Greater,
            }
        }

        result
    }

    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let columns = strs.first().map_or(0, String::len);
        let mut rows = strs.iter().map(String::as_str);
        let mut prev_row = rows.next().unwrap();
        let mut diffs = Vec::with_capacity(rows.len());

        for row in rows {
            diffs.push(
                prev_row
                    .bytes()
                    .zip(row.bytes())
                    .map(|(lhs, rhs)| lhs.cmp(&rhs))
                    .collect::<Box<_>>(),
            );

            prev_row = row;
        }

        let mut result = 0;

        for i in 0..columns {
            match Self::analyze(&diffs, i) {
                Ordering::Less => break,
                Ordering::Equal => diffs.retain(|row| row[i] == Ordering::Equal),
                Ordering::Greater => result += 1,
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_deletion_size(strs: Vec<String>) -> i32 {
        Self::min_deletion_size(strs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
