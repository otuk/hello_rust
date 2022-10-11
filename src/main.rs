/*!
 * This is a crate/module level comment
*/

// This is another comment, you will not see this as part of the documents.
mod chapter_00;
mod chapter_01;
mod chapter_02;
mod chapter_03;
mod chapter_04;
mod chapter_05;

pub fn main() {
    // let's call a function in module chapter_00
    let mut _b = true;
    
    println!("* chapter_00");
    _b = false;
    if _b {
        chapter_00::run()
    } else {
        println!("skipped")
    };
    println!();

    println!("* chapter_01");
    _b = true;
    if _b {
        chapter_01::run()
    } else {
        println!("skipped")
    };
    println!();

    println!("* chapter_02");
    _b = false;
    if _b {
        chapter_02::run()
    } else {
        println!("skipped")
    };
    println!();

    println!("* chapter_03");
    _b = false;
    if _b {
        chapter_03::run()
    } else {
        println!("skipped")
    };
    println!();

    println!("* chapter_04");
    _b = false;
    if _b {
        chapter_04::run()
    } else {
        println!("skipped")
    };
    println!("");

    println!("* chapter_05");
    _b = false;
    if _b {
        chapter_05::run()
    } else {
        println!("skipped")
    };
    println!("");
}
