use crate::data;
use crate::hub;
use crate::uri;

/// Query builder for the Current Weather API.
pub struct WeatherQuery<'a> {
    hub: &'a hub::WeatherHub,
    builder: uri::UriBuilder<'a>,
}

impl<'a> uri::HasBuilder<'a> for WeatherQuery<'a> {
    fn builder(&mut self) -> &mut uri::UriBuilder<'a> {
        &mut self.builder
    }
}

impl<'a> hub::FormatResponse<'a> for WeatherQuery<'a> {}

impl<'a> WeatherQuery<'a> {
    pub fn new(hub: &'a hub::WeatherHub, builder: uri::UriBuilder<'a>) -> WeatherQuery<'a> {
        WeatherQuery {
            hub: hub,
            builder: builder,
        }
    }

    /// Query current weather by passing a city name and an optional country code.
    pub async fn by_name(
        mut self,
        city: &str,
        country: Option<&str>,
    ) -> hub::Result<data::WeatherInfo> {
        let q = match country {
            None => city.to_string(),
            Some(code) => format!("{},{}", city, code),
        };

        self.hub
            .run_query(self.builder.method("weather").param("q", q).build())
            .await
    }

    /// Query current weather by passing a city ID. API responds with exact result.
    /// See http://bulk.openweathermap.org/sample/ for a list of city IDs.
    pub async fn by_id(mut self, id: i32) -> hub::Result<data::WeatherInfo> {
        self.hub
            .run_query(
                self.builder
                    .method("weather")
                    .param("id", id.to_string())
                    .build(),
            )
            .await
    }

    /// Query current weather by passing a ZIP code and an optional country code.
    pub async fn by_zip_code(
        mut self,
        zip: i32,
        country: Option<&str>,
    ) -> hub::Result<data::WeatherInfo> {
        let q = match country {
            None => zip.to_string(),
            Some(code) => format!("{},{}", zip, code),
        };

        self.hub
            .run_query(self.builder.method("weather").param("zip", q).build())
            .await
    }

    /// Query current weather by passing geographic coordinates.
    pub async fn by_coords(mut self, lat: f32, lon: f32) -> hub::Result<data::WeatherInfo> {
        self.hub
            .run_query(
                self.builder
                    .method("weather")
                    .param("lat", lat.to_string())
                    .param("lon", lon.to_string())
                    .build(),
            )
            .await
    }

    /// Query current weather for cities within the defined rectangle specified
    /// by the bounding box using the given zoom. Server clustering of points
    /// can also be used.
    pub async fn by_bounds(
        mut self,
        bbox: &hub::BoundingBox,
        zoom: i32,
        cluster: bool,
    ) -> hub::Result<data::WeatherBoxAggregate> {
        let q = format!(
            "{},{},{},{},{}",
            bbox.left, bbox.bottom, bbox.right, bbox.top, zoom
        );

        self.hub
            .run_query(
                self.builder
                    .method("box/city")
                    .param("bbox", q)
                    .param("cluster", (if cluster { "yes" } else { "no" }).to_string())
                    .build(),
            )
            .await
    }

    /// Query current weather for cities laid inside a circle specified by
    /// center point (lan, lot) and expected number of cities withing.
    pub async fn by_circle(
        mut self,
        lat: f32,
        lon: f32,
        count: i32,
        cluster: bool,
    ) -> hub::Result<data::WeatherBoxAggregate> {
        self.hub
            .run_query(
                self.builder
                    .method("find")
                    .param("lat", lat.to_string())
                    .param("lon", lon.to_string())
                    .param("cnt", count.to_string())
                    .param("cluster", (if cluster { "yes" } else { "no" }).to_string())
                    .build(),
            )
            .await
    }
}
