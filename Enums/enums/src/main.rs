fn main() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }
    enum Coin {
        Penny, 
        Nickle,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents(coin: &Coin) ->u8 {
        match coin {
            Coin::Penny => { 
                println!("Lucky Penny");
                1
            },
        
            Coin::Nickle =>5,
            Coin::Dime =>10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
        },
    }
    }

    let coin = Coin::Penny;
    let coin2 = Coin::Quarter(UsState::Alabama);

    println!("The Value of The coin is: {}",value_in_cents(&coin2));

}
