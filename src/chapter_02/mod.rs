/*!


*/

/**
 */

pub fn run() {
    let s: String = String::from("short string");
    assert_eq!(s, "short string");
    let s2 = move_and_use(s);
    assert_eq!(s2, "short string");
    // s was moved to the function and is no longer available
    // assert_eq!(s, "short string"); // this line will not compile

    borrow_and_use(&s2); // lend only, do not move
    assert_eq!(s2, "short string"); // still ok

    let mut s3 = move_and_use(s2);
    borrow_and_change(&mut s3);
    assert_eq!(s3.len(), 14);
    assert_eq!(s3, "Xshort stringx");

    println!("0x{:04X} of b'{:08b}'", 1000, 2);

    println!("{} {}", ones(5), 2);
    println!("{} {}", ones(1023), 10);
    println!("{} {}", ones(1024), 1);
    println!("{} {}", ones(0), 0);
    println!("{} {}", ones(1), 1);
    println!("{} {}", ones(1234), 5);
    println!("{} {}", ones(2u64.pow(63)), 1);
}

pub fn move_and_use(s: String) -> String {
    {
        s
    }
}

pub fn borrow_and_use(s: &String) {
    assert_eq!(s.capacity(), 12);
}

/**
You can modified mutable borrowed items using safe methods.
As well as modify  with unsafe methods
```
    s.insert_str(0,"X");
    unsafe {
        let bar = s.as_mut_vec();
        bar.push('x' as u8);
    }

```
*/
pub fn borrow_and_change(s: &mut String) {
    s.insert_str(0, "X");
    unsafe {
        let bar = s.as_mut_vec();
        bar.push('x' as u8);
    }
}

fn ones(i: u64) -> u64 {
    let mut s = 0;
    let mut a = i;
    let mut c = 0;
    while s < 64 {
        c += 1 & a;
        a >>= 1;
        s += 1;
    }
    c
}
