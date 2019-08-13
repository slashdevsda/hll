use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::convert::TryInto;
use fasthash::{murmur2, Murmur2Hasher};


// data structure
struct HLL {
    p: f64,
    m: f64,    
    set: std::vec::Vec<u8>
}

// constructor
fn init_HLL(p: usize) -> HLL {
    // p is the size of the bit-prefix
    // m is the number of registers, 2^p
    HLL{p: p as f64,
        m: (2 << p) as f64,
        set: vec![0; (2 << p)]}
}

impl HLL {
    fn add<T: Hash>(&mut self, item: &T) {
        // "add" an item to the set
        // first, we compute the hash.
        let h : u64 = hash(item);

        // find the leftmost bit _after_ the p firsts bits
        let phi_s = leftmost_pos(h, self.p as usize + 1);
        let idx = get_first_x_bits(h, self.p as usize);

        
        // println!("hash          : {:b}", h);
        // println!("first {:?} bits : {:?}", self.p, idx);
        // println!("leftm         : {:?}", phi_s);

        if self.set[idx as usize] < phi_s {
            self.set[idx as usize] = phi_s;
        }        
    }

    fn estimate(&self) -> f64 {
        // essentially translating the paper here
        let alpha_m : f64 = (0.7213 / (1.0 + 1.079 / self.m));        
        let mut indicator: f64 = 0.0;

        for x in self.set.iter() {
            if *x != 0 {
                indicator += (2_f64.powf(((1.0 - *x as f64))));
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
        let res = (x & (1 << i));
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
    println!("Hello, world!");
    


    let mut mset = init_HLL(16); // HLL{p: 16.0, set: &mut [0; 65537]};

    let mut total = 20_000;
    for i in 1..total {
        mset.add(&i)
    };

    let mut estimate = mset.estimate();
    println!("{:?} ({:?})",    estimate, total);
    println!("error: {:?}", ((estimate - (total as f64)) / (total as f64)) * 100.0);

    
    for j in 1..30 {
    total += j * 10000;
        for i in 1..total {
            mset.add(&i)
        };
        // 759 674.17162
        // 759 489.50858
        estimate = mset.estimate();
        println!("estim. {:?} ({:?} uniques)",    estimate as u64, total);
        println!("error: {:.3} %", ((estimate - (total as f64)) / (total as f64)) * 100.0);
    }        
}
