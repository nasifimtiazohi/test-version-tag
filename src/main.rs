mod newanother;
mod nounsafe;
mod unsafetoremove;

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("this line is getting changed again: r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    println!("only this line is old");
}
