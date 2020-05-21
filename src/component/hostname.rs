pub fn display() -> Option<String> {
    gethostname::gethostname().into_string().ok()
}
