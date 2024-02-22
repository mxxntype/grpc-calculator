use color_eyre::eyre::Result;
use proto::calculator_client::CalculatorClient;

pub mod proto {
    tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let url = "http://[::1]:50051";
    let mut client = CalculatorClient::connect(url).await?;

    let req = proto::CalculationRequest { a: 4, b: 5 };
    let request = tonic::Request::new(req);

    let response = client.add(request).await?;

    println!("Response: {:?}", response.get_ref().result);

    Ok(())
}
