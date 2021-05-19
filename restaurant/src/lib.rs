mod front_of_house ;

pub use crate::front_of_house::hosting;
fn server_order() {}
mod back_of_house {
        fn fix_incorrect_order() {
            cook_order();
            super::server_order();

        }
        fn cook_order() {}

        pub struct Breakfast { 

            pub toast: String, 
            seasonal_fruit: String, 
        }

        impl Breakfast {
            pub fn summer(toast: &str) ->Breakfast {
                Breakfast { 
                    toast: String::from(toast),
                    seasonal_fruit: String::from("Mangoes"),
                }
            }
        }
}
pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast= String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    hosting::add_to_waitlist();

}

