/*!
This is where everything learning **rust** programming _begins_.

This is a rust project called _hello_rust_ that is self-documenting to teach rust programming.
You are now reading an example of rust _crate comment_ that is placed at the top of a 
file called _main.rs_ in the root of the directory of this project.

You can view the source for this file by clicking on **source** on the upper right corner.
In the source code notice the start **\/\*\!** and end markers **\*\/.**  These determine
which comments are crate or module level comments.


You can see below that each module that is part of this crate is listed under the **Modules** header.
You can access each of these chapters by clicking model names,such as **chapter_00**.

Within each module/crate document there will also be additional information about the
functions within that module.  This crate has a single  function called _main_ and
you can see the document about it below under the **Functions** header.

After reading this entire page and the main function details you can continue your rust 
learning journey by clicking on the first module  **chapter_00**
*/

// This is another comment, you will not see this as part of the documents.

mod chapter_00;
mod chapter_01;
mod chapter_02;
mod chapter_03;
/**
function **fn main()** is the entry point to the binary executable.
ie your end goal of a running program.
fn main(){...} is where your program starts running.
All comments starting with **\/\*\*** on a line of its own for public functions
will be part of your documentation.
Now click on the name of the function _main_ to see more details about the main function.

# Arguments
None

You can write details about a functions arguments details using **# Arguments**

# Remarks
  You can write remarks for your functions
  using **# Remarks**

# Examples
You can also write code examples to share using **# Examples**
```
println!("Hello World");  
```

In the main function we will call another function from a module named chapter_00.
To use anything from another module we need to import that module first.
We import the module chapter_00 on line number 15 of the main.rs file.
```
mod chapter_00;
```
*/
pub fn main() {
    // let's call a function in module chapter_00
    println!("* chapter_00");
    chapter_00::print_the_ubiquitous_hw();
    println!("");

    println!("* chapter_01");
    chapter_01::run();
    println!("");

    println!("* chapter_02");
    chapter_02::run();
    println!("");

    println!("* chapter_03");
    chapter_03::run();
    println!("");
}
