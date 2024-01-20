use sha1::{Digest, Sha1};

pub fn hash(password: &str) -> String {
    let mut hasher = Sha1::new();

    hasher.update(password);

    let result = hasher.finalize();

    format!("{:x}", result)
}
