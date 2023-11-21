use std::net::SocketAddr;
use async_osc::{OscPacket, Error};
pub(super) type OscRxPacket = Option<Result<(OscPacket, SocketAddr), Error>>;
