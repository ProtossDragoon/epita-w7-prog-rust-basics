fn main()
{
    for i in 0..=90 {
        println!("fibo({}) = {}", i, fibo(i));
    }
}

fn fibo(n: u64) -> u64
{
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut f = [0, 1, 1];
    for i in 0..=(n-2) {
        f[((i + 2) % 3) as usize] = f[(i % 3) as usize] + f[((i + 1) % 3) as usize];
    }
    f[(n % 3) as usize]
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_fibo()
    {
        assert_eq!(fibo(0), 0);
        assert_eq!(fibo(1), 1);
        assert_eq!(fibo(2), 1);
        assert_eq!(fibo(3), 2);
        assert_eq!(fibo(10), 55);
        assert_eq!(fibo(90), 2880067194370816120);
    }
}
