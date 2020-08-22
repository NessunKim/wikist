use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::{ok, Ready};
use ipnetwork::IpNetwork;
use std::str::FromStr;

pub struct ConnectionInfo {
    pub ip_address: IpNetwork,
}

impl FromRequest for ConnectionInfo {
    type Error = Error;
    type Future = Ready<Result<ConnectionInfo, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let conn_info = req.connection_info();
        let remote = conn_info.remote().unwrap();
        let ip_address = IpNetwork::from_str(remote.split(':').collect::<Vec<&str>>()[0]).unwrap();
        ok(ConnectionInfo { ip_address })
    }
}
