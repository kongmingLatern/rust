enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            count += 1;
        }
        Coin::Nickel => println!("Nickel"),
        Coin::Dime => println!("Dime"),
        Coin::Quarter => println!("Quarter"),
    }
}