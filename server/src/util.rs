use argon2::{
    password_hash::{PasswordHasher, Salt},
    Argon2,
};

pub fn verify_hash(hashed_value: &str, unhashed_value: &str) -> bool {
    let salt = hashed_value.split("$").collect::<Vec<&str>>()[4];

    let pass = Argon2::default()
        .hash_password(unhashed_value.as_bytes(), Salt::from_b64(salt).unwrap())
        .unwrap();

    if pass.to_string() == hashed_value {
        return true;
    }

    return false;
}
