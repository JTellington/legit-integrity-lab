fn main() {
    // Hardcoded DB Creds - Legit will flag this immediately
    let db_pass = "admin:SuperSecretPassword123@prod-db-01.internal";

    println!("Connecting to factory...");
    println!("Target: {}", db_pass);
}
