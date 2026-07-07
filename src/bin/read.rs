use anyhow::Result;
use cursor_db::binary::BinaryFile;

fn main() -> Result<()> {
    let binary_path: &str = "./output/ticks.bin";

    let file: BinaryFile = BinaryFile::open(&binary_path)?;

    println!("Trades: {}", file.len());

    println!("{:#?}", file.header());

    println!("{:#?}", file.trade(0));

    println!("{:#?}", file.trade(file.len() - 1));

    Ok(())
}
