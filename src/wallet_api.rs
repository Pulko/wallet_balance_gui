use core::fmt;
use serde;
use serde_json;
use std::{
    fmt::{Display, Formatter},
    io,
};
use thiserror::Error;

use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://solana-gateway.moralis.io";

#[derive(Error, Debug)]
pub enum WalletApiError {
    #[error("Failed to fetch data form API")]
    RequestFailed(#[from] ureq::Error),
    #[error("Bad request: {0}")]
    _BadRequest(String),
    #[error("Failed to parse data")]
    ParseFailed(#[from] serde_json::Error),
    #[error("Failed to convert response to string")]
    ResponseToStringFailed(#[from] io::Error),
}

pub struct ApiService {
    pub api_key: String,
    pub endpoint: Endpoint,
    pub network: Network,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Network {
    Mainnet,
    _Testnet,
    Devnet,
}

impl PartialEq<Network> for Network {
    fn eq(&self, other: &Network) -> bool {
        match (self, other) {
            (Network::Mainnet, Network::Mainnet) => true,
            (Network::Devnet, Network::Devnet) => true,
            _ => false,
        }
    }
}

impl Display for Network {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Network::Mainnet => write!(f, "mainnet"),
            Network::_Testnet => write!(f, "testnet"),
            Network::Devnet => write!(f, "devnet"),
        }
    }
}

pub enum Endpoint {
    AccountTokens,
    _TokenPrice,
}

pub fn get_endpoint(endpoint: &Endpoint) -> impl Fn(String, String) -> String {
    match endpoint {
        Endpoint::AccountTokens => |network: String, address: String| {
            return format!("{}/account/{}/{}/tokens", BASE_URL, network, address);
        },
        Endpoint::_TokenPrice => |network: String, address: String| {
            return format!("{}/token/{}/{}/price", BASE_URL, network, address);
        },
    }
}

pub fn get_network(network: &Network) -> String {
    match network {
        Network::Mainnet => "mainnet".to_string(),
        Network::_Testnet => "testnet".to_string(),
        Network::Devnet => "devnet".to_string(),
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TokenPriceResponse {
    usd_price: f64,
    exchange_name: String,
    exchange_address: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AccountTokensResponse {
    pub tokens: Vec<Token>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Token {
    pub mint: String,
    pub amount: String,
    pub name: String,
    pub symbol: String,
}

impl ApiService {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            endpoint: Endpoint::AccountTokens,
            network: Network::Devnet,
        }
    }

    fn build_url(&self, network: &Network, address: String) -> String {
        let endpoint = get_endpoint(&self.endpoint);
        let network = get_network(network);

        endpoint(network, address)
    }

    pub fn get_account_tokens(
        &self,
        network: &Network,
        address: String,
    ) -> Result<AccountTokensResponse, WalletApiError> {
        let url = self.build_url(network, address);
        let req = ureq::get(&url).set("X-API-Key", &self.api_key);
        let resp: Vec<Token> = req.call()?.into_json()?;

        Ok(AccountTokensResponse { tokens: resp })
    }
}
