pub fn run() {
    assert_eq!(factorial(5), 120);
    assert_eq!(fibonacci(6)[5], 5);
}

fn factorial(n: u32) -> u32 {
    let mut rmul = 1;
    for i in 2..n + 1 {
        rmul = i * rmul;
    }
    rmul
}


fn fibonacci(n: u32) -> Vec<u32>{
    let mut fibl = vec![0, 1];
    while (fibl.len() as u32) < n {
        let l = fibl.len();
        fibl.push(fibl[l-1]+fibl[l-2]);
    }
    fibl
}




