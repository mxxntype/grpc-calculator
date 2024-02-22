use color_eyre::eyre::Result;
use proto::admin_server::{Admin, AdminServer};
use proto::calculator_server::{Calculator, CalculatorServer};
use proto::{CalculationRequest, CalculationResponse, CountRequest, CountResponse};
use tonic::metadata::MetadataValue;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

mod proto {
    tonic::include_proto!("calculator");

    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("calculator_descriptor");
}

type State = std::sync::Arc<tokio::sync::RwLock<u64>>;

#[derive(Debug, Default)]
struct CalculatorService {
    state: State,
}

impl CalculatorService {
    async fn increment_counter(&self) {
        let mut count = self.state.write().await;
        *count += 1;
    }
}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self,
        request: Request<CalculationRequest>,
    ) -> Result<Response<CalculationResponse>, tonic::Status> {
        self.increment_counter().await;

        let input = request.get_ref();
        let response = CalculationResponse {
            result: input.a + input.b,
        };

        Ok(Response::new(response))
    }

    async fn divide(
        &self,
        request: Request<CalculationRequest>,
    ) -> Result<Response<CalculationResponse>, tonic::Status> {
        self.increment_counter().await;

        let input = request.get_ref();
        let response = CalculationResponse {
            result: input
                .a
                .checked_div(input.b)
                .ok_or_else(|| tonic::Status::invalid_argument("Cannot divide by 0"))?,
        };

        Ok(Response::new(response))
    }
}

#[derive(Default, Debug)]
struct AdminService {
    state: State,
}

#[tonic::async_trait]
impl Admin for AdminService {
    async fn get_request_count(
        &self,
        _request: Request<CountRequest>,
    ) -> Result<Response<CountResponse>, tonic::Status> {
        let response = CountResponse {
            count: *self.state.read().await,
        };

        Ok(Response::new(response))
    }
}

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token: MetadataValue<_> = "12345".parse().unwrap();
    match req.metadata().get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let addr = "[::1]:50051".parse()?;
    let state = State::default();

    let calc = CalculatorService {
        state: state.clone(),
    };

    let admin = AdminService {
        state: state.clone(),
    };

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .accept_http1(true)
        .layer(tower_http::cors::CorsLayer::permissive())
        .add_service(service)
        .add_service(tonic_web::enable(CalculatorServer::new(calc)))
        .add_service(AdminServer::with_interceptor(admin, check_auth))
        .serve(addr)
        .await?;

    Ok(())
}
