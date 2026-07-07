use anyhow::Result;
use cursor_db::binary::BinaryFile;

fn main() -> Result<()> {
    let file = BinaryFile::open("./output/ticks.bin")?;

    let mut cursor = file.cursor();

    println!("{:#?}", cursor.current());
    println!("{:#?}", cursor.prev());
    println!("{:#?}", cursor.next());
    println!("{:#?}", cursor.seek(1_000_000));
    println!("{:#?}", cursor.prev());
 
    Ok(())
}
