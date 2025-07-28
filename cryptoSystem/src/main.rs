use num_traits::{One};
use num_primes::{BigUint, Generator};
use rand::{self};

fn main() {
    // Initialize a random number generator
    let mut rng = rand::rng();
    
    // Set the bit length for prime generation
    let l = 128;
    
    // Generate first large prime number (p)
    let p = Generator::new_prime(l);
    let q;  // Declare second prime (will be initialized in loop)

    // Generate a different second prime (q) that's not equal to p
    loop {
        let candidate = Generator::new_prime(l);
        if candidate != p {
            q = candidate;
            break;
        }
    }

    // Generate a random number alpha less than q
    let alpha = random_biguint_below(&q, &mut rng);
    
    // Compute public exponent e = 1 + alpha*p
    let e = &BigUint::one() + &alpha * &p;
    
    // Compute modulus N = p*q
    let N = &q * &p;

    // Print public key components
    println!("Public key (e, N):\n{}\n{}\n", e, N);
    println!("Secret key (p):\n{}\n", p);

    // Generate random message (x) and random number (r)
    let x = Generator::new_uint(l);
    let r = Generator::new_uint(l);
    
    // Encrypt: y = (x * e^r) mod N
    let y = (&x * e.modpow(&r, &N)) % &N;
    
    // Decrypt: x' = y mod p
    let decrypted_x = &y % &p;

    // Print original, encrypted, and decrypted messages
    println!("Original message x:\n{}", x);
    println!("Encrypted message y:\n{}", y);
    println!("Decrypted message:\n{}", decrypted_x);
}

/// Generates a random BigUint less than an upper bound
/// 
/// # Arguments
/// * `upper_bound` - The exclusive upper bound for the random number
/// * `rng` - Mutable reference to a random number generator
/// 
/// # Returns
/// A random BigUint in the range [0, upper_bound)
fn random_biguint_below<R: rand::Rng>(upper_bound: &BigUint, rng: &mut R) -> BigUint {
    // Convert upper bound to big-endian bytes
    let mut bytes = upper_bound.to_bytes_be();
    let num_bytes = bytes.len();

    loop {
        // Generate random bytes of same length as upper bound
        let mut random_bytes = vec![0u8; num_bytes];
        rng.fill_bytes(&mut random_bytes);

        // Reject if random number >= upper bound
        if random_bytes > bytes {
            continue;
        }

        // Convert valid random bytes to BigUint
        return BigUint::from_bytes_be(&random_bytes);
    }
}