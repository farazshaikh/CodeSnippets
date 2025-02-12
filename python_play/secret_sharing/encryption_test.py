import unittest
from encryption import setup, encrypt, decrypt  # Replace 'your_module' with the actual filename

class TestPairingEncryption(unittest.TestCase):

    def setUp(self):
        """Initialize keys before running tests."""
        self.sk, self.pk = setup()

    def test_encryption_decryption(self):
        """Test that encryption and decryption work correctly."""
        message = 123456789  # Example field element message
        ciphertext = encrypt(self.pk, message)
        decrypted_message = decrypt(self.sk, *ciphertext)
        self.assertEqual(message, decrypted_message, "Decryption should return the original message")

    def test_different_messages(self):
        """Test encryption of different messages produces different ciphertexts."""
        msg1 = 42
        msg2 = 987654321
        ciphertext1 = encrypt(self.pk, msg1)
        ciphertext2 = encrypt(self.pk, msg2)
        self.assertNotEqual(ciphertext1, ciphertext2, "Different messages should produce different ciphertexts")

    def test_randomized_ciphertext(self):
        """Test that the same message encrypts to different ciphertexts due to randomness."""
        message = 123456789
        ciphertext1 = encrypt(self.pk, message)
        ciphertext2 = encrypt(self.pk, message)
        self.assertNotEqual(ciphertext1, ciphertext2, "Encryption should be randomized and produce different ciphertexts")

    def test_invalid_decryption(self):
        """Test that incorrect secret key fails to decrypt correctly."""
        sk_fake, _ = setup()  # Generate a different secret key
        message = 123456789
        ciphertext = encrypt(self.pk, message)
        decrypted_message = decrypt(sk_fake, *ciphertext)
        self.assertNotEqual(message, decrypted_message, "Decryption with incorrect key should fail")

if __name__ == "__main__":
    unittest.main()