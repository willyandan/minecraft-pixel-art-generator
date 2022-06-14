struct Error {
    name: String,
    code: String,
    message: String,
}

impl Error {
    pub fn new(name: String, code: String, message: String) -> Self {}
}
