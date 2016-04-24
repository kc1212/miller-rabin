#![crate_type = "lib"]
#![crate_name = "miller_rabin"]

extern crate rand;
use rand::Rng;

// https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test#Algorithm_and_running_time
pub fn probably_prime(n: u64, k: u64) -> bool {
    if n < 2 || (n != 2 && n % 2 == 0) {
        return false;
    }

    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d >>= 1;
        s += 1;
    }

    for _ in 0 .. k {
        let a = rand::thread_rng().gen_range::<u64>(1, n);
        let x = powm(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1 .. s - 1 {
            if x == 1 {
                return false
            }
            if x == n - 1 {
                continue;
            }
        }
        return false
    }
    true
}

// https://rosettacode.org/wiki/Modular_exponentiation#Haskell
fn powm_(b: u64, e: u64, m: u64, r: u64) -> u64 {
    if e == 0 {
        return r;
    }

    if e % 2 == 1 {
        return powm_(b * b % m, e / 2, m, r * b % m);
    }

    powm_(b * b % m, e / 2, m, r)
}

fn powm(b: u64, e: u64, m: u64) -> u64 {
    powm_(b, e, m, 1)
}
