pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn cast_unsigned(x: Vec<Vec<i32>>) -> Vec<Vec<u32>> {
        x.into_iter()
            .map(|x| x.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>())
            .collect()
    }

    pub fn max_score(n: i32, k: i32, stay_score: Vec<Vec<i32>>, travel_score: Vec<Vec<i32>>) -> i32 {
        _ = k;

        let n = n.cast_unsigned() as usize;
        let stay_score = Self::cast_unsigned(stay_score);
        let travel_score = Self::cast_unsigned(travel_score);
        let mut buffer = vec![0; n * 2];
        let (mut cache, mut temp) = buffer.split_at_mut(n);

        for stay_scores in stay_score {
            for (i, target) in temp.iter_mut().enumerate() {
                *target = cache
                    .iter()
                    .zip(&travel_score)
                    .enumerate()
                    .map(|(j, (prev, travel_scores))| prev + (if j == i { &stay_scores } else { travel_scores })[i])
                    .fold(0, u32::max);
            }

            mem::swap(&mut cache, &mut temp);
        }

        cache.iter().copied().fold(0, u32::max).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_score(n: i32, k: i32, stay_score: Vec<Vec<i32>>, travel_score: Vec<Vec<i32>>) -> i32 {
        Self::max_score(n, k, stay_score, travel_score)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
