fn main() {
    let password = "Admin@123";
    
    // Generate bcrypt hash with cost 12
    match bcrypt::hash(password, 12) {
        Ok(hash) => println!("Hash for '{}': {}", password, hash),
        Err(e) => eprintln!("Error: {}", e),
    }
}
