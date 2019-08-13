use std::hash::{Hash, Hasher};
use fasthash::{Murmur2Hasher};


// data structure
struct HLL {
    p: f64,
    m: f64,    
    set: std::vec::Vec<u8>
}

// constructor
fn init_hll(p: usize) -> HLL {
    // `p` is the size of the bit-prefix
    // `m` is the number of registers, 2^p
    // `set` is the actual vector where registers are stored
    HLL{p: p as f64,
        m: (2 << p) as f64,
        set: vec![0; 2 << p]}
}

impl HLL {
    fn add<T: Hash>(&mut self, item: &T) {
        // "add" an item to the set
        // first, we compute the hash.
        let h : u64 = hash(item);

        // the firsts `p` bits, interpreted as an usize below, is the registry address
        let idx = get_first_x_bits(h, self.p as usize);

        // find the leftmost bit _after_ the p firsts bits,
        // skip p bits, previously used for address
        let phi_s = leftmost_pos(h, self.p as usize + 1);

        
        // println!("hash          : {:b}", h);
        // println!("first {:?} bits : {:?}", self.p, idx);
        // println!("leftm         : {:?}", phi_s);

        if self.set[idx as usize] < phi_s {
            self.set[idx as usize] = phi_s;
        }        
    }

    fn estimate(&self) -> f64 {
        // essentially translating the paper here
        // https://stefanheule.com/papers/edbt13-hyperloglog.pdf
        let alpha_m : f64 = 0.7213 / (1.0 + 1.079 / self.m);
        let mut indicator: f64 = 0.0;

        for x in self.set.iter() {
            if *x != 0 {
                indicator += 2_f64.powf(1.0 - *x as f64);
            }
        };        
        (alpha_m * self.m * self.m) * (1.0 / indicator)
    }
}

fn get_first_x_bits(x: u64, pos: usize) -> u64 {
    (x >> (64 - pos))
}

fn leftmost_pos(x: u64, start: usize) -> u8 {
    for i in start..63 {
        let res = x & (1 << i);
        if res != 0 {
            return (i - start) as u8
        }
    };
    panic!("We should not be there. It's a zero");
    0
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s: Murmur2Hasher = Default::default();
    t.hash(&mut s);
    s.finish()
}

fn main() {
    let mut mset = init_hll(16); // HLL{p: 16.0, set: &mut [0; 65537]};

    
    for j in 1..300_000_000 {
        mset.add(&j);
        if j % 10000000 == 0 {
            let estimate = mset.estimate();
            println!("estim. {:?} ({:?} uniques)",    estimate as u64, j);
            println!("error: {:.3} %", ((estimate - (j as f64)) / (j as f64)) * 100.0);
        }
    }
}
