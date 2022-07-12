pub fn run() {
    assert_eq!(factorial(5), 120);

    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(6), 8);

    let mut v = gen_primes(1);
    assert_eq!(v.len(), 0);
    v = gen_primes(2);
    assert_eq!(v[0], 2);
    v = gen_primes(100);
    assert_eq!(v[0], 2);
    assert_eq!(v[1], 3);
    assert_eq!(v[13], 43);
    assert_eq!(v[24], 97);

    for _ in 1..20 {
        let i = random_int(1, 10);
        assert!(1 <= i && i <= 10);
    }
}

fn factorial(n: u32) -> u32 {
    let mut rmul = 1;
    for i in 2..n + 1 {
        rmul = i * rmul;
    }
    rmul
}

/**
 * Generate and return the nth fibonacci number,
 * starting with 0 and 1 as the 0th and 1st fibonacci numbers.
 * # Return
 * the nth fibonacci number as u32
 * # Parameter
 * n:  which fibonacci number(F_n) to return starting from 0 as the F_0(0th fibonacci number)
*/
fn fibonacci(n: u8) -> u32 {
    let (mut a, mut b) = (0, 1);
    let mut count: u8 = 1;
    if n == 0 {
        a
    } else if n == 1 {
        b
    } else {
        while count < n {
            (a, b) = (b, a + b);
            count += 1;
        }
        b
    }
}

/**
 * Generate and return the primes up to the number n
 * # Return
 * a vector containing all the primes until n and including n
 * # Parameter
 * n:  the primes will be searched for up to n
 * */
fn gen_primes(n: u8) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();
    match n {
        0 | 1 => {}
        2 => {primes.push(2);}
        3.. => {
            primes.push(2);
            let mut is_p: bool;
            for i in 3..n+1 {
                is_p = true;
                for p in &primes {
                    if (i as u32) % p == 0 {
                        is_p = false;
                        break;
                    }
                }
                if is_p {
                    primes.push(i as u32);
                }
            }
        }
    }
    primes
}


extern crate rand;
use self::rand::{thread_rng, Rng};
fn random_int(a: u32, b: u32) -> u32 {
    return thread_rng().gen_range(a..b + 1);
}
