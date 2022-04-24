use num_primes::{Generator, BigUint};

struct PublicKey {
    n: BigUint,
    e: u64,
}

struct PrivateKey {
    n: BigUint,
    d: u64,
}

impl PublicKey {
    pub fn new(n: BigUint, e: u64) -> Self {
        PublicKey {
            n: n,
            e: e,
        }
    }

    pub fn print(&self) {
        println!("Public Key");
        println!("\tN: {}", self.n);
        println!("\te: {}", self.e);
    }
}

impl PrivateKey {
    pub fn new(n: BigUint, d: u64) -> Self {
        PrivateKey {
            n: n,
            d: d
        }
    }

    pub fn print(&self) {
        println!("Private Key");
        println!("\tN: {}", self.n);
        println!("\td: {}", self.d);
    }
}

fn calc_e_d(e: u64, d: u64, t: &BigUint) -> u64 {
    let T: u64 = t.bits().try_into().unwrap();
    (e * d) % T
}

fn find_d_e(t: &BigUint) -> (u64, u64) {
    let mut d: u64 = t.bits().try_into().unwrap();
    let mut e: u64 = 0;

    let one: u64 = 1;

    while calc_e_d(e, d, &t) != one {
        e = e + one;
        d = d - one;
    }
    
    (e, d)
}

fn main() {
    let p = Generator::new_prime(64);
    let q = Generator::new_prime(64);

    println!("Let p = {p}");
    println!("Let q = {q}");

    let N = &p * &q;
    println!("Let N = {N}");

    let one: u64 = 1;

    let T = (p - one) * (q - one);

    println!("Let T = {T}");

    let keys = find_d_e(&T);

    let public_key = PublicKey::new(N.clone(), keys.0);

    let private_key = PrivateKey::new(N.clone(), keys.1);

    public_key.print();
    private_key.print();


}
