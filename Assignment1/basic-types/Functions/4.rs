// Diverging functions

// 4.


fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    never_return_fn()
}

// IMPLEMENT this function
// DON'T change any code else
fn never_return_fn() -> ! {
    unimplemented!()
}

///////

fn never_return_fn() -> ! {
    panic!()
}

///////

fn never_return_fn() -> ! {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}

