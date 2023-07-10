mod inputs;
mod opcode;

pub use inputs::Inputs;
pub use opcode::Opcode;

use ruint::Uint;
use smallvec::SmallVec;

pub type U256 = Uint<256, 4>;

pub fn execute(opcodes: &[u8], inputs: &SmallVec<[U256; 8]>) -> Option<U256> {
    if inputs.len() < 8 {
        panic!("inputs too short");
    }

    let mut inputs = inputs.clone();

    for &opcode in opcodes.iter() {
        match opcode {
            0 => return inputs.pop(),
            1 => {
                let a = inputs.pop()?;
                let b = inputs.pop()?;
                inputs.push(a.overflowing_add(b).0)
            }
            2 => {
                let a = inputs.pop()?;
                let b = inputs.pop()?;
                inputs.push(a.overflowing_sub(b).0)
            }
            3 => {
                let a = inputs.pop()?;
                let b = inputs.pop()?;
                inputs.push(a.overflowing_mul(b).0)
            }
            4 => {
                let a = inputs.pop()?;
                let b = inputs.pop()?;
                if b == U256::ZERO {
                    inputs.push(U256::ZERO);
                } else {
                    inputs.push(a / b);
                }
            }
            n => {
                let swap_depth = n as usize - 4;
                let index = inputs.len() - 1;
                inputs.swap(index, index - swap_depth);
            }
        }
    }

    None
}
