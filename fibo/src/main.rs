fn main() {
    for i in 0..=10 {
        println!("fibo({}) = {}", i, fibo(i));
    }
    println!("...");
    for i in 81..=90 {
        println!("fibo({}) = {}", i, fibo(i));
    }
}

fn idx(n: u64) -> usize {
    (n % 3) as usize
}

fn fibo(n: u64) -> u64 {
    let mut f = [0 as u64, 1, 1]; 
    for i in 2..=n {
        f[idx(i)] = f[idx(i-2)] + f[idx(i-1)];
    }
    f[idx(n)]
}   

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fibo() {
        assert_eq!(fibo(0), 0);
        assert_eq!(fibo(1), 1);
        assert_eq!(fibo(2), 1);
        assert_eq!(fibo(3), 2);
        assert_eq!(fibo(4), 3);
        assert_eq!(fibo(90), 2880067194370816120);
    }
}
