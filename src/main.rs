/*!
 * This is a crate/module level comment
*/

// This is another comment, you will not see this as part of the documents.
mod chapter_00;
mod chapter_01;
mod chapter_02;
mod chapter_03;

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
