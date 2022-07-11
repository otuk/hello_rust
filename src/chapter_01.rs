pub fn run() {
    let x = 2; // this makes x an i32
    assert_eq!(x, 2);

    let y: i64 = 2;
    assert_eq!(y, x);

    let d = 3.2; // f64
    let f: f32 = 3.2;
    let truth = true;
    assert_eq!(d == f, truth);

    let msg = "message"; // msg is of type &str
    assert_eq!(7, msg.len());
    assert_eq!("MESSAGE", msg.to_uppercase());

    let mut x = 3;
    x += 1;
    assert_eq!(x, 4);

    if truth {
        assert!(truth);
    } else {
        assert!(false);
    }
}