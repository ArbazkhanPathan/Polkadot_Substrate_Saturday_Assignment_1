// String index

// 11. You can't use index to access a char in a string, but you can use slice &s1[start..end].


fn main() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; 
    assert_eq!(h, "h");

    let h1 = &s1[3..6];
    assert_eq!(h1, "中");
}
