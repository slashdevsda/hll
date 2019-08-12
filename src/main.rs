
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::convert::TryInto;
use fasthash::{murmur, MurmurHasher};

// log2(64) = 6
// m = 6

// αm = 1 / 2(log 2) = 0.72134
// βm = 1.03896

// const alpha_m: f64 = 0.72134;
const beta_m: f64 = 1.03896;

struct HLL {
    p: f64,
    m: f64,    
    // 65635 registers            
    //set: &'a mut [u8; 65537] // 2**16
    set: std::vec::Vec<u8>
}

impl HLL {
    fn add<T: Hash>(&mut self, item: &T) {
        // "add" an item to the set
        // first, we compute the hash.
        // TODO: this hash is a 64bit. I'm not sure the hashing algorithm here is suitable
        // for HLL but hey, let's see how things are going on.
        //
        // quoting the paper:
        // > All known efficient cardinality estimators rely on randomization, which is ensured
        // > by the use of hash functions.
        // > The elements to be counted belonging to a certain data domain D,
        // > we assume given a hash function, h : D → {0, 1}∞;
        // > that is, we assimilate hashed values to infinite binary strings of {0, 1}∞, or equivalently to real numbers of the unit interval.
        // > […]
        // > We postulate that the hash function has been designed in such a way that the hashed values closely resemble a uniform model of randomness,
        // > namely, bits of hashed values are assumed to be independent and to have each probability [0.5] of occurring.
        //
        // I hope these hash are random enougth because this is pretty important here.

        let h : u64 = hash(item);
        
        
        //letfmost
        let phi_s = leftmost_pos(h) + 1;
        //println!("mask: {:b}", mask);

        let idx = get_first_x_bits(h, self.p as usize);

        
        println!("hash          : {:b}", h);
        println!("first 16 bits : {:?}", idx);
        println!("leftm         : {:?}", phi_s);

        if self.set[idx as usize] < phi_s {
            self.set[idx as usize] = phi_s;
        }
        
    }

    fn estimate(&self) -> f64 {
        // working with self.p bits
        // 65635 registers max

        let alpha_m : f64 = (0.7213 / (1.0 + 1.079 / self.m));

        
        let mut indicator: f64 = 0.0;

        for x in self.set.iter() {
            {
                //println!("x: {:?}", x);
                //println!("indicator: {:?}", indicator);
                //indicator += (2_f64.powf(((1.0 - *x as f64)).into()));
                //indicator += (*x as f64).powf(-1.0);
                indicator += (2_f64.powf(((1.0 - *x as f64))));
            }
        };
        // alpha_m * 16_f64.powi(2) * (indicator as f64) //f64::powi(indicator as f64, -1)
        alpha_m * self.m * self.m * (1.0 / indicator)
    }
}

fn get_first_x_bits(x: u64, pos: usize) -> u64 {
    //x ^ (((x >> pos)) << pos)
    (x >> (64 - pos))// << (64 - 16)
}

fn leftmost_pos(x: u64) -> u8 {
    let mut n = x;

    // n |= n << 32;    
    // n |= n << 16;
    // n |= n << 8;
    // n |= n << 4;
    // n |= n << 2;
    // n |= n << 1;
    // n ^= n >> 1;
    // ((n >> 1) & 0xf) as u8
    for i in 0..63 {
        let mask : u64 = 1 << i;
        let res: u8 = (x & (1 << i)) as u8;
        //println!("hash: {:b}", x);
        //println!("mask: {:b}", mask);
        //println!("res:  {:?}", res);
        if res > 0 {
            return i as u8
        }
    };
    2
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s: MurmurHasher = Default::default();
    t.hash(&mut s);
    s.finish()
}


fn get_rand_string() {


}

fn init_HLL(p: usize) -> HLL {
    HLL{p: p as f64, m: (2 << p) as f64,  set: vec![0; (2 << p)]}
}

fn main() {
    println!("Hello, world!");
    let s = String::from("_sbeul45");
    let s2 = String::from("toto45");
    
    let h = hash(&s);
    println!("-> {:?}", h);
    // println!("-> {:?}", h.to_ne_bytes());
    println!("-> {:b}", h);
    let mut mset = init_HLL(16); // HLL{p: 16.0, set: &mut [0; 65537]};
    mset.add(&s);
    mset.add(&s2);
    let total = 120_000;
    for i in 1..total {
        mset.add(&i)
    };
    // 759 674.17162
    // 759 489.50858
    let estimate = mset.estimate();
    println!("{:?}",    estimate);
    println!("error: {:?}", ((estimate - (total as f64)) / (total as f64)) * 100.0)
    //leftmost_offset(112323482384902384);
        // leftmost bit
        //self.set = &mut [0, 0, 0, 0, 0, 0, 0, 0];
}
