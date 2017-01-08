
pub struct Crc16 {
    pub divisor: u16,
    pub initial_reminder: u16,
    pub table: Vec<u16>,
    pub reversed_input: bool,
    pub reverser_output: bool,
    pub xorout: u16
}

impl Crc16 {

    /// Precomputes table for CRC calculation
    ///
    /// *Note*: Calling calcuate CRC without having precomputed table will result
    /// in panic.
    pub fn create_table(&mut self) {
        for i in 0..256 {
            let mut value: u16 = i;
            value = value << 8;
            for _ in 0..8 {
                if (value & 0x8000) != 0 {
                    value = (value << 1) ^ self.divisor;
                } else {
                    value = value << 1;
                }
            }
            self.table.push(value);
        }
    }

    /// Calculates CRC16 value for given byte array
    ///
    /// # Arguments
    ///
    /// * `bytes` - A byte array whose value is being calculated.
    pub fn calculate(&self, bytes: &[u8]) -> u16 {
        self.calculate_rolling(bytes, self.initial_reminder)
    }

    /// Calculates CRC16 value for given byte array, but assumes 
    /// calculation is resumed from previous state (multiple_parts)
    ///
    /// # Arguments
    ///
    /// * `bytes` - A byte array whose value is being calculated.
    /// * `current_value` - A CRC16 value obtained when calculating CRC16 of 
    /// previous part.
    pub fn calculate_rolling(&self, bytes: &[u8], current_value: u16) -> u16 {
         assert_eq!(self.table.is_empty(), false, "Table was not initialized.");
         assert_eq!(self.table.len(), 256, "Table size was incorrect.");

         let mut value: u16 = current_value;
         for &i in bytes.iter() {
             let current_byte: u8 = if self.reversed_input {
                 reflect(i as u16, 8) as u8
             }
             else {
                 i
             };
             value = self.table[((value >> 8 ) ^ current_byte as u16) as usize] ^ (value << 8);
         }
         
         if self.reverser_output {
             reflect(value ^ self.xorout, 16)
         }
         else {
             value ^ self.xorout
         }
    }

    /// Checks if byte array has valid CRC appended
    /// are continuing calculation from previous state (multiple_parts)
    ///
    /// # Arguments
    ///
    /// * `bytes` - A byte array whose value is being checked.
    ///
    /// *Note*: Assuming that last two bytes in array are CRC16 value that 
    /// was appended, this only works if whole data we are checking is in byte
    /// array. For larger files we can check validity by calculating CRC value
    /// again and checking if it is 0.
    pub fn check(&self, bytes: &[u8]) -> bool {
        return self.calculate(bytes) == 0u16;
    }        

}

fn reflect(data: u16, bit_size: u8) -> u16
{
    let mut reflection: u16 = 0x0000;

    for i in 0..bit_size {
        if (data >> i) & 0x01 == 1 {
            reflection |= 1 << ((bit_size - 1) - i);
        }
    }

    reflection
}

