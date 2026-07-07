use crate::trade::Trade;

pub struct Cursor<'a> {
    trades: &'a [Trade],
    index: usize,
}

impl<'a> Cursor<'a> {
    #[inline]
    pub fn new(trades: &'a [Trade]) -> Self {
        Self { trades, index: 0 }
    }

    #[inline]
    pub fn current(&self) -> Option<&'a Trade> {
        self.trades.get(self.index)
    }

    #[inline]
    pub fn next(&mut self) -> Option<&'a Trade> {
        if self.index + 1 >= self.trades.len() {
            return None;
        }

        self.index += 1;
        self.current()
    }

    #[inline]
    pub fn prev(&mut self) -> Option<&'a Trade> {
        if self.index == 0 {
            return None;
        }

        self.index -= 1;
        self.current()
    }

    #[inline]
    pub fn seek(&mut self, index: usize) -> Option<&'a Trade> {
        if index >= self.trades.len() {
            return None;
        }

        self.index = index;
        self.current()
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.trades.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.trades.is_empty()
    }

    #[inline]
    pub fn is_end(&self) -> bool {
        self.index + 1 >= self.trades.len()
    }

    #[inline]
    pub fn reset(&mut self) {
        self.index = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary::BinaryFile;

    #[test]
    fn cursor_matches_direct_access() {
        let file = BinaryFile::open("./output/ticks.bin").unwrap();

        let mut cursor = file.cursor();

        let indices = [
            0,
            1,
            10,
            100,
            1_000,
            file.len() / 2,
            file.len() - 2,
            file.len() - 1,
        ];

        for index in indices {
            let direct = file.trade(index);
            let cursor_trade = cursor.seek(index);

            assert_eq!(direct, cursor_trade);
        }
    }

    #[test]
    fn next_prev_are_inverse() {
        let file = BinaryFile::open("./output/ticks.bin").unwrap();

        let mut cursor = file.cursor();

        let indices = [0, 1, 10, 100, 1_000, file.len() / 2, file.len() - 2];

        for index in indices {
            cursor.seek(index).unwrap();

            let original = *cursor.current().unwrap();

            cursor.next().unwrap();
            cursor.prev().unwrap();

            assert_eq!(original, *cursor.current().unwrap());
            assert_eq!(cursor.index(), index);
        }
    }

    #[test]
    fn cursor_is_sequential() {
        let file = BinaryFile::open("./output/ticks.bin").unwrap();

        let mut cursor = file.cursor();

        for index in 0..file.len() {
            assert_eq!(cursor.current(), file.trade(index));

            if index + 1 < file.len() {
                cursor.next().unwrap();
            }
        }
    }

    #[test]
    fn next_stops_at_end() {
        let file = BinaryFile::open("./output/ticks.bin").unwrap();

        let mut cursor = file.cursor();

        cursor.seek(file.len() - 1).unwrap();

        let last = *cursor.current().unwrap();

        assert!(cursor.next().is_none());

        // El cursor debe seguir apuntando al último trade.
        assert_eq!(cursor.index(), file.len() - 1);
        assert_eq!(*cursor.current().unwrap(), last);
    }

    #[test]
    fn prev_stops_at_beginning() {
        let file = BinaryFile::open("./output/ticks.bin").unwrap();

        let mut cursor = file.cursor();

        let first = *cursor.current().unwrap();

        assert_eq!(cursor.index(), 0);

        assert!(cursor.prev().is_none());

        // El cursor debe seguir apuntando al primer trade.
        assert_eq!(cursor.index(), 0);
        assert_eq!(*cursor.current().unwrap(), first);
    }
}
