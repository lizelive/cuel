use chrono::{DateTime, Utc};
use serde::{de::IntoDeserializer, Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};
use subprocess::{Exec, ExitStatus, Popen, PopenConfig, PopenError};

use crate::id::{random_id, Id};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Healthcheck {}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Protocol {
    Tcp,
    Udp,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Port {
    name: String,
    port: Option<u16>,
    protocol: Protocol,
    expose: bool,
    ip: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectionConfig {
    ports: Vec<Port>,
    secrets: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum CommuncationFileConfig {
    JupyterStyle,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payload {
    exec: Exec,
    name: Option<String>,
    timout: Option<f64>,
    connection_config: ConnectionConfig,
    connection_file_config: Option<CommuncationFileConfig>,
    healtcheck: Option<Healthcheck>,
}
pub struct Task {
    id: Id,
    payload: Payload,
    state: State,
}

impl Task {
    fn new(id: Id, payload: Payload) -> Self {
        Self {
            id,
            payload,
            state: State::inital(),
        }
    }
}
