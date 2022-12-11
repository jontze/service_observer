#[derive(Debug)]
pub(crate) enum ObserverEvents {
    /// First eintry is lat, second is lng
    Geolocation((f64, f64)),
}
