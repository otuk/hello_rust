pub fn run() {
    assert_eq!(Fraction::gcd(6, 3), 3);
    assert_eq!(Fraction::gcd(1, 3), 1);
    assert_eq!(Fraction::gcd(210, 120), 30);
    assert_eq!(Fraction::gcd(120, 210), 30);
    assert_eq!(Fraction::gcd(240, 160), 80);
    assert_eq!(Fraction::gcd(0, 7), 0);
    assert_eq!(Fraction::gcd(1, 0), 0);

    let mut f1= Fraction{num: 1, denum: 5};
    let mut f2 = Fraction::new(3, 21);
    f1.simplify(); assert_eq!(f1.num == 1 && f1.denum==5, true);
    f2.simplify(); assert_eq!(f2.num == 1 && f2.denum==7, true);
    let f3 = f1 + f2 ;
    assert_eq!(f3.num == 12 && f3.denum==35, true);
    let f4 = f1 + f1 + f2 + f2 ;
    assert_eq!(f4.num == 24 && f4.denum==35, true);
    let f5 = f2 / f1;
    assert_eq!(f5.num == 5 && f5.denum==7, true);

    hopeless_search();
}

type FRN = u32;

#[derive (Copy, Clone)] // needed to be able to add same Fraction more than once in the same expression
struct Fraction {
    num: FRN ,   //numerator
    denum: FRN, //denumerator
}

impl Fraction {

    fn new(num: FRN, denum: FRN)-> Self{
        let c = Self::gcd(num, denum);
        Self { 
            num: num/c, 
            denum: denum/c 
        }
    }

    fn simplify(&mut self) {
        let c = Fraction::gcd(self.num, self.denum);
        (self.num, self.denum) = (self.num/c, self.denum/c);
    }

    fn gcd(mut n: FRN, mut d: FRN) -> FRN {
        if d > n {
            (d, n) = (n, d);
        }     
        while  d != 0 {
            n = n % d;
            match n {  // we could have easily used if statement here
                0 => {break}
                _ => {(d, n) = (n, d)}
            }
        }
        d
    }
}

use std::ops::{Add, Div};
impl Add for Fraction {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut f = Fraction {
            num: (self.num*rhs.denum + self.denum * rhs.num),
            denum: self.denum * rhs.denum
        };
        f.simplify();
        f
    }
}

impl Div for Fraction{
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let mut f = Fraction {
            num: self.num*rhs.denum ,
            denum: self.denum * rhs.num
        };
        f.simplify();
        f
    }
}


fn hopeless_search(){
    //find integer num x such that (x*x + 1) is divisble by 3
    //this is a hopeless search, it can be easily shown no such number exists 
    let mut found = false;
    let mut i: u128 = 0;
    const MAX: u128 = 10_000_000_000;
    while i < MAX {
        if (i*i + 1) % 3 == 0 {
            found = true;
            break;
        }
        i += 1 ;
    }
    if found {
        println!("Number is {}", i );
    }
}
