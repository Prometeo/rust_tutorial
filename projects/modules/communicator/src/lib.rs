mod network {
    pub fn connect() {}

    mod client {
        fn connect() {}
    }
}

#[cfg(test)]
mod tests {
    use super::network;
    #[test]
    fn it_works() {
        // ::network::connect(); --> option 1
        // super::network::connect(); --> option 2
        network::connect;
    }
}
