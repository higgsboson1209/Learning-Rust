use std::io;
fn main() {
    let mut a =[3;5];
    loop {
        println!("Please enter an array index.");
        let mut index = String::new();
        io::stdin().read_line(&mut index).expect("Failed to read a line"); 
        let index: usize = index.trim().parse().expect("Index entered was not a number");
        println!("Please enter an value.");
        let mut value= String::new();
        io::stdin().read_line(&mut value).expect("Failed to read a line"); 
        let value: u32= match value.trim().parse(){
            Ok(num) => num,
            Err(_)=> break,
        };
        a[index]=value;

    } 

        println!("The modified array is: {:?}",a);
}

