//! Auto-generated file by build script, do not edit!
#[path = "y2024/mod.rs"]
pub mod y2024;
#[path = "y2023/mod.rs"]
pub mod y2023;
/// Selects the function for the given year, day, and part
pub fn select_function(
    year: u32,
    day: u32,
    part: u32,
) -> Result<fn(String) -> String, String> {
    match year {
        2024 => Ok(y2024::select_function(day, part)?),
        2023 => Ok(y2023::select_function(day, part)?),
        _ => Err("Invalid year!".into()),
    }
}
