use std::net::TcpStream;
use tungstenite::{connect, stream::MaybeTlsStream, Error, Message, WebSocket};
use url::Url;

#[allow(dead_code)]
pub enum AnthemError {
    Io,
    ConnectionClosed,
    AlreadyClosed,
    Capacity,
    Other,
}

#[allow(dead_code)]
pub type AnthemResult = Result<(), AnthemError>;

#[allow(dead_code)]
pub struct AnthemIP {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl AnthemIP {
    pub fn new(uri: &str) -> Self {
        let (socket, _response) = connect(Url::parse(uri).unwrap()).expect("Can't connect");

        AnthemIP { socket }
    }

    #[allow(dead_code)]
    pub fn set_power(self, power: bool) -> AnthemResult {
        self.write_cmd(&format!("Z1POW{}", u32::from(power)))
    }

    #[allow(dead_code)]
    fn write_cmd(mut self, cmd: &str) -> AnthemResult {
        let result = self.socket.write_message(Message::Text(cmd.into()));
        match result {
            Ok(()) => {
                if self.socket.read_message().is_err() {
                    Err(AnthemError::Other)
                } else {
                    Ok(())
                }
            }
            Err(Error::ConnectionClosed) => Err(AnthemError::ConnectionClosed),
            Err(Error::AlreadyClosed) => Err(AnthemError::AlreadyClosed),
            Err(Error::Io(..)) => Err(AnthemError::Io),
            Err(Error::Capacity(..)) => Err(AnthemError::Capacity),
            _ => Err(AnthemError::Other),
        }
    }
}
