// Operating on structs

// 4. You can make a whole struct mutable when instantiating it, but Rust doesn't allow us to mark only certain fields as mutable.


// Fill the blank and fix the error without adding/removing new line
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };

    // how can you believe sunface is only 18? 
    p.age = 30;

    p.name = String::from("sunfei");

    println!("Success!");
}
