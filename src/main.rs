/*!  
This is where **everything** _begins_.

What you are reading  is an example of rust _module comment_. 

You can view the source for this file by clicking on **\[src\]** on the upper right corner.
In the source code notice the start **\/\*\!** and end markers **\*\/.**  These determine 
which comments are module comments.  
*/

// This is another comment, you will not see this as part of the documents.

pub mod chapter_00;
pub mod chapter_01;

/**
function **fn main()** is the entry point a binary executable.
This is where your program starts running. 
All comments starting with **\/\*\*** on a line of its own for public functions 
will be part of your documentation.
 
# Arguments
None 

You can write arguments details using **# Arguments**

# Remarks
  You can write remarks for your functions
  using **# Remarks** 

# Examples
You can also write code examples to share using **# Examples**
```
assert_eq!(false);  //TODO this doesnot work as expected
```

In the main function we will call another function from module chapter_00.
To use anything from another module we need to add the following line.
```
mod chapter_00;
```
*/
pub fn main() {
    // let's call a function in module chapter_00
    chapter_00::print_the_ubiquitous_hw();
    chapter_01::run();
}



