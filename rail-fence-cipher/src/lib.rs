pub struct RailFence {
    rails: u32,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence { rails }
    }

    fn rail_index(&self, pos: usize) -> u32 {
        if self.rails <= 1 {
            return 0;
        }
        let cycle_len = 2 * (self.rails - 1) as usize;
        let pos_in_cycle = pos % cycle_len;
        if pos_in_cycle < self.rails as usize {
            pos_in_cycle as u32
        } else {
            2 * (self.rails - 1) - pos_in_cycle as u32
        }
    }

    pub fn encode(&self, text: &str) -> String {
        if self.rails <= 1 {
            return text.to_string();
        }
        let chars: Vec<char> = text.chars().collect();
        let mut rails: Vec<Vec<char>> = (0..self.rails).map(|_| Vec::new()).collect();
        for (i, &c) in chars.iter().enumerate() {
            let rail = self.rail_index(i) as usize;
            rails[rail].push(c);
        }
        rails.into_iter().flat_map(|r| r).collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        if self.rails <= 1 {
            return cipher.to_string();
        }
        let len = cipher.chars().count();
        let mut rail_indices: Vec<Vec<usize>> = (0..self.rails).map(|_| Vec::new()).collect();
        for i in 0..len {
            let rail = self.rail_index(i) as usize;
            rail_indices[rail].push(i);
        }
        let mut result: Vec<char> = vec![' '; len];
        let mut char_iter = cipher.chars();
        for indices in &rail_indices {
            for &pos in indices {
                if let Some(c) = char_iter.next() {
                    result[pos] = c;
                }
            }
        }
        result.into_iter().collect()
    }
}
