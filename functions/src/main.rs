fn main() {
    let y = {
        plus_one(five())
    };
    println!("The Value of y is {}",y);

}

fn five() -> i32 { 
    5
}

fn plus_one( x: i32) -> i32 {
    //A statement without a semi-colon is an expression 
    x+1
}
