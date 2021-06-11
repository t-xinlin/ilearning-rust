use ring::{hmac, rand};
use ring::rand::SecureRandom;
use ring::error::Unspecified;
use data_encoding::HEXUPPER;
use url;

pub fn hmacSha256(algorithm: hmac::Algorithm,
                  key_value: &[u8],
                  input: &[u8],
                  output: &[u8],
) -> Result<Box<[u8]>, Unspecified> {
    let key = hmac::Key::new(algorithm, &key_value);
    let signature = hmac::sign(&key, input);
    println!("signature: {:?}", HEXUPPER.encode(signature.as_ref()));
    println!("verify: {:?}", hmac::verify(&key, input, output).is_ok());
    Ok(Box::from(signature.as_ref()))
}

#[cfg(test)]
mod tests {
    use ring::test::from_hex;

    // use ring::{digest, hmac, test, test_file};
    // use ring::{digest, hmac, test as uni_test, test_file};
    // HMAC = SHA256
    // Input = "Sample message for keylen<blocklen"
    // Key = 000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F
    // Output = A28CF43130EE696A98F14A37678B56BCFCBDD9E5CF69717FECF5480F0EBDF790
    #[test]
    pub fn test_hmac_sha256() {
        use super::*;
        // HMAC = SHA256
        let mut input = b"Sample message for keylen=blocklen";
        let mut key = HEXUPPER.decode(b"000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F202122232425262728292A2B2C2D2E2F303132333435363738393A3B3C3D3E3F").unwrap();
        let mut output = HEXUPPER.decode(b"8BB9A1DB9806F20DF7F77B82138C7914D174D59E13DC4D0169C9057B133E1D62").unwrap();
        let result = hmacSha256(hmac::HMAC_SHA256, &key[..], &input[..], &output[..]);
        match result {
            Ok(out) => {
                println!("{}", HEXUPPER.encode(out.as_ref()))
            }
            _ => {}
        }
    }


    #[test]
    fn hmac_tests() {
        // use ring::{digest, hmac, test, test_file};
        // use ring::{digest, hmac, test as uni_test, test_file};
        // HMAC = SHA256
        // Input = "Sample message for keylen<blocklen"
        // Key = 000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F
        // Output = A28CF43130EE696A98F14A37678B56BCFCBDD9E5CF69717FECF5480F0EBDF790
        use data_encoding::HEXUPPER;
        use sha2::Sha256;
        use hmac::{Hmac, Mac, NewMac};
        use std::str;

        // Create alias for HMAC-SHA256
        type HmacSha256 = Hmac<Sha256>;
        let mut k = b"000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F";

        let mut key_bytes = HEXUPPER.decode(&k[..]);
        let mut mac = HmacSha256::new_from_slice(key_bytes.as_ref().unwrap())
            .expect("Sample message for keylen<blocklen");
        mac.update(b"Sample message for keylen<blocklen");
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        println!("{:?}", HEXUPPER.encode(code_bytes.as_slice()));
    }
}
