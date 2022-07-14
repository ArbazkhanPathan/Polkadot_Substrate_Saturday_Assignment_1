// 1. 🌟 A variable can be used only if it has been initialized.

// Fix the error below with least amount of modification to the code
fn main() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    println!("{} is equal to 5", x);
}
