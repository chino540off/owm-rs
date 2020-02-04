extern crate hyper;
extern crate owm;
extern crate tokio_test;

use std::env;

use self::owm::hub::{BoundingBox, FormatResponse, Units, WeatherHub};

#[test]
fn current_by_name() {
    let hub = WeatherHub::new(hyper::Client::new(), &env::var("OWM_API_KEY").unwrap());

    match tokio_test::block_on(
        hub.current().by_name("Pisa", Some("IT")), // Pisa
    ) {
        Err(e) => {
            println!("{:#?}", e);
            assert!(false);
        }
        Ok(info) => {
            assert_eq!(Some(10.4), info.coord.clone().unwrap().lon);
            assert_eq!(Some(43.72), info.coord.clone().unwrap().lat);
            assert_eq!(Some("Pisa".to_string()), info.name);
        }
    }
}

#[test]
fn current_by_id() {
    let hub = WeatherHub::new(hyper::Client::new(), &env::var("OWM_API_KEY").unwrap());

    match tokio_test::block_on(
        hub.current().by_name("Pisa", Some("IT")), // Pisa
    ) {
        Err(e) => {
            println!("{:#?}", e);
            assert!(false);
        }
        Ok(info) => {
            assert_eq!(Some(10.4), info.coord.clone().unwrap().lon);
            assert_eq!(Some(43.72), info.coord.clone().unwrap().lat);
            assert_eq!(Some("Pisa".to_string()), info.name);
        }
    }
}

#[test]
fn current_by_coords() {
    let hub = WeatherHub::new(hyper::Client::new(), &env::var("OWM_API_KEY").unwrap());

    match tokio_test::block_on(
        hub.current().by_name("Pisa", Some("IT")), // Pisa
    ) {
        Err(e) => {
            println!("{:#?}", e);
            assert!(false);
        }
        Ok(info) => {
            assert_eq!(Some(10.4), info.coord.clone().unwrap().lon);
            assert_eq!(Some(43.72), info.coord.clone().unwrap().lat);
            assert_eq!(Some("Pisa".to_string()), info.name);
        }
    }
}

#[test]
fn current_by_zip() {
    let hub = WeatherHub::new(hyper::Client::new(), &env::var("OWM_API_KEY").unwrap());

    match tokio_test::block_on(
        hub.current().by_name("Pisa", Some("IT")), // Pisa
    ) {
        Err(e) => {
            println!("{:#?}", e);
            assert!(false);
        }
        Ok(info) => {
            assert_eq!(Some(10.4), info.coord.clone().unwrap().lon);
            assert_eq!(Some(43.72), info.coord.clone().unwrap().lat);
            assert_eq!(Some("Pisa".to_string()), info.name);
        }
    }
}

#[test]
fn current_by_bounds() {
    let hub = WeatherHub::new(hyper::Client::new(), &env::var("OWM_API_KEY").unwrap());

    match tokio_test::block_on(hub.current().by_bounds(
        &BoundingBox {
            top: 43.73,
            left: 10.38,
            bottom: 43.7,
            right: 10.42,
        },
        10,
        false,
    )) {
        Err(e) => {
            println!("{:#?}", e);
            assert!(false);
        }
        Ok(info) => {
            assert_eq!(1, info.list.clone().unwrap().len());
            assert_eq!(Some("Pisa".to_string()), info.list.clone().unwrap()[0].name);
        }
    }
}

#[test]
fn current_by_circle() {
    let hub = WeatherHub::new(hyper::Client::new(), &env::var("OWM_API_KEY").unwrap());

    match tokio_test::block_on(hub.current().by_circle(43.71, 10.41, 10, false)) {
        Err(e) => {
            println!("{:#?}", e);
            assert!(false);
        }
        Ok(info) => {
            assert_eq!(10, info.list.clone().unwrap().len());
            assert_eq!(Some("Pisa".to_string()), info.list.clone().unwrap()[0].name);
        }
    }
}

#[test]
fn current_with_units() {
    let hub = WeatherHub::new(hyper::Client::new(), &env::var("OWM_API_KEY").unwrap());

    match (
        tokio_test::block_on(hub.current().by_id(6542122)),
        tokio_test::block_on(hub.current().units(Units::Metric).by_id(6542122)),
    ) {
        (_, Err(e)) | (Err(e), _) => {
            println!("{:#?}", e);
            assert!(false);
        }
        (Ok(i1), Ok(i2)) => {
            assert_eq!(i1.name, i2.name);
            assert!(i1.main.unwrap().temp != i2.main.unwrap().temp);
        }
    }
}

#[test]
fn current_with_language() {
    let hub = WeatherHub::new(hyper::Client::new(), &env::var("OWM_API_KEY").unwrap());

    match (
        tokio_test::block_on(hub.current().by_name("Pisa", Some("IT"))),
        tokio_test::block_on(hub.current().lang("IT").by_name("Pisa", Some("IT"))),
    ) {
        (_, Err(e)) | (Err(e), _) => {
            println!("{:#?}", e);
            assert!(false);
        }
        (Ok(i1), Ok(i2)) => {
            assert_eq!(i1.name, i2.name);
            assert!(i1.main.unwrap().temp != i2.main.unwrap().temp);
        }
    }
}
