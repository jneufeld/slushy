/// A height value is an integer where larger values indicate greater heights
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Height {
    value: u8,
}

impl Height {
    pub fn can_move_to(&self, other: Height) -> bool {
        self.value >= other.value - 1
    }
}

impl Height {
    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn is_lowest(&self) -> bool {
        self.value == b'a'
    }
}

impl From<char> for Height {
    fn from(c: char) -> Self {
        let value = match c {
            'S' => 'a',
            'E' => 'z',
            _ => c,
        };

        let value = value as u8;

        Height { value }
    }
}
