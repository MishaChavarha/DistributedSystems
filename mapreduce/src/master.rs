// use tonic::{transport::Server, Request, Response, Status};
// use std::{env, process::exit};
use std::iter::Iterator;
use std::net::SocketAddr;

pub mod mapreduce {
    tonic::include_proto!("mapreduce"); 
}

use mapreduce::map_reduce_client::MapReduceClient;
use mapreduce::{StatusRequest,StatusResponse};

extern crate clap;

use clap::{Arg,App,SubCommand};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let validate_sockaddr = |server_ip:String|{
        let result = server_ip.parse::<SocketAddr>();
        match result {
            Ok(addr) => return Ok(()),
            _ => return Err(String::from("one of the values is not valid socket address"))
        };
    };

    let arg_workers = Arg::with_name("workers")
        .short("w")
        .help("list of workers's ip addresses")
        .required(true)
        .multiple(true)
        .use_delimiter(true)
        .value_delimiter(",")
        .min_values(1)
        .validator(validate_sockaddr);
        
    // let arg_input_dir = Arg::with_name("input directory")
    //     .short("i")
    //     .help("directory that contains map reduce files")
    //     .required(true);
    //     // .last(true);
      
    let mr_master = App::new("mapreduce-master")
        .version("0.1")
        .author("Misha C")
        .about("Map Reduce Master")
        .arg(arg_workers)
        // .arg(arg_input_dir)
        .get_matches();

    let mut addrs = mr_master.values_of("workers").unwrap().collect::<Vec<&str>>();
    println!("{:?}",addrs);



    // for argument in env::args() {
    //     println!("input argume is: {}", argument);
    // }

    // let addr = "http://192.168.50.232:50051";

    // let mut worker = MapReduceClient::connect(addr).await?;
    // let status = worker.get_worker_status(StatusRequest{}).await?;

    // println!("{:?}",status);

    Ok(())
}
