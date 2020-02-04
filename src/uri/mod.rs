/// Generic URI builder that handles all URI-related stuff.
pub struct UriBuilder<'a> {
    api_ver: &'a str,
    method: &'a str,
    params: std::collections::HashMap<&'a str, String>,
}

/// Implemented by basically every query builder in the crate.
pub trait HasBuilder<'a> {
    fn builder(&mut self) -> &mut UriBuilder<'a>;
}

impl<'a> UriBuilder<'a> {
    pub fn new() -> Self {
        UriBuilder {
            api_ver: "2.5",
            method: "",
            params: std::collections::HashMap::with_capacity(10),
        }
    }

    /// Set the endpoint method.
    pub fn method(&mut self, method: &'a str) -> &mut Self {
        self.method = method;
        self
    }

    /// Add param to the URI.
    pub fn param(&mut self, key: &'a str, val: String) -> &mut Self {
        self.params.insert(key, val);
        self
    }

    /// Consumes the builder and returns the corresponding URI.
    pub fn build(&self) -> hyper::Uri {
        let query = self
            .params
            .iter()
            .map(|(key, value)| format!("{key}={value}", key = key, value = value))
            .collect::<Vec<String>>()
            .join("&");

        let path = format!(
            "/data/{api}/{method}{query}",
            api = self.api_ver,
            method = self.method,
            query = if query.len() > 0 {
                "?".to_owned() + &query
            } else {
                "".to_owned()
            }
        );

        hyper::Uri::builder()
            .scheme("http")
            .authority("api.openweathermap.org")
            .path_and_query(path.as_str())
            .build()
            .unwrap()
    }
}
