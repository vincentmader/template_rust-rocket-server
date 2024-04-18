use bcrypt::{hash_with_salt, HashParts, DEFAULT_COST};
use rand::{thread_rng, Rng};

type Salt = [u8; 16];

pub fn generate_hashed_password_and_salt(password: &str) -> HashParts {
    let salt = generate_salt();
    let hash = generate_hash(password, salt);
    hash
}

fn generate_salt() -> Salt {
    let mut rng = thread_rng();
    let mut salt = [0u8; 16];
    rng.fill(&mut salt);
    salt
}

fn generate_hash(password: &str, salt: Salt) -> HashParts {
    let cost = DEFAULT_COST;
    let hashed_password = hash_with_salt(password, cost, salt).unwrap();
    println!("{:?}", hashed_password);
    hashed_password
}
