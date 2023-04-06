fn main()
{
    for i in 0..=25 {
        println!("isqrt({}) = {}", 8 * i, isqrt(8 * i));
    }
}

fn isqrt(n: u64) -> u64
{
    let mut r: u64 = n;
    while r * r > n {
        r = r + n/r;
        r = r/2;
    }
    r
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_isqrt()
    {
        assert_eq!(isqrt(0), 0);
        assert_eq!(isqrt(8), 2);
        assert_eq!(isqrt(64), 8);
        assert_eq!(isqrt(81), 9);
        assert_eq!(isqrt(104), 10);
        assert_eq!(isqrt(112), 10);
        assert_eq!(isqrt(192), 13);
        assert_eq!(isqrt(200), 14);
    }
}

