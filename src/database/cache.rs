use evmap::{WriteHandle, ReadHandle};
use super::models::{barset::BarSet, bar::Bar};


#[derive(Debug)]
pub struct InMemoryCache {
    read_handle: ReadHandle<String, BarSet>,
    write_handle: WriteHandle<String, BarSet>,
}

impl InMemoryCache {
    // create a new cache instance
    pub fn new() -> Self {
        // let data: HashMap<String, Vec<BarSet>> = HashMap::new();
        let (read_handle, write_handle): (ReadHandle<String, BarSet>, WriteHandle<String, BarSet>) = evmap::new();

        Self {
            read_handle,
            write_handle,
        }
    }

    // get a set number of bars from the cache
    pub fn get(&mut self, key: String, limit: i32) -> Option<BarSet> {
        // obtain a read handle to the cache
        let read_handle = self.read_handle.read();
        if let Some(view) = &read_handle {
            // search for the key in the cache
            if let Some(barset) = view.get_one(&key) {
                // check if the limit is greater than the number of bars
                if limit > barset.bars.len() as i32 {
                    // remove the bars from the cache
                    let empty_barset = BarSet::new();
                    self.write_handle.remove(key, empty_barset);
                    self.write_handle.refresh();

                    // return all the bars
                    return Some(barset.clone());
                }

                // get bars up to the limit
                let bars = barset.bars[0..limit as usize].to_vec();
                let is_last = barset.is_last.clone();

                // remove the bars that are returned from the cache
                let remaining_bars = barset.bars[limit as usize..].to_vec();
                if !remaining_bars.is_empty() {
                    self.write_handle.update(key.clone(), BarSet::new_with(remaining_bars, is_last));
                }
                else {
                    self.write_handle.remove(key.clone(), BarSet::new());
                }
                self.write_handle.refresh();

                // return the barsets up to the limit
                return Some(BarSet::new_with(bars, is_last));
            }
        }

        None
    }

    // add a set of barsets to the cache
    pub fn add(&mut self, key: String, bars: Vec<Bar>, is_last: bool) -> bool {
        // get a read handle to the cache
        let barset = if let Some(existing_barset) = self.write_handle.get_one(&key) {
            let existing_bars = existing_barset.bars.clone();
            // let is_last = existing_barset.is_last.clone();
            BarSet::new_with(bars.clone().into_iter().chain(existing_bars).collect(), is_last)
        }
        else {
            BarSet::new_with(bars, is_last)
        };

        // add or update the cache record
        self.write_handle.update(key.clone(), barset);
        self.write_handle.refresh();

        true
    }
}