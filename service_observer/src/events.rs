use std::net::Ipv4Addr;

#[derive(Debug)]
pub(crate) enum ObserverEvents {
    Geolocation(IpGeolocation),
}

#[derive(Debug)]
pub(crate) struct IpGeolocation {
    pub lat: f64,
    pub lng: f64,
    pub ipv4: Ipv4Addr,
}
