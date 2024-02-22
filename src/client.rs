use color_eyre::eyre::Result;
use proto::calculator_client::CalculatorClient;
use tonic::Request;

pub mod proto {
    tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let req = proto::CalculationRequest { a: 4, b: 5 };
    let request = Request::new(req);
    let response = CalculatorClient::connect("http://[::1]:50051")
        .await?
        .add(request)
        .await?;

    println!("Response: {:?}", response.get_ref().result);

    Ok(())
}
