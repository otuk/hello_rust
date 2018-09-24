/*! 
The following is how you use another 'crate', a crate is the name for a rust executable or library.
'std' is the identifier for the standard rust crate that comes with rust by default.
Any rust project can use the std crate without the need for the import statement which otherwise would 
be needed to import a foreign crate :
```
extern crate std
```
In the std crate there quite a few parts called modules.  One of them is the **std::env** module that
helps you access the environment of your program.   To *use* env we need the following line:
```
use std::env;
mod point; 
```
Besides the foreign crates, your project code can have different modules.
In the above line you can see how to access the point module from the main.rs file that we are in.
It enables access to the public contents of the point.rs file which is in the same directory.

As you can see rust comes up a full development environment, even the documentation 
support comes out of the box. And your code documentation becomes an easy to read standard
HTML format that you can share as you please. 

Not only that, but you can even write unit test statements into code comments,
which the cargo test command will execute.

*/

use std::env;
mod point;


/**  
will this be part of documents ?
*/
pub fn testfn() -> f64 {
    let mut x = 1.0;
    x += 1.7;
    println!("formatted output {:.4}", x+1.4);

    let a = 3; let b = 8;
    println!("max of {} and {} is {}", a, b, max(&a, &b));
    println!("min of {} and {} is {}", a, b, min(a, b));


    return x + 3.0;
}


/**
Let's create a struct

# Remarks

 */

pub fn test_struct() {
    let mut p = point::Point::new(2, 1);
    let z = point::Point::new(1,1);
    println!("{:?}", p);
    println!("{:?}", z);
    let r = z;
    println!("{:?}", p);
    println!("distance to origin is {:.2}", p.distance_to_origin());
    println!("distance to self is {}", p.distance(&p));


    // move a point
    p.move_x(1);
    println!("Now p is moved to {:?}", p);
    println!(" p == r? {}", p == r);
    p.move_x(-2);
    println!(" p == r? {}", p == r);
}


fn max(a:&i64, b:&i64) -> i64 {
    return if a > b {
        *a
    } else {
        *b
    }
}

fn min(a:i64, b:i64)->i64{
    return if a < b {
        a
    } else {
        b
    };
}
