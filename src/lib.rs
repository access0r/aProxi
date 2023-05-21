use actix_web::{web, App, HttpServer};
use web3::transports::Http;
use web3::types::Address;
use web3::Web3;

mod handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let infura_project_id = "your-infura-project-id";
    let infura_url = format!("https://mainnet.infura.io/v3/{}", infura_project_id);
    let transport = Http::new(&infura_url).expect("Failed to create web3 transport");
    let web3 = Web3::new(transport);

    let resolver_address = handlers::resolver::load_address_from_file("contracts/resolver_address.txt")
        .expect("Failed to load resolver address");
    let resolver_abi = handlers::resolver::load_abi_from_file("contracts/resolver_abi.json")
        .expect("Failed to load resolver ABI");

    let relayer_address = handlers::relayer::load_address_from_file("contracts/relayer_address.txt")
        .expect("Failed to load relayer address");
    let relayer_abi = handlers::relayer::load_abi_from_file("contracts/relayer_abi.json")
        .expect("Failed to load relayer ABI");

    HttpServer::new(move || {
        App::new()
            .data(web3.clone())
            .service(handlers::resolver::resolve_ens_name)
            .service(handlers::relayer::relay_endpoint_request)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
