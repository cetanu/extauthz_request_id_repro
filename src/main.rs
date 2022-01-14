use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use socket2::{Domain, Socket, Type};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;


mod proto;
use proto::envoy::service::auth::v3::{
    authorization_server::{Authorization, AuthorizationServer},
    check_response::HttpResponse::OkResponse,
    CheckRequest, CheckResponse, OkHttpResponse,
};
use proto::google;

struct ExtAuthzServer {}

fn response() -> CheckResponse {
    CheckResponse {
        status: Some(google::rpc::Status {
            code: 0,
            details: vec![],
            message: "".to_string(),
        }),
        http_response: Some(OkResponse(OkHttpResponse {
            headers: vec![],
            response_headers_to_add: vec![],
            headers_to_remove: vec![],
            dynamic_metadata: None,
        })),
        dynamic_metadata: None,
    }
}

#[tonic::async_trait]
impl Authorization for ExtAuthzServer {
    async fn check(
        &self,
        request: Request<CheckRequest>,
    ) -> Result<Response<CheckResponse>, Status> {
        // Unpack attribute context and http request
        let attributes = request.into_inner().attributes.unwrap();
        let http = attributes.request.unwrap().http.unwrap();

        // Print the received request id
        println!("{}", http.id);

        return Ok(Response::new(response()))

    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Networking
    let addr = IpAddr::from_str("0.0.0.0")?;
    let port = 9991;
    let addr = SocketAddr::new(addr, port);

    let socket = Socket::new(Domain::for_address(addr), Type::STREAM, None)?;
    socket.set_reuse_address(true)?;
    socket.set_reuse_port(true)?;
    socket.bind(&addr.into())?;
    socket.set_nonblocking(true)?;
    socket.listen(128)?; // backlog


    let async_listener = TcpListener::from_std(std::net::TcpListener::from(socket))?;
    let incoming = TcpListenerStream::new(async_listener);

    // Service
    let extauthz = ExtAuthzServer {};
    let service = AuthorizationServer::new(extauthz);

    // Start server
    println!("Starting");
    Server::builder()
        .add_service(service)
        .serve_with_incoming(incoming)
        .await?;

    // Exit zero
    Ok(())
}
