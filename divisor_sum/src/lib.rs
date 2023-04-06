pub fn divisor_sum(n: u64) -> u64
{
    if n == 1 {
        return 0;
    }
    let mut sum: u64 = 1;
    let mut leftover: u64 = n;
    for prime in 2..=n {
        let mut term: u64 = 1;
        let mut sum_terms: u64 = 1;
        while leftover % prime == 0 {
            leftover /= prime;
            term *= prime; 
            sum_terms += term;
        }
        sum *= sum_terms;
        if leftover == 1 {
            break;
        }
    }
    sum - n
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_divisor_sum()
    {
        assert_eq!(divisor_sum(1), 0);
        assert_eq!(divisor_sum(2), 1);
        assert_eq!(divisor_sum(3), 1);
        assert_eq!(divisor_sum(4), 3);
        assert_eq!(divisor_sum(6), 6);
        assert_eq!(divisor_sum(30), 42);
        assert_eq!(divisor_sum(496), 496);
        assert_eq!(divisor_sum(8589869056), 8589869056);
        assert_eq!(divisor_sum(137438691328), 137438691328);
    }
}

