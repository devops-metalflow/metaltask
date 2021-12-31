use crate::config::config::{Config, VERSION};
use flow::flow_proto_server::{FlowProto, FlowProtoServer};
use flow::{FlowReply, FlowRequest};
use std::error::Error;
use std::str;
use tonic::{transport::Server, Request, Response, Status, Streaming};

pub mod flow {
    tonic::include_proto!("flow");
}

pub struct Flow {
    pub config: Config,
    pub routine: fn(Config, Vec<u8>, String, bool) -> Result<String, Box<dyn Error>>,
}

impl Flow {
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let url = self.config.listen_url.parse().unwrap();
        let server = FlowServer {
            config: self.config.clone(),
            routine: self.routine,
        };

        Server::builder()
            .add_service(FlowProtoServer::new(server))
            .serve(url)
            .await?;

        Ok(())
    }
}

pub struct FlowServer {
    pub config: Config,
    pub routine: fn(Config, Vec<u8>, String, bool) -> Result<String, Box<dyn Error>>,
}

#[tonic::async_trait]
impl FlowProto for FlowServer {
    async fn send_flow(
        &self,
        request: Request<Streaming<FlowRequest>>,
    ) -> Result<Response<FlowReply>, Status> {
        let mut data: Vec<u8> = Vec::new();
        let mut path: String = "".to_string();
        let mut runnable: bool = false;

        let err: String;
        let out: String;

        let mut stream: Streaming<FlowRequest> = request.into_inner();

        while let Some(mut m) = stream.message().await? {
            data.append(&mut m.data);
            path = m.path;
            runnable = m.runnable;
        }

        match str::from_utf8(&data) {
            Ok(s) => {
                if s == VERSION {
                    err = "".to_string();
                    out = self.config.version_info.clone();
                } else {
                    match (self.routine)(self.config.clone(), data, path, runnable) {
                        Ok(b) => {
                            err = "".to_string();
                            out = b;
                        }
                        Err(e) => {
                            err = e.to_string();
                            out = "".to_string();
                        }
                    }
                }
            }
            Err(e) => {
                err = e.to_string();
                out = "".to_string();
            }
        }

        let reply = flow::FlowReply {
            error: err,
            output: out,
        };
        Ok(Response::new(reply))
    }
}
