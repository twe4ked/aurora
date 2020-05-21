pub fn display() -> Option<String> {
    std::env::var("USER").ok()
}
