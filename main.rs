// unsafe fn dangerous() {}

// pub struct unsafe_struct {
//     pub a: i64,
// }

// impl unsafe_struct {
//     pub unsafe fn print(&mut self) {
//         self.a = 1;
//     }

// }


// unsafe trait Animal {
//     // Static method signature; `Self` refers to the implementor type.
//     fn new(name: &'static str) -> Self;

//     // Instance method signatures; these will return a string.
//     fn name(&self) -> &'static str;
//     fn noise(&self) -> &'static str;

//     // Traits can provide default method definitions.
//     fn talk(&self) {
//         println!("{} says {}", self.name(), self.noise());
//     }
// }

fn main() {
    // let mut num = 5;

    // let r1 = &num as *const i32;
    // let r2 = &mut num as *mut i32;

    // unsafe {
    //     println!("r1 is: {}", *r1);
    //     println!("r2 is: {}", *r2);
    // }
    println!("only this line is old");
}
