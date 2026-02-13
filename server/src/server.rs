use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};

use crate::rock_paper_scissors::{self, Game, TurnResult};

pub struct Server {
    port: u16,
    bind_address: SocketAddr,
}

impl Server {
    pub async fn run(self) -> std::io::Result<()> {
        info!("Starting Server on port {}", self.port);
        let listener = tokio::net::TcpListener::bind(self.bind_address).await?;

        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(async {
                info!("Handling incomming connection request");
                Server::accept_connection(stream).await
            });
        }

        Ok(())
    }
    async fn accept_connection(stream: TcpStream) {
        info!("Begin establising connection & starting game");
        let addr = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        info!("Peer address: {}", addr);

        let ws_stream = tokio_tungstenite::accept_async(stream)
            .await
            .expect("Error during the websocket handshake occurred");

        info!("New WebSocket connection: {}", addr);
        let (mut write, mut read) = ws_stream.split();

        let mut game_session = rock_paper_scissors::Game::new();

        write
            .send(Message::Text(Utf8Bytes::from(format!("Hi!"))))
            .await
            .expect("Error welcoming peer");

        while let Some(msg) = read.next().await {
            if let Ok(msg) = msg {
                match deserialise(&msg) {
                    Ok(JsonIn::Payload(choice)) => {
                        let result = game_session.process_turn(choice);
                        let response = JsonOut::Result {
                            turn_result: result,
                            game: &game_session,
                        };
                        if let Ok(text) = serde_json::to_string(&response) {
                            if write.send(Message::Text(text.into())).await.is_err() {
                                error!(
                                    "Closing Connection: Error serialising JsonOut::Result response"
                                );
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        let response = JsonOut::Error(ErrorCode::InvalidRequest);
                        error!("{e:?}: {msg}");
                        if let Ok(text) = serde_json::to_string(&response) {
                            if write.send(Message::Text(text.into())).await.is_err() {
                                error!(
                                    "Closing Connection: Error serialising JsonOut::Error response"
                                );
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct ServerBuilder {
    port: Option<u16>,
    bind_address: Option<String>,
}

#[derive(Debug)]
pub enum BuildError {
    MissingOrInvalidPort,
    InvalidBindAddress,
}

impl ServerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn build(self) -> Result<Server, BuildError> {
        let port = self.port.ok_or(BuildError::MissingOrInvalidPort)?;

        let address = self.bind_address.unwrap_or_else(|| "127.0.0.1".into());

        let ip = IpAddr::from_str(&address).map_err(|_| BuildError::InvalidBindAddress)?;

        Ok(Server {
            port: port,
            bind_address: SocketAddr::new(ip, port),
        })
    }
}

fn deserialise(msg: &Message) -> Result<JsonIn, ErrorCode> {
    let text = msg.to_text().map_err(|_| ErrorCode::InternalServerError)?;

    let req = serde_json::from_str(text).map_err(|_| ErrorCode::InvalidRequest)?;

    Ok(req)
}

#[derive(Deserialize, Debug)]
pub enum JsonIn {
    Payload(rock_paper_scissors::Option),
}

#[derive(Serialize, Debug)]
pub enum JsonOut<'a> {
    Result {
        turn_result: TurnResult,
        game: &'a Game,
    },
    Error(ErrorCode),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ErrorCode {
    InvalidRequest = 400,
    InternalServerError = 500,
}
