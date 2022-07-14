// 3.All elements in an array can be initialized to the same value at once.


fn main() {
    let list: [i32; 100] = [1; 100];

    assert!(list[0] == 1);
    assert!(list.len() == 100);
    println!("success")
}
