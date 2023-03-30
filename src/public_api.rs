use std::time::Duration;
use ureq;

#[derive(Debug)]
pub struct PublicApi {
    end_point: String,
    agent: ureq::Agent,
}

impl PublicApi {
    pub fn new(end_point: String) -> Self {
        let agent: ureq::Agent = ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .build();
        Self { end_point, agent }
    }

    pub fn get_ticker(self, pair: &str) -> Result<ureq::serde_json::Value, ureq::Error> {
        let path = format!("{}/{}/ticker", self.end_point, pair);
        let json: ureq::serde_json::Value = self.agent.get(&path).call()?.into_json()?;
        Ok(json)
    }
}
