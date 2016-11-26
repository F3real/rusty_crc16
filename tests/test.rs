extern crate rusty_crc;

#[cfg(test)]
mod tests {

    use rusty_crc::crc::Crc16;
    use rusty_crc::crc_builder::CrcBuilder;
    
    
    #[test]
    fn check_crc16_ccitt_false() {          
        let crc : Crc16 =  CrcBuilder::generate_crc("crc16_ccitt_false").unwrap();

        let raw_bytes: &[u8] = "123456789".as_bytes();
        assert_eq!(crc.calculate(raw_bytes), 0x29B1, "Incorrectly calculated CRC.");

        let raw_bytes: &[u8] = &[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x29, 0xB1];
        assert_eq!(crc.check(raw_bytes), true, "CRC verification failed.");
    }


    #[test]
    fn check_crc16_kermit() {          
        let crc : Crc16 =  CrcBuilder::generate_crc("crc16_kermit").unwrap();

        let raw_bytes: &[u8] = "123456789".as_bytes();
        assert_eq!(crc.calculate(raw_bytes), 0x2189, "Incorrectly calculated CRC.");

        /*note when checking we need to append bytes in reverse order*/
        let raw_bytes: &[u8] = &[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x89, 0x21];
        assert_eq!(crc.check(raw_bytes), true, "CRC verification failed.");
    }

}
