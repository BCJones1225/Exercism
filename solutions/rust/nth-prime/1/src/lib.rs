pub fn nth(n: u32) -> u32 {
    let mut vec: Vec<u32> = Vec::with_capacity((n + 1) as usize);

    for j in 2..(n * n + 3) {
        if vec.len() < vec.capacity() && u32::is_prime(j) {
            vec.push(j);
        }
    }

    println!("What is the 0-indexed {}th prime number?", n);
    println!("{}", vec[n as usize]);

    vec[n as usize]
}

trait Prime {
    fn is_prime(num: u32) -> bool;
}

impl Prime for u32 {
    fn is_prime(num: u32) -> bool {
        match num {
            1 => false,
            n if n % 2 == 0 => n == 2,
            _ => {
                let limit = ((num as f32).sqrt()) as u32;
                (3..=limit).all(|i| num % i != 0)
            }
        }
    }
}
