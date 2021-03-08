// use tonic::{transport::Server, Request, Response, Status};
// use std::{env, process::exit};
use std::{iter::Iterator, print};
use std::net::SocketAddr;

pub mod mapreduce {
    tonic::include_proto!("mapreduce"); 
}

use mapreduce::map_reduce_client::MapReduceClient;
use mapreduce::{StatusRequest,StatusResponse};

extern crate clap;

use clap::{Arg,App,SubCommand};

// define submodules of master that contain information how to  
mod mr_master;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let validate_sockaddr = |server_ip:String|{
        let result = server_ip.parse::<SocketAddr>();
        match result {
            Ok(addr) => return Ok(()),
            Err(addr) => {
                let ret_string = format!("{} {}",server_ip,"is invalid IP address format");
                // let message = String::from(" is invalid IP addrress format");
                return Err(ret_string)}
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

    // iterate through the list of sockete addresses and connect to the corresponding server; 
    // keep a list of workers. For each file in the folder assign it to a worker. 
    // for each worker



    // for argument in env::args() {
    //     println!("input argume is: {}", argument);
    // }

    // let addr = "http://192.168.50.232:50051";

    // let mut worker = MapReduceClient::connect(addr).await?;
    // let status = worker.get_worker_status(StatusRequest{}).await?;

    // println!("{:?}",status);

    Ok(())
}
