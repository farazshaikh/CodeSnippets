from py_ecc.bls12_381 import G1, G2, multiply, add, pairing
from py_ecc.bls import G2ProofOfPossession as bls
import unittest
import os

"""
The encryption scheme described in the paper has these key characteristics:
1.  Type III Pairing-Based Encryption:
    Uses three groups G1,G2,GT with a bilinear map:
        e:G1xG2→GT

2.  Public-Key Setup:
    The paper describes a public key of the form:
    pk=g2^x

3. Encryption Structure:
    The ciphertext in the paper consists of:
    C1=g1^r, C2=e(g1,pk)^r x m

4. Decryption Structure:
    m=C2/ e(C1, g2^x)

TODO:
    Basic Encryption	    ✅ Yes	✅ Yes (Same Formula)
    Pairing-Based Security	✅ Yes	✅ Yes
    Multi-Recipient Support	✅ Yes	❌ Not Yet
    Forward Secrecy     	✅ Yes	❌ Not Yet
    Proof chunking      	✅ Yes	❌ Not Yet
    Proof encryption      	✅ Yes	❌ Not Yet
    CCA-Security        	✅ Yes	❌ Not Yet
"""
def setup():
    """Setup phase: Initializes the BLS12-381 curve and generates public/private keys."""
    sk = int.from_bytes(os.urandom(32), 'big')  # Secret key
    pk = multiply(G2, sk)  # Public key in G2
    return sk, pk

def encrypt(pk, m):
    """Encryption function using BLS12-381 curve."""
    r = int.from_bytes(os.urandom(32), 'big')  # Random exponent
    C1 = multiply(G1, r)  # First part of ciphertext in G1
    C2 = pairing(pk, G1) ** r * m  # Second part of ciphertext in GT
    return (C1, C2)

def decrypt(sk, C1, C2):
    """Decryption function as described in the paper."""
    e_C1_sk = pairing(multiply(G2, sk), C1)  # Compute e(C1, G2^sk)
    m = (C2 / e_C1_sk).coeffs[0]  # Recover original message as integer
    return m

if __name__ == "__main__":
    sk, pk = setup()
    m = 123456789  # Example field element message
    ciphertext = encrypt(pk, m)
    decrypted_m = decrypt(sk, *ciphertext)

    print("Original message:", m)
    print("Decrypted message:", decrypted_m)
    print("Decryption successful?", m == decrypted_m)
