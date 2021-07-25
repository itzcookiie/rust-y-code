use std::fmt::Debug;
use std::net::TcpStream;
use url::Url;
use std::io::{Write, Read};
use serde::*;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct ExampleRequest {
    pub name:String
}

#[serde(crate = "deps")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackMessage {
    pub text:String,
}
