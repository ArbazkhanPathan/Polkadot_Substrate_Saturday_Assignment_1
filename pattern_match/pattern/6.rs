// 6.  Using pattern &mut V to match a mutable reference needs you to be very careful, due to V being a value after matching.


// FIX the error with least changing
// DON'T remove any code line
fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        // The type of value is &mut String
       value => value.push_str(" world!") 
    }
}
