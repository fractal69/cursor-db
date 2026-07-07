use crate::trade::Trade;

pub struct Cursor<'a> {
    trades: &'a [Trade],
    index: usize,
}

impl<'a> Cursor<'a> {
    #[inline]
    pub fn new(trades: &'a [Trade]) -> Self {
        Self {
            trades,
            index: 0,
        }
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