use crc::Crc16;

pub struct CrcBuilder{

}

impl CrcBuilder{
    
    pub fn generate_crc(algorithm: &str) -> Result<Crc16, &'static str> {
        match algorithm {

            "crc16_ccitt_false" => {
                let mut crc =
                Crc16 {
                    divisor: 0x1021,
                    initial_reminder: 0xFFFF,
                    table: Vec::with_capacity(256),
                    reversed_input:false,
                    reverser_output:false,
                    xorout:0x0000};
                crc.create_table();
                Ok(crc)
            },

            "crc16_kermit" => {
                let mut crc =
                Crc16 {
                    divisor: 0x1021,
                    initial_reminder: 0x0000,
                    table: Vec::with_capacity(256),
                    reversed_input:true,
                    reverser_output:true,
                    xorout:0x0000};
                crc.create_table();
                Ok(crc)
            },
            
             _ => Err("Unsupported algoritham specified"),
        }

    }

}