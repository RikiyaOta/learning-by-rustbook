use rand::distributions::Standard;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    // RNG を初期化する. シード値固定.
    let mut rng = Pcg64Mcg::from_seed([0; 16]);

    let mut v = Vec::with_capacity(n);

    for _ in 0..n {
        // Standard = 一様分布
        v.push(rng.sample(&Standard));
    }

    v
}
