fn main() {
    //Create a new string

    let data = "initial contents";

    let mut  s = data.to_string();

    //Grow your string
    
    s.push_str(" Bar");
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s = format!("{}",s3);

    println!("{}",s3);

}
