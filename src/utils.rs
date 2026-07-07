use crate::trade::{Header, Trade};
use std::io::Write;
use anyhow::Result;

pub fn write_header<W: Write>(writer: &mut W, header: &Header) -> Result<()> {
    writer.write_all(&header.magic.to_le_bytes())?;
    writer.write_all(&header.version.to_le_bytes())?;
    writer.write_all(&header.trade_count.to_le_bytes())?;
    Ok(())
}

pub fn write_trade<W: Write>(writer: &mut W, trade: &Trade) -> Result<()> {
    writer.write_all(&trade.id.to_le_bytes())?;
    writer.write_all(&trade.time.to_le_bytes())?;
    writer.write_all(&trade.price.to_le_bytes())?;
    writer.write_all(&trade.qty.to_le_bytes())?;
    writer.write_all(&[trade.is_buyer_maker])?;
    writer.write_all(&[0; 7])?; // padding

    Ok(())
}
