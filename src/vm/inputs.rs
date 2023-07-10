use super::U256;
use smallvec::{smallvec, SmallVec};

pub struct Inputs(SmallVec<[U256; 8]>);

impl Inputs {
    pub fn new() -> Self {
        Self(smallvec![U256::from(1); 8])
    }

    pub fn increment(&mut self) {
        for i in 0..8 {
            self.0[i] += U256::from(1);
            if self.0[i] <= U256::from(6) {
                break;
            }
            self.0[i] = U256::from(1);
        }
    }

    pub fn inner(&self) -> &SmallVec<[U256; 8]> {
        &self.0
    }
}

impl std::fmt::Debug for Inputs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.0.iter().map(|e| e.as_limbs()[0]).rev())
            .finish()
    }
}
