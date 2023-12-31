use tonic::{transport::Server, Request, Response, Status};
use super::two_phase_commit::{PreparePhaseReq, PreparePhaseResp, CommitPhaseReq, CommitPhaseResp};
use super::two_phase_commit::two_phase_commit_service_server::{TwoPhaseCommitService, TwoPhaseCommitServiceServer};

// defining a struct for our service
#[derive(Default, Debug)]
pub struct TwoPhaseCommit {}

// implementing rpc for service defined in .proto
#[tonic::async_trait]
impl TwoPhaseCommitService for TwoPhaseCommit {
    async fn prepare(&self, req: Request<PreparePhaseReq>) -> Result<Response<PreparePhaseResp>, Status> {
        println!("{:?}", req);
        Ok(Response::new(PreparePhaseResp {
            version: "1".to_string(),
            ack: "YES".to_string()
        }))
    }
    async fn commit(&self, req: Request<CommitPhaseReq>) -> Result<Response<CommitPhaseResp>, Status> {
        let _version = &req.get_ref().version;
        Ok(Response::new(CommitPhaseResp { 
            version: "1".to_owned() 
        }))
    }
}


// #[tokio::main]
// pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // defining address for our service
//     let addr = "[::1]:50051".parse().unwrap();
//     // creating a service
//     let prepare_phase = TwoPhaseCommit::default();
//     println!("Rpc Server listening on {}", addr);
//     // adding our service to our server.
//     Server::builder()
//         .add_service(PreparePhaseServer::new(prepare_phase))
//         .serve(addr)
//         .await?;
//     Ok(())
// }


#[tokio::main]
pub async fn init_rpc_server() -> Result<(), Box<dyn std::error::Error>>{
    // defining address for our service
    // let addr: std::net::SocketAddr = "[::1]:50051".parse().unwrap();
    let addr: std::net::SocketAddr = "127.0.0.1:50051".parse().unwrap();

    // creating a service
    let prepare_phase = TwoPhaseCommit::default();
    println!("Rpc Server listening on {:?}", addr);

    // adding our service to our server.
    Server::builder()
        .add_service(TwoPhaseCommitServiceServer::new(prepare_phase))
        .serve(addr)
        .await?;

    Ok(())
}

pub fn init_rpc_server1(){
    // defining address for our service
    // let addr: std::net::SocketAddr = "[::1]:50051".parse().unwrap();
    let addr: std::net::SocketAddr = "127.0.0.1:50051".parse().unwrap();
    // creating a service
    let prepare_phase = TwoPhaseCommit::default();
    println!("Rpc Server listening on {}", addr);
    
    // adding our service to our server.
    let rt = tokio::runtime::Runtime::new().expect("failed to obtain a new Runtime object");
    let serve_future = Server::builder()
        .add_service(TwoPhaseCommitServiceServer::new(prepare_phase))
        .serve(addr);

    rt.block_on(serve_future)
        .expect("failed to successfully run the future on Runtime");
}