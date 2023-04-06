use divisor_sum::divisor_sum;

fn main()
{
    let arg = std::env::args().nth(1);
    let arg = match arg
    {
        Some(v) => v,
        None =>
        {
            eprintln!("One parameter is required: an integer greater than 0.");
            std::process::exit(1);
        }
    };

    let n = match u64::from_str_radix(&arg, 10)
    {
        Ok(v) if v > 0 => v,
        _ =>
        {
            eprintln!("The first parameter is not a valid number.");
            std::process::exit(1);
        }
    };

    println!("divisor_sum({}) = {}", n, divisor_sum(n));
}

