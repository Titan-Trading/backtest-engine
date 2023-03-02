use std::mem::ManuallyDrop;

use evmap::ShallowCopy;
use super::{bar::Bar};


// represents a set of bars linked by timestamp across multiple symbols and exchanges
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct BarSet {
    pub is_last: bool,
    pub bars: Vec<Bar>,
}

impl BarSet {
    pub fn new() -> Self {
        Self {
            is_last: false,
            bars: Vec::new(),
        }
    }

    pub fn new_with(bars: Vec<Bar>, is_last: bool) -> Self {
        Self {
            is_last,
            bars,
        }
    }

    // add a bar to the barset using the source id as the key
    // example source id would be "binance:BTCUSDT"
    pub fn add_bar(&mut self, bar: Bar) {
        self.bars.push(bar);
    }
}

impl Eq for BarSet {}

impl ShallowCopy for BarSet {
    unsafe fn shallow_copy(&self) -> ManuallyDrop<Self> {
        let cloned_is_last = self.is_last.clone();
        let cloned_bars = self.bars.clone();
        let cloned_barset = Self {
            is_last: cloned_is_last,
            bars: cloned_bars,
        };
        ManuallyDrop::new(cloned_barset)
    }
}