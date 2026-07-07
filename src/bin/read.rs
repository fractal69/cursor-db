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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trades_are_time_sorted() {
        let file = BinaryFile::open("./output/ticks.bin").unwrap();

        let mut previous = file.trade(0).unwrap();

        for index in 1..file.len() {
            let current = file.trade(index).unwrap();

            assert!(
                previous.time <= current.time,
                "Time order violated at index {}: {} > {}",
                index,
                previous.time,
                current.time,
            );

            previous = current;
        }
    }
}
