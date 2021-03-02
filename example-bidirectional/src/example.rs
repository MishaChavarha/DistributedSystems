
pub mod example {
    use example::greeter_server::{Greeter, GreeterServer};
use example::{HelloReply, HelloRequest};

    tonic::include_proto!("example"); // The string specified here must match the proto package name
  
    
}