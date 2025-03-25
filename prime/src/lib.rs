pub fn gen_prime(n: usize) -> usize {
    let mut count = 0;
    let mut num = 2;

    while count < n {
        if is_prime(num) {
            count += 1;
            if count == n {
                return num;
            }
        }
        num += 1;
    }

    num
}

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_prime_1() {
        assert_eq!(gen_prime(1), 2);
    }

    #[test]
    fn test_gen_prime_5() {
        assert_eq!(gen_prime(5), 11);
    }

    #[test]
    fn test_gen_prime_25() {
        assert_eq!(gen_prime(25), 97);
    }
}
