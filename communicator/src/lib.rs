#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::client::connect();
        assert_eq!(2 + 2, 4);
    }
}

pub mod client;
mod network;

