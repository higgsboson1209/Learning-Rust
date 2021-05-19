use std::io;
fn main() {
    let mut  v: Vec<i32> = Vec :: new();

    /* Program to take user inputs and add to vectors and print the vectors along with deletion of
     elements from the vector */

    loop {

        println!("Enter 1 for adding element");
        println!("Enter 2 for viewing element at a specific index");
        println!("Enter 3 for viewing the entire array");
        println!("Enter 4 or anything for exiting the program");

        let mut element = String::new();


        io::stdin().read_line(&mut element).expect("Failed to read line"); 

        let element: i32 = match element.trim().parse() {
              Ok(num) => num,
              Err(_) => break,

          };
        match element {

            1 => add_element(&mut v),
            2 => view_element(&mut v),
            3 => println!("{:?}",v),
            _ => std::process::exit(1),
          };
    }
}

fn add_element(v : &mut Vec<i32>  ) { 

        println!("Enter the number you want to push into the array!");
        let mut element = String::new();


        io::stdin().read_line(&mut element).expect("Failed to read line"); 
        let element: i32 = match element.trim().parse() {
              Ok(num) => num,
              Err(_) => return,

          };

        v.push(element);
}

fn view_element(v : & Vec<i32>) { 


        println!("Enter the position at which you want to see the element in our vector!");
        let mut position= String::new();


        io::stdin().read_line(&mut position).expect("Failed to read line"); 
        let position: usize= match position.trim().parse() {
              Ok(num) => num,
              Err(_) => return,

          };
    match v.get(position) {
        Some(third) => println!("The element at position {} is {}",position, third),
        None => println!("There is no element at the {}rd  position",position),
    };
}

