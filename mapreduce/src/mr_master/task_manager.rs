use std::{net::SocketAddr, path::{Path, PathBuf}};
use std::sync::mpsc::{Sender, Receiver};

// task scheduler for map reduce tasks. 
// assigns tasks based on collection of files; 

// monitors worker status.
// assigns work. 

// 1 datastructure - collection of connections protected with mutexes; 
// brief outline of procedure calls.
// first establish a series of connections; 
// two spawn thread; 
//     main thread - assigns map reduce jobs to workers: 
//          
//          pick a map file -> assign a job based on this map file by taking client 
//          from the pool of available clients. If all clients are busy blocks until 
//          new client becomes avaialable; 
//              idea create an iterator?
//          keep track of the map of clients where key is client and value is its status? 
//          is it a good design decision? 
//          for each job spawn a thread that talks to a worker and monitors progress; 
//          when the task is done borrowed client is returned to the thread pool of clients
//     second thread: 
//          
// 2 threads, monitors workers progress and assigns based on available connection.  
// 


// taks manager: executes map reduce jobs; 
//
// first: Some datastructure with a list of active connections. 
// iterator implemented over this datastructure that gives next available Connection. 
// blocks when connection is not available. if all workers are busy, mutexes are locked. 
// then loop goes to sleep and wakes up when task is completed or failed. If map/reduce task is 
// failed it is enqueued into the task of map pools. In both cases task wakes up thread 
// Anwser: blocking queue with a sender channel  
// 
// 
// list of map jobs;
// list of reduce jobs; 
// each map job is moved from the iterator and executed. if it 
// each structure has execute function that takes an


pub enum Job{
    Map(String),
    Reduce(String),
}

pub struct TaskManager{

    

    // pool of ongoing amp and reduce jobs. 
    // when job is completed it is automatically removed from the 
    // job is a tasks, when spawning a future pass a function that is executed when it is completed. 
    // also notify scheduling thread about available 

    // number of map jobs; 
    // number of reduce jobs;
    
    // list of map jobs completed; 
    // perhaps iterator over map jobs 

    // list of reduce jobs completed; 
    // iterator over reduce jobs; 


    // perhaps have a map_job struct with a function assign; 
    // assign spawns an async thread that performs a job. 

    // perhaps i need some kind of structure that contains 


    // so I need some kind of struct that contains a pool of active jobs. 
    // each job when completed ( or failed automatically dequeued from the pool and thread )
    // i think this is the key. 

    // another idea. For number of socket addresses running send a job through a channel. 
    // return value. when value is locked execution takes place so when value is returned 
    // job is updated and channel is unlocked. Each thread executes async task that sends request
    // with a time result is returned when failure occurs. If failure 
    
    // another idea for a design. 
    // Spanws a heartbeat task. Heartbeat task updates progress, results of which are passed to the master task. 
    // How would that work. 
    // Spans job task. 
    // need to add logic when heartbeat is not available 
    // 

    // 
}


impl TaskManager {
    pub fn new(addrs:Vec<SocketAddr>, jobs:Vec<PathBuf>) -> TaskManager {

        let connection = |addr:SocketAddr| -> () {
            
            loop {

            }
        }; 

        // consume vector;
        for addr in addrs {

            let (tx, rx): (Sender<Job>, Receiver<>) = mpsc::channel();

        }

        TaskManager{}
    }

    pub fn run() -> Result<(),()>{
        Ok(())
    }
}

