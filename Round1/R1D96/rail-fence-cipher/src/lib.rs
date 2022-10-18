pub struct RailFence(u32);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence(rails)
    }

    pub fn encode(&self, text: &str) -> String {
        let cycle = (0..(self.0 - 1)).chain((1..self.0).rev()).cycle();

        let mut vec: Vec<_> = cycle.zip(text.char_indices()).collect();

        vec.sort();

        vec.iter().map(|x| x.1 .1).collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let cycle = (0..(self.0 - 1)).chain((1..self.0).rev()).cycle();

        let mut vec: Vec<_> = cycle.zip(0..cipher.len()).collect();

        vec.sort();

        let mut vec: Vec<_> = cipher
            .chars()
            .zip(vec.iter())
            .map(|(a, (_, c))| (c, a))
            .collect();

        vec.sort();

        vec.iter().map(|(_, c)| c).collect()
    }
}
