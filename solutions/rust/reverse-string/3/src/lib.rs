use unicode_reverse::reverse_grapheme_clusters_in_place;

pub fn reverse(input: &str) -> String {
    //unimplemented!("Write a function to reverse {}", input);
    let mut string = input.to_string();
    println!("{}", string);
    reverse_grapheme_clusters_in_place(string.as_mut_str());
    println!("{}", string);
    string
}
