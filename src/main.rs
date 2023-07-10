pub mod solver;
pub mod vm;

use std::env;
use vm::U256;

fn main() {
    // return _check_all();
    let args: Vec<String> = env::args().collect();

    let inputs = match sanitize(&args) {
        Ok(inputs) => inputs,
        Err(err) => {
            println!("ERROR: {}", err);
            return;
        }
    };

    match solver::solve(&inputs) {
        Some(solution) => println!(
            "solution found in {} iterations: {:?}",
            solution.iterations, solution.opcodes
        ),
        None => println!("no solution found"),
    }
}

fn sanitize(args: &[String]) -> Result<smallvec::SmallVec<[U256; 8]>, String> {
    if args.len() != 2 {
        println!("usage `cargo run --release -- <input>`");
        println!("note: `<input>` is in the following format `11111111`");
    }

    let inputs: Vec<u32> = args[1].chars().map(|c| c.to_digit(10).unwrap()).collect();

    if inputs.len() != 8 {
        return Err("input must be 8 digits long".to_string());
    }

    for &input in inputs.iter() {
        if input > 6 || input < 1 {
            return Err("input must be between 1 and 6".to_string());
        }
    }

    Ok(smallvec::smallvec![
        U256::from(inputs[0]),
        U256::from(inputs[1]),
        U256::from(inputs[2]),
        U256::from(inputs[3]),
        U256::from(inputs[4]),
        U256::from(inputs[5]),
        U256::from(inputs[6]),
        U256::from(inputs[7]),
    ])
}

fn _check_all() {
    use std::io::Write;
    const PERMUTATIONS: usize = 262_144;
    let mut inputs = vm::Inputs::new();
    let mut lock = std::io::stdout().lock();

    for _ in 0..PERMUTATIONS {
        let input = U256::from(
            inputs
                .inner()
                .iter()
                .enumerate()
                .fold(0, |acc, (i, input)| acc | input.as_limbs()[0] << (8 * i)),
        );

        let solution = solver::solve(inputs.inner())
            .unwrap()
            .opcodes
            .iter()
            .enumerate()
            .fold(U256::ZERO, |acc, (i, opcode)| {
                acc | U256::from(opcode.to_u8()) << (248 - 8 * i)
            });

        write!(lock, "{:x}{:x}\n", input, solution).unwrap();
        inputs.increment();
    }
}
