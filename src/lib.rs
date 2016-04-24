#![crate_type = "lib"]
#![crate_name = "miller_rabin"]

extern crate rand;
use rand::Rng;

// https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test#Algorithm_and_running_time
pub fn probably_prime(n: u64, k: u64) -> bool {
    if n <= 3 {
        if n == 2 || n == 3 {
            return true
        }
        return false;
    }

    let mut d = n - 1;
    let mut r = 0;
    while d % 2 == 0 {
        d >>= 1;
        r += 1;
    }
    assert_eq!(n - 1, 2u64.pow(r) * d);

    'witness: for _ in 0 .. k {
        let a = rand::thread_rng().gen_range::<u64>(2, n - 1);
        let mut x = powm(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0 .. r - 1 {
            x = powm(x, 2, n);
            if x == 1 {
                return false
            }
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false
    }
    true
}

// https://rosettacode.org/wiki/Modular_exponentiation#Haskell
pub fn powm(b: u64, e: u64, m: u64) -> u64 {
    powm_(b, e, m, 1)
}

fn powm_(b: u64, e: u64, m: u64, r: u64) -> u64 {
    if e == 0 {
        return r;
    }

    if e % 2 == 1 {
        return powm_(b * b % m, e / 2, m, r * b % m);
    }

    powm_(b * b % m, e / 2, m, r)
}
