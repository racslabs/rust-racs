pub mod socket;
pub mod pack;
mod frame;
mod command;
mod utils;
mod pipeline;
mod client;

#[cfg(test)]
mod tests {
    use crate::client::Client;

    #[test]
    fn it_works() {

        let client = Client::new("127.0.0.1:6381");
        let mut pipeline = client.pipeline();
        let result = pipeline.list("*").execute();
        assert!(result.is_ok());

    }
}
