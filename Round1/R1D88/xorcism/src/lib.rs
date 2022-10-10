use std::borrow::Borrow;

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],

    cnt: usize,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: ?Sized + AsRef<[u8]>>(key: &'a Key) -> Xorcism<'a> {
        Self {
            key: key.as_ref(),

            cnt: 0,
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        let mut it = self.key.iter().cycle();

        if self.cnt != 0 {
            it.nth(self.cnt - 1);
        }

        for (u, &v) in data.iter_mut().zip(it) {
            *u ^= v;

            self.cnt += 1;
        }
    }

    /// XOR each byte of the data with a byte from the key.
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<'b, Data, Item, Iter>(&'b mut self, data: Data) -> impl Iterator<Item = u8> + 'b
    where
        Data: IntoIterator<Item = Item, IntoIter = Iter>,
        Iter: Iterator<Item = Item> + 'b,
        Item: Borrow<u8> + 'b,
    {
        let mut it = self.key.iter().cycle();

        if self.cnt != 0 {
            it.nth(self.cnt - 1);
        }

        data.into_iter().zip(it).map(|(x, &y)| {
            self.cnt += 1;
            *x.borrow() ^ y
        })
    }
}





