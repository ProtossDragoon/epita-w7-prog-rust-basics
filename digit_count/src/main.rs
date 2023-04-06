fn main()
{
    for i in 0..=64 {
        let mut inp: u64 = i;
        if i == 0 {
            inp = 0;
        }
        else {
            inp = (1 as u64) << (i - 1);
        }
        println!("digit_count({}) = {}", inp, digit_count(inp));
    }
}

fn digit_count(mut n: u64) -> u8
{
    let mut cnt: u8 = 1;
    while (n / 10) != 0 {
        n /= 10;
        cnt += 1;
    }
    cnt
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_digit_count() {
        assert_eq!(digit_count(0), 1);
        assert_eq!(digit_count(7), 1);
        assert_eq!(digit_count(10), 2);
        assert_eq!(digit_count(70368744177664), 14);
    }
}

