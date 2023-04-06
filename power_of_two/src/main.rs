fn main()
{
    for i in 0..64 {
        println!("power_of_two({}) = {}", i, power_of_two(i));
    }
}

fn power_of_two(n: u8) -> u64
{
    if n == 0 {
        return 1;
    }
    (1 as u64) << n 
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_power_of_two()
    {
        assert_eq!(power_of_two(0), 1);
        assert_eq!(power_of_two(1), 2);
        assert_eq!(power_of_two(2), 4);
        assert_eq!(power_of_two(10), 1024);
        assert_eq!(power_of_two(63), 9223372036854775808);
    }
}
