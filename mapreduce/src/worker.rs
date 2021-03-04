use std::vec;
use std::env;

use std::net::SocketAddr;

use tonic::{transport::Server, Request, Response, Status};

pub mod mapreduce {
    tonic::include_proto!("mapreduce"); 
}

use mapreduce::map_reduce_server::MapReduce;
use mapreduce::map_reduce_server::MapReduceServer;
use mapreduce::{StatusRequest,StatusResponse};
use mapreduce::{MapRequest,MapResponse};
// use mapreduce::map_reduce_server::MapReduce::{get_worker_status,map};

#[derive(Debug, Default)]
struct MapReduceInstance {}

#[tonic::async_trait]
impl MapReduce for MapReduceInstance {

    async fn get_worker_status(&self,request: Request<StatusRequest>) -> Result<Response<StatusResponse>,Status> {

        let status = 0;
        let status_response = StatusResponse {status};

        Ok(Response::new(status_response))
    }

    async fn map(&self, request:tonic::Request<MapRequest>) -> Result<Response<MapResponse>,Status> {

        let mut vec_response = Vec::new();
        let val = String::from("abcd");
        vec_response.push(val);

        Ok(Response::new(MapResponse{intermediate:vec_response}))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let server_addr = "192.168.50.232:50051".parse::<SocketAddr>()?;
    let mr_instance = MapReduceInstance::default();

    Server::builder().
        add_service(MapReduceServer::new(mr_instance)).
        serve(server_addr).
        await?;
    
    Ok(())
}