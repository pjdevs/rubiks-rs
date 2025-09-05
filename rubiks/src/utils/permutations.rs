use rand::Rng;
use rand::seq::SliceRandom;

pub type Permutation<const N: usize> = [usize; N];

pub fn random_uniform_permutation<const N: usize, R: Rng>(rng: &mut R) -> Permutation<N> {
    let mut permutation = identity_permutation::<N>();
    permutation.shuffle(rng);
    permutation
}

pub fn identity_permutation<const N: usize>() -> Permutation<N> {
    let mut permutation = [0; N];

    for i in 0..N {
        permutation[i] = i;
    }

    permutation
}

pub fn permutation_parity(permutation: &[usize]) -> bool {
        let mut swap_count = 0;

        for i in 0..permutation.len() {
            for j in (i+1)..permutation.len() {
                if permutation[i] > permutation[j] {
                    swap_count += 1;
                }
            }
        }

        swap_count % 2 == 0
    }