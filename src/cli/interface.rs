use std::path::PathBuf;

use hex::FromHex;
use structopt::StructOpt;

use crate::apdu::capdu::CryptogramType;
use crate::utils::extension::Hexadecimal;

#[derive(StructOpt)]
pub struct Emv {
    #[structopt(subcommand)]
    pub mode: Mode,
}

#[derive(StructOpt)]
pub enum Mode {
    Shell,
    Run {
        #[structopt(parse(from_os_str))]
        input: PathBuf
    },
}

#[derive(Debug)]
pub enum Command {
    Select {
        application: Vec<u8>
    },
    GetProcessingOptions,
    ReadRecord {
        record: u8,
        sfi: u8,
    },
    GenerateAC {
        cryptogram_type: CryptogramType,
        cdol: Option<Vec<u8>>,
    },
    PutData {
        tag: u16,
        value: Vec<u8>,
    },
    GetData {
        tag: u16
    },
    Verify {
        pin: Vec<u8>
    },
    PinUnblock,
    PinChange {
        pin: Vec<u8>
    }
}

impl Command {
    pub fn from_str(str: String) -> Result<Command, String> {
        let parts: Vec<&str> = str.trim().split(' ').collect();
        let name = parts[0].to_lowercase();

        match name.as_str() {
            "select" => Ok(Command::Select {
                application: parts[1].to_vec_u8()
            }),
            "get_processing_options" => Ok(Command::GetProcessingOptions),
            "generate_ac" => {
                let mut cdol = None;
                if parts.len() > 2 {
                    cdol = Some(parts[2].to_vec_u8());
                }
                Ok(Command::GenerateAC {
                    cryptogram_type: CryptogramType::from_str(parts[1]),
                    cdol,
                })
            }
            "get_data" => Ok(Command::GetData {
                tag: parts[1].to_u16()
            }),
            "put_data" => Ok(Command::PutData {
                tag: parts[1].to_u16(),
                value: parts[2].to_vec_u8(),
            }),
            "read_record" => Ok(Command::ReadRecord {
                record: parts[1].to_u8(),
                sfi: parts[2].to_u8(),
            }),
            "verify" => Ok(Command::Verify {
                pin: parts[1].to_vec_u8()
            }),
            "pin_unblock" => Ok(Command::PinUnblock),
            "pin_change" => Ok(Command::PinChange {
                pin: parts[1].to_vec_u8()
            }),
            _ => Err(name)
        }
    }
}
