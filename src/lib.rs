//! This crate provides access to OpenWeatherMap's API.
//!
//! At the moment, JSON is the only supported data format. To use the API, you
//! also need to provide an API key, which can obtained at the following link:
//! http://openweathermap.org/appid.
//!
//! # Features
//!
//! Right now, the crate supports:
//!
//! * [Querying the current weather](struct.WeatherQuery.html)
//!  * By city name
//!  * By city ID
//!  * By ZIP code
//!  * By coordinates (point, bounding box and bounding circle)
//! * Multi-language queries
//! * Standard, metric and imperial units
//!
//! # Example
//!
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate owm;
//!
//! use owm::hub::WeatherHub;
//!
//! # async fn eval() {
//! let hub = WeatherHub::new(hyper::Client::new(), "YOUR_API_KEY");
//! let res = hub.current().by_name("London", Some("UK")).await;
//!
//! match res {
//!     Err(e) => println!("{:?}", e),
//!     Ok(res) => println!("{:?}", res),
//! }
//! # }
//! ```

pub mod current;
pub mod data;
pub mod hub;
mod uri;

extern crate hyper;
extern crate serde;
