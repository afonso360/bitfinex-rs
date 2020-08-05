use types::*;
use client::*;
use errors::*;
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug)]
pub struct TradingPair { 
    pub bid: Price,
    pub bid_size: Amount,
    pub ask: Price,
    pub ask_size: Amount,
    pub daily_change: Amount,
    pub daily_change_perc: Amount,
    pub last_price: Price,
    pub volume: Amount,
    pub high: Price,
    pub low: Price                    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundingCurrency { 
    pub frr: Amount,
    pub bid: Price,
    pub bid_period: i64,
    pub bid_size: Amount,   
    pub ask: Price,      
    pub ask_period: i64,              
    pub ask_size: Amount,
    pub daily_change: Amount,
    pub daily_change_perc: Amount,
    pub last_price: Price,
    pub volume: Amount,
    pub high: Price,
    pub low: Price
}

#[derive(Clone)]
pub struct Ticker {
    client: Client,
}

impl Ticker {
    pub fn new() -> Self {
        Ticker {
            client: Client::new(None, None),
        }
    }

    pub fn funding_currency<S>(&self, symbol: S) -> Result<FundingCurrency>
        where S: Into<String>
    {     
        let endpoint: String = format!("ticker/f{}", symbol.into());
        let data = self.client.get(endpoint, String::new())?;

        let ticker: FundingCurrency = from_str(data.as_str())?;

        Ok(ticker)
    }    

    pub fn trading_pair<S>(&self, symbol: S) -> Result<TradingPair>
        where S: Into<String>
    {     
        let endpoint: String = format!("ticker/t{}", symbol.into());
        let data = self.client.get(endpoint, String::new())?;

        let ticker: TradingPair = from_str(data.as_str())?;

        Ok(ticker)
    }
}