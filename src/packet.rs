use crate::tcpflags;
use pnet::packet::{ip::IpNextHeaderProtocols, Packet};
use pnet::util;

use std::fmt::{self, Debug};
use std::net::Ipv4Addr;
const TCP_HEADER_SIZE: usize = 20;
