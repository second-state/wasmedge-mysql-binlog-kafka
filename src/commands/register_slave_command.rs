use crate::commands::command_type::CommandType;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Cursor;

/// Used for MariaDB Gtid replication.
/// See <a href="https://mariadb.com/kb/en/com_register_slave/">MariaDB docs</a>
/// See <a href="https://dev.mysql.com/doc/internals/en/com-register-slave.html">MySQL docs</a>
pub struct RegisterSlaveCommand {
    pub server_id: u32,
}

impl RegisterSlaveCommand {
    pub fn new(server_id: u32) -> Self {
        Self { server_id }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        let mut cursor = Cursor::new(&mut vec);

        cursor.write_u8(CommandType::RegisterSlave as u8).unwrap();
        cursor.write_u32::<LittleEndian>(self.server_id).unwrap();

        //Empty host, user, password, port, rank, masterid
        cursor.write_u8(0).unwrap();
        cursor.write_u8(0).unwrap();
        cursor.write_u8(0).unwrap();
        cursor.write_u16::<LittleEndian>(0).unwrap();
        cursor.write_u32::<LittleEndian>(0).unwrap();
        cursor.write_u32::<LittleEndian>(0).unwrap();

        vec
    }
}
