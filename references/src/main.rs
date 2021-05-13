fn main() {
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1,len);

    let mut s= String::from("Hello");

    change(&mut s);

    println!("{}",s);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String) {
    some_string.push_str(", World is appended to the original string s");
}
