use anyhow::Result;
use csv::Reader;

use cursor_db::{
    trade::{Header, MAGIC, Trade, VERSION},
    utils::{write_header, write_trade},
};
use serde::Deserialize;
use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Seek, SeekFrom, Write},
};

#[derive(Debug, Deserialize)]
struct CsvTrade {
    id: u64,
    time: u64,
    price: u64,
    qty: u64,
    is_buyer_maker: u8,
}

fn main() -> Result<()> {
    let input = "./output/output.csv";
    let output = "./output/ticks.bin";

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(input)?;

    let mut writer = BufWriter::new(File::create(output)?);

    // Header temporal
    let header = Header {
        magic: MAGIC,
        version: VERSION,
        trade_count: 0,
    };

    write_header(&mut writer, &header)?;

    let mut count = 0u64;

    for row in reader.deserialize() {
        let csv: CsvTrade = row?;

        let trade = Trade {
            id: csv.id,
            time: csv.time,
            price: csv.price,
            qty: csv.qty,
            is_buyer_maker: csv.is_buyer_maker,
            _padding: [0; 7],
        };

        write_trade(&mut writer, &trade)?;

        count += 1;
    }

    writer.flush()?;
    drop(writer);

    // Actualizar trade_count
    let mut file: File = OpenOptions::new().write(true).open(output)?;

    file.seek(SeekFrom::Start(8))?;
    file.write_all(&count.to_le_bytes())?;

    println!("Trades escritos: {}", count);

    Ok(())
}
