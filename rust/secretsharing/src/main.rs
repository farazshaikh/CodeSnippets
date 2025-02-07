use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Zero};
use std::collections::{BTreeMap, HashMap};
use std::ops::Neg;

/// Generate shares of a secret using Shamir's secret sharing.
fn generate_shares(
    secret: BigInt,
    threshold: usize,
    num_shares: usize,
    prime: &BigInt,
) -> BTreeMap<usize, BigInt> {
    let mut rng = rand::thread_rng();
    let mut coefficients = vec![secret.clone()];
    for _ in 1..threshold {
        coefficients.push(rng.gen_bigint_range(&BigInt::zero(), prime));
    }

    let mut shares = BTreeMap::new();
    for x in 1..=num_shares {
        let mut y = BigInt::zero();
        let x_big = BigInt::from(x);

        for (i, coeff) in coefficients.iter().enumerate() {
            let term = (coeff * x_big.modpow(&BigInt::from(i), prime)) % prime;
            y = (y + term) % prime;
        }

        shares.insert(x, y);
    }

    shares
}

/// Reconstruct the secret using Lagrange interpolation with proper normalization.
fn reconstruct_secret_fixed(shares: &BTreeMap<usize, BigInt>, prime: &BigInt) -> BigInt {
    let mut secret = BigInt::zero();

    for (&x_j, y_j) in shares {
        let mut numerator = BigInt::one();
        let mut denominator = BigInt::one();

        for (&x_m, _) in shares {
            if x_j != x_m {
                let x_j_big = BigInt::from(x_j);
                let x_m_big = BigInt::from(x_m);

                // Normalize subtraction results to avoid negative values
                let term_num = (prime + (&x_m_big).neg()) % prime; // Equivalent to `-x_m`
                let term_den = (x_j_big - x_m_big + prime) % prime; // Normalize x_j - x_m

                numerator = (numerator * term_num) % prime;
                denominator = (denominator * term_den) % prime;
            }
        }

        // Compute the modular inverse of the denominator
        let denominator_inv = denominator.modpow(&(prime - BigInt::from(2)), prime);

        // Calculate the Lagrange basis term and update the secret
        let lagrange_basis = (numerator * denominator_inv) % prime;
        secret = (secret + (y_j * lagrange_basis)) % prime;
    }

    // Ensure the secret is positive
    if secret < BigInt::zero() {
        secret += prime;
    }

    secret
}

/// Reconstruct the secret from a set of shares using Lagrange interpolation.
fn reconstruct_secret(shares: &BTreeMap<usize, BigInt>, prime: &BigInt) -> BigInt {
    let mut secret = BigInt::zero();

    for (&x_j, y_j) in shares {
        let mut numerator = BigInt::one();
        let mut denominator = BigInt::one();

        for (&x_m, _) in shares {
            if x_j != x_m {
                let x_j_big = BigInt::from(x_j);
                let x_m_big = BigInt::from(x_m);

                numerator = (numerator * (-x_m_big.clone())) % prime;
                denominator = (denominator * (x_j_big - x_m_big)) % prime;
            }
        }

        // Compute the Lagrange basis polynomial
        let denominator_inv = denominator.modpow(&(prime - BigInt::from(2)), prime);
        let lagrange_basis = (numerator * denominator_inv) % prime;

        // Add the contribution of this share to the secret
        secret = (secret + (y_j * lagrange_basis)) % prime;
    }

    // Ensure the secret is positive
    if secret < BigInt::zero() {
        secret += prime;
    }

    secret
}

fn main() {
    let prime = BigInt::parse_bytes(
        b"208351617316091241234326746312124448251235562226470491514186331217050270460481",
        10,
    )
    .unwrap();
    let secret = BigInt::from(12345);

    let shares = generate_shares(secret.clone(), 9, 10, &prime);
    println!("Generated Shares: {:?}", shares);

    let reconstructed_secret = reconstruct_secret(&shares, &prime);
    println!("Reconstructed Secret: {}", reconstructed_secret);
}
