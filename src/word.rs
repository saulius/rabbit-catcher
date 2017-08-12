use std::fmt;
use std::ops::Sub;
use std::str::Chars;
use simd::i8x16;

lazy_static! {
    static ref ALPHABET_LENGTH: usize = 26;
    pub static ref ZERO_VEC: i8x16 = i8x16::splat(0);
}

#[derive(Copy)]
pub struct Histogram {
    pub value: i8x16
}

impl Histogram {
    #[inline]
    pub fn from_chars(chars: Chars) -> Self {
        let mut value = vec![0; *ALPHABET_LENGTH];

        for chr in chars {
            value[(chr as i8 - 'a' as i8) as usize] += 1
        }

        Histogram {
            value: i8x16::new(
                value[0],  // letter: a
                value[8],  // letter: i
                value[11], // letter: l
                value[13], // letter: n
                value[14], // letter: o
                value[15], // letter: p
                value[17], // letter: r
                value[18], // letter: s
                value[19], // letter: t
                value[20], // letter: u
                value[22], // letter: w
                value[24], // letter: y
                0,
                0,
                0,
                value[1] + value[2] + value[3] + value[4] + value[5] +
                    value[6] + value[7] + value[9] + value[10] + value[12] +
                    value[16] + value[21] + value[23] + value[25])
        }
    }
}

impl Sub for Histogram {
    type Output = Histogram;

    #[inline]
    fn sub(self, other: Histogram) -> Histogram {
        Histogram { value: self.value - other.value }
    }
}

impl fmt::Debug for Histogram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Histogram<{:?}>", self.value)
    }
}

impl Clone for Histogram {
    #[inline]
    fn clone(&self) -> Self {
        Histogram { value: self.value }
    }
}

pub struct Word {
    pub value: Vec<u8>,
    pub histo: Histogram,
}

impl Word {
    #[inline]
    pub fn from_string(s: String) -> Self {
        Word {
            value: s.clone().into_bytes(),
            histo: Histogram::from_chars(s.chars()),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.value.len()
    }

    #[inline]
    pub fn is_superset_of(&self, word: &Word) -> bool {
        (self.histo.value - word.histo.value).ge(*ZERO_VEC).all()
    }
}

impl PartialEq for Word {
    #[inline]
    fn eq(&self, other: &Word) -> bool {
        self.value == other.value
    }
}

impl fmt::Debug for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Word<{} {:?}>", self.value, self.histo)
        write!(f, "Word<{:?}>", self.value)
    }
}

impl Clone for Word {
    #[inline]
    fn clone(&self) -> Self {
        Word {
            value: self.value.clone(),
            histo: self.histo.clone(),
        }
    }
}
