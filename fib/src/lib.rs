use num_bigint::BigUint;
use num_traits::One;

struct Fibonacci {
    sequence: Vec<BigUint>,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            sequence: vec![BigUint::one(), BigUint::one()],
        }
    }

    fn calc_nth(&mut self, n: usize) -> BigUint {
        if n == 0 {
            return BigUint::ZERO;
        }
        if let Some(nth) = self.sequence.get(n - 1) {
            nth.clone()
        } else {
            let nth = self.calc_nth(n - 2) + &self.calc_nth(n - 1);
            self.sequence.push(nth.clone());
            nth
        }
    }
}

pub fn gen_fib(n: usize) -> BigUint {
    Fibonacci::new().calc_nth(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phib_0() {
        assert_eq!(gen_fib(0), BigUint::ZERO);
    }

    #[test]
    fn phib_1() {
        assert_eq!(gen_fib(1), BigUint::one());
    }

    #[test]
    fn phib_3() {
        assert_eq!(
            gen_fib(3),
            BigUint::parse_bytes(b"2", 10).expect("should be number two")
        );
    }

    #[test]
    fn phib_8() {
        assert_eq!(
            gen_fib(8),
            BigUint::parse_bytes(b"21", 10).expect("shuold be correct number")
        );
    }
}
