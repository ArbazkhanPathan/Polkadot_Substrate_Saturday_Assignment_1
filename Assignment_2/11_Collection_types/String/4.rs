// UTF-8 & Indexing

// Q4


// FILL in the blank and FIX errors
fn main() {
    let s = String::from("hello, 世界");
    let slice1 = &s[0..1]; //tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(slice1, "h");

    let slice2 = &s[7..10];// tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(slice2, "世");

    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世')
        }
    }

    println!("Success!")
}