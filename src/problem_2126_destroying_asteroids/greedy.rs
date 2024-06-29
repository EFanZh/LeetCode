pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        let mut asteroids = asteroids;

        asteroids.sort_unstable_by_key(|&x| x as u32);

        let mut mass = mass as u32;

        for asteroid in asteroids {
            let asteroid = asteroid as u32;

            if mass < asteroid {
                return false;
            }

            if let Some(next_mass) = mass.checked_add(asteroid) {
                mass = next_mass;
            } else {
                return true;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        Self::asteroids_destroyed(mass, asteroids)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
