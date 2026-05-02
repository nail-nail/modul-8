use tonic::transport::Server;

pub mod services {
    tonic::include_proto!("services");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _addr = "[::1]:50051".parse()?;
    let _server = Server::builder();

    Ok(())
}
