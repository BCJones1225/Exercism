pub fn raindrops(n: u32) -> String {
    let mut s = String::new();

    match (n % 3, n % 5, n % 7) {
        (0, 0, 0) => s.push_str("PlingPlangPlong"),
        (0, 0, _) => s.push_str("PlingPlang"),
        (0, _, 0) => s.push_str("PlingPlong"),
        (_, 0, 0) => s.push_str("PlangPlong"),
        (0, _, _) => s.push_str("Pling"),
        (_, 0, _) => s.push_str("Plang"),
        (_, _, 0) => s.push_str("Plong"),
        (_, _, _) => s += &n.to_string(),
    }

    println!("What sound does Raindrop #{} make?", n);
    println!("{}", s);

    s
}
