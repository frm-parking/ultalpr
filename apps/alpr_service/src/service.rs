use std::net::SocketAddr;

use tokio::task::JoinHandle;
use tonic::async_trait;
use tonic::transport::Error;
use tonic::transport::Server;
use tonic::Request;
use tonic::Response;
use tonic::Status;

use crate::alpr::HelloReply;
use crate::alpr::HelloRequest;
use crate::alpr_server::Alpr;
use crate::alpr_server::AlprServer;

#[derive(Default)]
pub struct AlprService;

#[async_trait]
impl Alpr for AlprService {
	async fn hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
		todo!()
	}
}

pub async fn serve(addr: SocketAddr) -> JoinHandle<Result<(), Error>> {
	tokio::spawn(
		Server::builder()
			.add_service(AlprServer::new(AlprService))
			.serve(addr),
	)
}
