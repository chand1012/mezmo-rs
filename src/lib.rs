use serde::Serialize;
use std::collections::HashMap;
use std::vec::Vec;

use gethostname::gethostname;
use reqwest;

const BASE_URL: &str = "https://logs.logdna.com/logs/ingest";

type Line = HashMap<String, String>;
type Lines = Vec<Line>;

#[derive(Serialize)]
struct Body {
    lines: Lines,
}

pub struct Logger {
    blocking_client: reqwest::blocking::Client,
    client: reqwest::Client,
    apikey: String,
    hostname: String,
    tags: String,
    app: String,
}

impl Logger {
    // Create a new logger.
    pub fn new(apikey: String, tags: String, app: String) -> Self {
        let blocking_client = reqwest::blocking::Client::new();
        let client = reqwest::Client::new();
        let hostname = gethostname().into_string().unwrap();

        Self {
            blocking_client,
            client,
            apikey,
            hostname,
            tags,
            app,
        }
    }

    // Sends a log line to Mezmo. This is a blocking call.
    pub fn blocking_log(
        &self,
        log_line: String,
        level: String,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let query = format!(
            "hostname={}&timestamp={}&tags={}",
            self.hostname, timestamp, self.tags
        );

        let url = format!("{}?{}", BASE_URL, query);

        let mut lines = Lines::new();

        let mut line = Line::new();

        let app = self.app.clone();

        line.insert("line".to_string(), log_line);
        line.insert("app".to_string(), app);
        line.insert("level".to_string(), level);
        line.insert("timestamp".to_string(), timestamp.to_string());

        lines.push(line);

        let body = Body { lines };

        let res = self
            .blocking_client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("apikey", &self.apikey)
            .json(&body)
            .send();

        return res;
    }

    // Sends a log line to Mezmo. This is an async call.
    pub async fn log(
        &self,
        log_line: String,
        level: String,
    ) -> Result<reqwest::Response, reqwest::Error> {
        // get the current unix timestamp in ms
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let query = format!(
            "hostname={}&timestamp={}&tags={}",
            self.hostname, timestamp, self.tags
        );

        let url = format!("{}?{}", BASE_URL, query);

        let mut lines = Lines::new();

        let mut line = Line::new();

        // copy the app name into the current fn
        let app = self.app.clone();

        line.insert("line".to_string(), log_line);
        line.insert("app".to_string(), app);
        line.insert("level".to_string(), level);
        line.insert("timestamp".to_string(), timestamp.to_string());

        lines.push(line);

        let body = Body { lines };

        let res = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("apikey", &self.apikey)
            .json(&body)
            .send()
            .await;

        return res;
    }
}

#[cfg(test)]
mod tests {

    use std::env;

    use crate::Logger;

    fn getenv(key: &str) -> String {
        match env::var(key) {
            Ok(val) => val,
            Err(_) => panic!("{} is not set", key),
        }
    }

    #[test]
    fn test_constructor() {
        let apikey = getenv("LOGDNA_APIKEY");
        let tags = getenv("LOGDNA_TAGS");
        Logger::new(apikey, tags, "test".to_string());
    }

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_log() {
        let apikey = getenv("LOGDNA_APIKEY");
        let tags = getenv("LOGDNA_TAGS");
        let logger = Logger::new(apikey, tags, "test".to_string());

        let res = logger.log("test log".to_string(), "info".to_string());

        assert!(aw!(res).is_ok());
    }

    #[test]
    fn test_blocking_log() {
        let apikey = getenv("LOGDNA_APIKEY");
        let tags = getenv("LOGDNA_TAGS");
        let logger = Logger::new(apikey, tags, "test".to_string());

        let res = logger.blocking_log("test blocking log".to_string(), "info".to_string());

        assert!(res.is_ok());
    }
}
