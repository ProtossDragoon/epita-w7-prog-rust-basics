use divisor_sum::divisor_sum;

fn main()
{
    for i in 1..=100000 {
        if is_perfect_number(i) {
            println!("{}", i);
        }
    }
}

fn is_perfect_number(n: u64) -> bool
{
    n == divisor_sum(n)
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_is_perfect_number() {
        assert_eq!(is_perfect_number(1), false);
        assert_eq!(is_perfect_number(2), false);
        assert_eq!(is_perfect_number(5), false);
        assert_eq!(is_perfect_number(6), true);
        assert_eq!(is_perfect_number(495), false);
        assert_eq!(is_perfect_number(496), true);
        assert_eq!(is_perfect_number(8128), true);
        assert_eq!(is_perfect_number(33550336), true);
    }
}

