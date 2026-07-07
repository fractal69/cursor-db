# cursor-db

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-GPLv3-blue.svg)](LICENSE)

A high-performance immutable cursor for binary tick databases.

`cursor-db` is designed for quantitative trading, backtesting, and market data replay. It memory-maps a binary file and exposes the data as an immutable slice with an ergonomic cursor API.

No parsing. No allocations. No copies.

---

## Features

- 🚀 Memory-mapped binary storage (`mmap`)
- 📈 Optimized for millions or billions of ticks
- 💾 Zero-copy reads
- 🎯 O(1) random access
- 🧭 Cursor navigation
- 🔒 Immutable data
- 🧵 Thread-safe read access
- ⚡ Minimal memory usage

---

## Binary Layout

Each file begins with a fixed-size header.

| Offset | Size | Type | Description |
|---------|-----:|------|-------------|
| 0 | 4 | `u32` | Magic number |
| 4 | 4 | `u32` | Version |
| 8 | 8 | `u64` | Trade count |

Each trade occupies **40 bytes**.

| Field | Type |
|--------|------|
| id | `u64` |
| time | `u64` |
| price | `u64` |
| qty | `u64` |
| is_buyer_maker | `u8` |
| padding | `[u8; 7]` |

---

# Example

```rust
use anyhow::Result;
use cursor_db::binary::BinaryFile;

fn main() -> Result<()> {
    let file = BinaryFile::open("ticks.bin")?;

    let mut cursor = file.cursor();

    println!("{:?}", cursor.current());

    cursor.next();

    println!("{:?}", cursor.current());

    cursor.seek(1_000_000);

    println!("{:?}", cursor.current());

    Ok(())
}
```

---

# Cursor API

```rust
cursor.current();

cursor.next();

cursor.prev();

cursor.seek(index);

cursor.move_by(offset);

cursor.index();

cursor.len();

cursor.is_end();

cursor.reset();
```

---

# Direct Access

Random access is O(1).

```rust
let trade = file.trade(5_000_000).unwrap();

println!("{:?}", trade);
```

---

# Sequential Iteration

```rust
let mut cursor = file.cursor();

loop {
    let trade = cursor.current().unwrap();

    // process trade

    if cursor.next().is_none() {
        break;
    }
}
```

---

# Performance

Unlike CSV files, binary data requires no parsing.

```text
CSV
 ├─ split(',')
 ├─ parse()
 ├─ decimal conversion
 ├─ allocations
 └─ objects

↓

Binary
 ├─ mmap()
 └─ &Trade
```

Each access is essentially equivalent to:

```rust
let trade = &trades[index];
```

---

# Design

```text
CSV
 │
 ▼
csv2bin
 │
 ▼
ticks.bin
 │
 ▼
Memory Map (mmap)
 │
 ▼
&[Trade]
 │
 ▼
Cursor
```

The cursor never owns data.

It only stores:

```rust
pub struct Cursor<'a> {
    trades: &'a [Trade],
    index: usize,
}
```

---

# Safety

The binary format is immutable.

Opening a file validates:

- Magic number
- Version
- File size
- Trade count

Corrupted files are rejected before reading any trade.

---

# Intended Use Cases

- Algorithmic trading
- Backtesting
- Tick replay
- Strategy simulation
- Quantitative research
- High-frequency datasets

---

# Why Binary?

Reading CSV requires parsing every field.

Binary files are already in their final representation.

```text
CSV

63114.900000

↓

parse()

↓

6311490000000

↓

u64
```

Binary files store:

```text
6311490000000
```

directly.

---

# License

This project is licensed under the **GNU General Public License v3.0 (GPL-3.0)**.

See the [LICENSE](LICENSE) file for details.