use actix_web::{web, App, HttpServer};
use web3::transports::Http;
use web3::types::Address;
use web3::Web3;

mod endpoints;
mod handlers;
mod models;
mod utils;

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
            .data(models::ContractData {
                resolver_address: resolver_address.clone(),
                resolver_abi: resolver_abi.clone(),
                relayer_address: relayer_address.clone(),
                relayer_abi: relayer_abi.clone(),
            })
            .service(web::resource("/api/resolver/{ens_name}").to(handlers::resolver::resolve_ens_name))
            .service(web::resource("/api/relayer/{ens_address}").to(handlers::relayer::relay_endpoint_request))
            .service(web::resource("/endpoint1").to(endpoints::endpoint1))
            .service(web::resource("/endpoint2").to(endpoints::endpoint2))
            .service(web::resource("/endpoint3").to(endpoints::endpoint3))
            .service(web::resource("/endpoint4").to(endpoints::endpoint4))
            .service(web::resource("/endpoint5").to(endpoints::endpoint5))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
    .map_err(|err| {
        eprintln!("Server error: {}", err);
        err
    })
}
