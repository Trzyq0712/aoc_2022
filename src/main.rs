use std::env::args;

mod solvs;
mod utils;

fn main() -> Result<(), &'static str> {
    let day = args()
        .nth(1)
        .ok_or("No day argument. Give a number")?
        .parse::<u32>()
        .map_err(|_| "Not a number")?;

    let solution = solvs::get_solution(day).ok_or("No solution for that day")?;
    solution.run()?;

    Ok(())
}
