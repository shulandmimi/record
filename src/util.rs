use base64ct::{Base64, Encoding};
use sha1::{Digest, Sha1};

pub fn hash(data: &String) -> String {
    let mut sh = Sha1::new();

    sh.update(data);

    let result = sh.finalize();

    return Base64::encode_string(&result);
}
