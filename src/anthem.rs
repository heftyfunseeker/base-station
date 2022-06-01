use std::{io::Read, io::Write, net::TcpStream, os::unix::prelude::OsStrExt};

#[allow(dead_code)]
pub enum AnthemError {
    Io,
    Other,
}

pub type AnthemResult = Result<(), AnthemError>;

pub struct AnthemIP {
    stream: TcpStream,
}

enum AnthemCommand {
    QueryModel,
    QueryModelAndFirmwareVersion,
    QuerySoftwareVersion,
    QuerySerialNumber,
    QueryActiveInputs,
    QueryActiveInputName(u8), // input number
    Zone1Power(bool),
    Zone1CurrentInput(u8), // input number
}

impl From<AnthemCommand> for String {
    fn from(command: AnthemCommand) -> Self {
        match command {
            AnthemCommand::QueryModel => "IDM".into(),
            AnthemCommand::QueryModelAndFirmwareVersion => "IDQ".into(),
            AnthemCommand::QuerySoftwareVersion => "IDS".into(),
            AnthemCommand::QuerySerialNumber => "GSN".into(),
            AnthemCommand::QueryActiveInputs => "ICN".into(),
            AnthemCommand::QueryActiveInputName(input_num) => format!("IS{}IN", input_num),
            AnthemCommand::Zone1Power(power_on) => format!("Z1POW{}", u32::from(power_on)),
            AnthemCommand::Zone1CurrentInput(input_num) => format!("Z1INP{}", input_num),
        }
    }
}

impl AnthemIP {
    pub fn new(address: &str, socket: &str) -> Self {
        let result = TcpStream::connect(format!("{}:{}", address, socket));
        match result {
            Ok(stream) => {
                println!("connected to server");
                AnthemIP { stream }
            }
            Err(error) => {
                panic!("error: {}", error);
            }
        }
    }

    pub fn set_power(&mut self, power_on: bool) -> AnthemResult {
        self.write_cmd(AnthemCommand::Zone1Power(power_on))
    }

    pub fn set_current_input(&mut self, input_num: u8) -> AnthemResult {
        self.write_cmd(AnthemCommand::Zone1CurrentInput(input_num))
    }

    pub fn get_model(&mut self) -> AnthemResult {
        self.write_query_cmd(AnthemCommand::QueryModel)?;
        self.read_result()?;
        Ok(())
    }

    pub fn get_active_inputs(&mut self) -> Result<u8, AnthemError> {
        self.write_query_cmd(AnthemCommand::QueryActiveInputs)?;
        self.read_result()?;
        Ok(8)
    }

    pub fn get_active_input_name(&mut self, input_num: u8) -> Result<String, AnthemError> {
        self.write_query_cmd(AnthemCommand::QueryActiveInputName(input_num))?;
        self.read_result()
    }

    fn read_result(&mut self) -> Result<String, AnthemError> {
        let mut buff = vec![0; 64];
        let _ = self.stream.read(&mut buff);
        let result_buf = String::from_utf8(buff).unwrap_or_default();
        let result = result_buf.split(';').next().unwrap_or_default().to_owned();
        println!("{:?}", result);
        Ok(result)
    }

    fn write_cmd(&mut self, cmd: AnthemCommand) -> AnthemResult {
        self.write_cmd_string(&format!("{};", String::from(cmd)))
    }

    fn write_query_cmd(&mut self, cmd: AnthemCommand) -> AnthemResult {
        self.write_cmd_string(&format!("{}?;", String::from(cmd)))
    }

    fn write_cmd_string(&mut self, cmd: &str) -> AnthemResult {
        let result = self
            .stream
            .write_all(cmd.as_bytes());
        match result {
            Ok(_) => Ok(()),
            _ => Err(AnthemError::Io),
        }
    }
}
