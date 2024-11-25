//! Auto-generated file by build script, do not edit!
pub mod d4;
pub mod d1;
pub mod d5;
pub mod d2;
pub mod d3;
/// Selects the function for the given day and part
pub fn select_function(day: u32, part: u32) -> Result<fn(String) -> String, String> {
    match day {
        4 => {
            match part {
                1 => Ok(d4::part1),
                2 => Ok(d4::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        1 => {
            match part {
                1 => Ok(d1::part1),
                2 => Ok(d1::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        5 => {
            match part {
                1 => Ok(d5::part1),
                2 => Ok(d5::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        2 => {
            match part {
                1 => Ok(d2::part1),
                2 => Ok(d2::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        3 => {
            match part {
                1 => Ok(d3::part1),
                2 => Ok(d3::part2),
                _ => Err("Invalid part!".into()),
            }
        }
        _ => Err("Invalid day!".into()),
    }
}
