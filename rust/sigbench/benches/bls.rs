#![feature(test)]

enum SK {
    MIN_PK(blst::min_pk::SecretKey),
    MIN_SIG(blst::min_sig::SecretKey),
}

enum PK {
    MIN_PK(blst::min_pk::PublicKey),
    MIN_SIG(blst::min_sig::PublicKey),
}

enum SIG {
    MIN_PK(blst::min_pk::Signature),
    MIN_SIG(blst::min_sig::Signature),
}

trait Gen {
    fn key_gen() -> SK;
}

trait Sig {
    fn sign() -> SIG;
}

trait Verf {}

#[cfg(test)]
mod test {
    extern crate test;
    use rand::Rng;
    use test::Bencher;
    const MSG_SIZE: usize = 50;
    const KM_SIZE: usize = 32;

    #[bench]
    fn bls_minsig_sign(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut msg = [0u8; MSG_SIZE];
        let mut km = [0u8; KM_SIZE];
        let alg = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
        rng.fill(&mut msg[..]);
        rng.fill(&mut km[..]);
        let sk = blst::min_sig::SecretKey::key_gen(&km[..], &[]).unwrap();
        // bench min sig
        b.iter(|| sk.sign(&msg, alg, &[]));
    }

    #[bench]
    fn bls_minsig_verify(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut msg = [0u8; MSG_SIZE];
        let mut km = [0u8; KM_SIZE];
        let alg = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
        rng.fill(&mut msg[..]);
        rng.fill(&mut km[..]);
        let sk = blst::min_sig::SecretKey::key_gen(&km[..], &[]).unwrap();
        let sig = sk.sign(&msg, alg, &[]);
        let pk = sk.sk_to_pk();
        // bench min sig
        b.iter(|| sig.verify(true, &msg, alg, &[], &pk, true));
    }
}
