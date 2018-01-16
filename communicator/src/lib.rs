mod network {
    fn connect() {
    }

    mod client {
        fn connect() { // Called with network::client::connect
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
