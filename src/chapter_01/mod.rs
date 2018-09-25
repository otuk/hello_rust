/*!

Variables and basic data types in rust
---

integer types:  i8, i16, i32, i64, isize

unsigned types: u8, u16, u32, u64, usize

floating point: f32, f64

unicode character:  char

boolean type: bool

simple string str 

*/

/**

Each chapter will have a run function that will be called from main.rs.

These function will execute sample code relevant to each chapter.

# Arguments
None

# Remarks
This function will introduce some variables.  Using ```let x = 2``` is a 
simple way to create a variable on stack.  Rust infers the local variables types, but you can also specify.

All variables are immutable by default.  To have a variable that you can 
change you have to use _mut_ keyword. 
 */
pub fn run() {
    let x = 2;  // this makes x an i32
    assert_eq!(x, 2);

    let y: i64 = 2;
    assert_eq!( y, x);

    let d = 3.2; // f64
    let f: f32 = 3.2;
    let truth = true;
    assert_eq!(d == f, truth);    

    let msg = "message"; // msg is of type &str
    assert_eq!("message", msg);
    

    let mut x = 3;
    x += 1;
    assert_eq!(x, 4);
}
