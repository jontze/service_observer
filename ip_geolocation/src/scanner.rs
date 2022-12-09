use std::net::Ipv4Addr;

use threaty::api::shodan::{shodan_api::ShodanAPI, shodan_client::ShodanClient};

use crate::{Geolocation, IpScannerError};

#[derive(Debug, Clone)]
pub struct IpScanner {
    shodan_client: ShodanClient,
}

impl<'a> IpScanner {
    pub fn new(shodan_key: &'a str) -> Self {
        Self {
            shodan_client: ShodanClient::new(shodan_key, None, None),
        }
    }

    pub async fn ip_geolocation(self, ipv4: &Ipv4Addr) -> Result<Geolocation, IpScannerError> {
        self.shodan_client
            .host_info(ipv4.to_owned().into(), None, None)
            .send()
            .await
            .map_err(|_| IpScannerError::RequestError)?
            .text()
            .await
            .map_err(|_| IpScannerError::RequestError)
            .and_then(|str_body| {
                let ret = serde_json::from_str::<Geolocation>(&str_body)?;
                Ok(ret)
            })
    }
}
