//! SSD1306 I2C Interface

use embedded_hal::{delay::DelayNs, i2c::I2c, i2c::ErrorType};

use super::DisplayInterface;
use crate::{command::Page, Error};

/// SSD1306 I2C communication interface
pub struct I2cInterface<I2C> {
    i2c: I2C,
    addr: u8,
}

impl<I2C> I2cInterface<I2C>
where
    I2C: I2c,
{
    /// Create new ssd1306 I2C interface
    pub fn new(i2c: I2C, addr: u8) -> Self {
        Self { i2c, addr }
    }
}

impl<I2C> DisplayInterface for I2cInterface<I2C>
// where
//     I2C: hal::blocking::i2c::Write<Error = CommE>,
where 
    I2C: I2c + ErrorType,
{
    type Error = Error;

    fn init(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn send_commands(&mut self, cmds: &[u8]) -> Result<(), Self::Error> {
        // Copy over given commands to new aray to prefix with command identifier
        let mut writebuf: [u8; 8] = [0; 8];
        writebuf[1..=cmds.len()].copy_from_slice(&cmds);
        if (cmds.len() == 1) {
            log::debug!("send_command : length = {} {:#04x}", cmds.len(), cmds[0]);
        } else if (cmds.len() > 1) {
            log::debug!("send_command : length = {} {:#04x} {:#04x}", cmds.len(), cmds[0], cmds[1]);
        }
        
        //log::debug!("send_commands, length = {}, command id = {:#04x}", cmds.len(), cmds[0]);
        Ok(self.i2c
            .write(self.addr, &writebuf[..=cmds.len()])?)
            //.map_err(Error::Comm)
    }

    fn send_data(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        // Display is always 128px wide
        const CHUNKLEN: usize = 128;

        const BUFLEN: usize = CHUNKLEN + 1;

        // Noop if the data buffer is empty
        if buf.is_empty() {
            return Ok(());
        }

        let mut page = Page::Page0 as u8;

        // Display width plus 4 start bytes
        let mut writebuf: [u8; BUFLEN] = [0; BUFLEN];

        writebuf[0] = 0x40; // Following bytes are data bytes
        log::debug!("send_data buf length is {}", buf.len());
        for chunk in buf.chunks(CHUNKLEN) {
            // Copy over all data from buffer, leaving the data command byte intact
            writebuf[1..BUFLEN].copy_from_slice(&chunk);

            self.i2c
                .write(
                    self.addr,
                    &[
                        0x00, // Command
                        page, // Page address
                        0x02, // Lower column address
                        0x10, // Upper column address (always zero, base is 10h)
                    ],
                )?;
                //.map_err(Error::Comm)?;

            self.i2c.write(self.addr, &writebuf)?; // .map_err(Error::Comm)?;

            page += 1;
        }

        Ok(())
    }
    
    

}
