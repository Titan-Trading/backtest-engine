use reqwest::blocking::Client as RestClient;
use reqwest::Error;

// provides an easy way to make rest requests
pub struct Client {
    client: RestClient,
}

impl Client {
    pub fn new() -> Self {
        let client = RestClient::new();

        Self {
            client
        }
    }

    // make a get request to the given url and return the response body
    pub fn get(&self, url: &str) -> Result<String, Box<Error>> {
        let mut response = self.client.get(url).send()?;
        let body = response.text()?;
        Ok(body)
    }

    // make a post request to the given url and return the response body
    pub fn post(&self, url: &str, body: &str) -> Result<String, Box<Error>> {
        let mut response = self.client.post(url).body(body.to_string()).send()?;
        let body = response.text()?;
        Ok(body)
    }

    // make a put request to the given url and return the response body
    pub fn put(&self, url: &str, body: &str) -> Result<String, Box<Error>> {
        let mut response = self.client.put(url).body(body.to_string()).send()?;
        let body = response.text()?;
        Ok(body)
    }

    // make a delete request to the given url and return the response body
    pub fn delete(&self, url: &str) -> Result<String, Box<Error>> {
        let mut response = self.client.delete(url).send()?;
        let body = response.text()?;
        Ok(body)
    }
}