// For
// 3.  The for in construct can be used to iterate through an Iterator, e.g a range a..b

fn main() {
    for n in 1..100 {
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }
 
    println!("Success!");
} 
