extern crate ring;

use ring::{hmac, rand};
use ring::rand::SecureRandom;
use ring::error::Unspecified;
// extern crate data_encoding;
use data_encoding::HEXUPPER;

// HMAC = SHA256
// Input = "Sample message for keylen=blocklen"
// Key = 000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F202122232425262728292A2B2C2D2E2F303132333435363738393A3B3C3D3E3F
// Output = 8BB9A1DB9806F20DF7F77B82138C7914D174D59E13DC4D0169C9057B133E1D62

pub fn testHmacSha256() {
    // HMAC = SHA256
    let mut input = "Sample message for keylen=blocklen".as_bytes();
    let mut key = "000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F202122232425262728292A2B2C2D2E2F303132333435363738393A3B3C3D3E3F".as_bytes();
    let re = hmacSha256(hmac::HMAC_SHA256, &key[..], &input[..]);
    match re {
        Ok(se) => {
            info!("{}", HEXUPPER.encode(se.as_ref()))
        }
        _ => {}
    }
}

pub fn hmacSha256(algorithm: hmac::Algorithm,
                  key_value: &[u8],
                  input: &[u8],
) -> Result<Box<[u8]>, Unspecified> {
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);
    let mut signature = hmac::sign(&key, input);
    info!("{:?}", signature);
    Ok(Box::from(signature.as_ref()))
}
