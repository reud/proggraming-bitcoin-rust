use crate::helper::helper::read_varint;
use std::io::{Cursor, Read};
use num_bigint::{BigInt, Sign};

pub struct Script {
    cmds: Vec<BigInt>
}

pub fn new_empty_script() -> Script {
    return Script {
        cmds: vec![],
    }
}

pub fn new_script(cmds: Vec<BigInt>) -> Script {
    Script {
        cmds,
    }
}

impl Script {
    #[allow(dead_code)]
    pub fn parse(c: &mut Cursor<Vec<u8>>) -> Script {
        let length = read_varint(c);
        let mut cmds:Vec<BigInt> = vec![];
        let mut count = 0;
        while count < length {
            let mut one_byte_buf = [0u8];
            let read_result = c.read(&mut one_byte_buf);
            if read_result.is_err() {
                panic!(read_result.err().unwrap());
            }

            count += 1;
            let current_byte = one_byte_buf[0];

            if current_byte >= 1 && current_byte <= 75 {
                let read_len = current_byte;
                let mut buf: Vec<u8> = vec![0u8; read_len as usize];

                let read_result = c.read(&mut buf);
                if read_result.is_err() {
                    panic!(read_result.err().unwrap());
                }
                let bi = BigInt::from_bytes_le(Sign::NoSign, &*buf);
                cmds.push(bi);
                count += read_len;
            } else if current_byte == 76 {
                let read_len = current_byte;
                let mut buf: Vec<u8> = vec![0u8; read_len as usize];

                let read_result = c.read(&mut buf);
                if read_result.is_err() {
                    panic!(read_result.err().unwrap());
                }
                let bi = BigInt::from_bytes_le(Sign::NoSign, &*buf);
                cmds.push(bi);
                count += read_len;
            }
        }
        return new_script(cmds);
    }
}