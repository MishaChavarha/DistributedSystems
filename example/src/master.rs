use tonic::{transport::Server, Request, Response, Status};

pub mod example {
    tonic::include_proto!("example"); // The string specified here must match the proto package name
}
// use example::greeter_server::GreeterServer;
use example::greeter_server::{Greeter, GreeterServer};
use example::{HelloReply, HelloRequest};


#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = example::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "192.168.50.232:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("Server is running");
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}