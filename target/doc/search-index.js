var N = null;var searchIndex = {};
searchIndex["hello_rust"]={"doc":"This is where everything begins.","items":[[5,"main","hello_rust","function fn main() is the entry point a binary executable. This is where your program starts running.  All comments starting with /** on a line of its own for public functions  will be part of your documentation.",N,[[]]],[0,"chapter_00","","",N,N],[5,"print_the_ubiquitous_hw","hello_rust::chapter_00","This function will use the println! macro to print \"Hello, world.\" to screen.",N,[[]]],[0,"chapter_01","hello_rust","The following is how you use another 'crate', a crate is the name for a rust executable or library. 'std' is the identifier for the standard rust crate that comes with rust by default. Any rust project can use the std crate without the need for the import statement which otherwise would  be needed to import a foreign crate : `extern crate std` In the std crate there quite a few parts called modules.  One of them is the std::env module that helps you access the environment of your program.   To use env we need the following line: `use std::env; mod point;` Besides the foreign crates, your project code can have different modules. In the above line you can see how to access the point module from the main.rs file that we are in. It enables access to the public contents of the point.rs file which is in the same directory.",N,N],[5,"testfn","hello_rust::chapter_01","will this be part of documents ?",N,[[],["f64"]]],[5,"test_struct","","Let's create a struct",N,[[]]]],"paths":[]};
initSearch(searchIndex);