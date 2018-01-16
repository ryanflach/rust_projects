mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function(); // OK because fn is pub is mod is outermost
    outermost::middle_secret_function(); // Will not compile due to private fn
    outermost::inside::inner_function(); // Will not compile due to private mod
    outermost::inside::secret_function(); // Will not compile due to private mod
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
