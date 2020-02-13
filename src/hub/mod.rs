use crate::current;
use crate::uri;

use bytes::buf::ext::BufExt;
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Rectangle specified by geographic coordinates (latitude and longitude).
#[derive(Debug)]
pub struct BoundingBox {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

/// Units format for this query.
#[derive(Debug, Serialize, Deserialize)]
pub enum Units {
    Metric,
    Imperial,
}

impl ToString for Units {
    fn to_string(&self) -> String {
        match self {
            &Units::Metric => "metric".to_string(),
            &Units::Imperial => "imperial".to_string(),
        }
    }
}

pub trait FormatResponse<'a>
where
    Self: std::marker::Sized + uri::HasBuilder<'a>,
{
    /// Change units format for the query. Default is Standard.
    fn units(mut self, units: Units) -> Self {
        self.builder().param("units", units.to_string());
        self
    }

    /// Change language for the query. Note that only the `description` field
    /// of [Weather](struct.Weather.html) is translated.
    fn lang(mut self, lang: &str) -> Self {
        self.builder().param("lang", lang.to_string());
        self
    }
}

/// Central hub to access all weather-related facilities.
pub struct WeatherHub {
    client: hyper::client::Client<hyper::client::HttpConnector>,
    key: String,
}

impl<'a> WeatherHub {
    /// Creates a new WeatherHub which will use the provided client to perform
    /// its requests. It also requires an OWM API key.
    pub fn new(client: hyper::Client<hyper::client::HttpConnector>, key: &str) -> WeatherHub {
        WeatherHub {
            client: client,
            key: key.to_string(),
        }
    }

    /// Provides access to the current-weather facilities.
    pub fn current(&'a self) -> current::WeatherQuery<'a> {
        current::WeatherQuery::new(&self, {
            let mut ub = uri::UriBuilder::new();
            ub.param("appid", self.key.clone());
            ub
        })
    }

    /// Does the actual API call, parses the response and handles any errors.
    pub async fn run_query<D: for<'de> serde::Deserialize<'de>>(
        &'a self,
        uri: hyper::Uri,
    ) -> Result<D> {
        let res = self.client.get(uri).await?;
        let body = hyper::body::aggregate(res).await?;

        Ok(serde_json::from_reader(body.reader())?)
    }
}
