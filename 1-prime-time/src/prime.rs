pub fn is_prime(n: i32) -> bool {
    match n {
        0 | 1 => false,
        2 | 3 => true,
        n => {
            for x in 2..=(f32::sqrt(n as f32) as i32) {
                if n % x == 0 {
                    return false;
                }
            }

            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime() {
        let mut primes = vec![];
        for n in 0..20 {
            if is_prime(n) {
                primes.push(n);
            }
        }
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }
}
