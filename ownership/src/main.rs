fn main() {
    let s1 = String::from("Hello");

    let (s2,len) = calculate_length(s1);

    println!("The length of {} is {}" , s2, len);
}
fn calculate_length(s1 : String) -> (String, usize) {
    let length = s1.len();
    
    (s1,length)
}
