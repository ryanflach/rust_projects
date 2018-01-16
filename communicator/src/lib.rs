pub mod client; // All code in Rust is private by default

pub mod network;

#[cfg(test)]
mod tests {
    // use brings the specified into scope
    use super::client; // super goes up one level, :: (i.e. ::client) would start from the root

    #[test]
    fn it_works() {
        client::connect();
    }
}
