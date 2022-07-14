// Mutability

// 8. 🌟 Error: Borrow an immutable object as mutable


fn main() {
    //fix error by modifying this line
    let mut s = String::from("hello, ");

    borrow_object(&mut s)
}

fn borrow_object(s: &mut String) {}