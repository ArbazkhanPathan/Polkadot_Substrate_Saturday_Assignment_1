// 2. Members can be extracted from the tuple using indexing.


// Make it work
fn main() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");
 
    println!("Success!");
}
