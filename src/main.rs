mod auto_import;
use std::time::Instant;
use time::Month;

use aoc_client::{Client, SolutionDay};

fn main() {
    let solution_day = get_solution_day();
    let SolutionDay { year, day, part } = solution_day;
    let client = Client::new().unwrap();
    println!("Getting input for year {} day {}...", year, day);
    let input = client.get_input(&solution_day).unwrap();
    // get puzzle input
    println!(
        "Getting function for year {} day {} part {}...",
        year, day, part
    );
    // run puzzle solution
    let func = auto_import::select_function(year, day as u32, part as u32).unwrap();
    println!("Running function...");
    let now = Instant::now();
    let result = func(input);
    println!("Completed in: {}ms", now.elapsed().as_secs_f64() * 1000.0);
    // print result
    println!("Result: {}", result);
    // submit result?
    if !prompt_for_input(
        "Press enter to exit or type anything to submit answer",
        "".to_string(),
    )
    .is_empty()
    {
        let res = client.submit_solution(&solution_day, &result).unwrap();
        println!("{}", res);
    }
}

fn get_solution_day() -> SolutionDay {
    let args = std::env::args().collect::<Vec<String>>();
    let current_time = time::OffsetDateTime::now_utc();
    // get year
    let current_year = current_time.year() as u32;
    let prompt = format!("Enter year(default {}):", current_year);
    let year = match args.get(1) {
        Some(arg) => arg
            .parse::<u32>()
            .unwrap_or_else(|_| prompt_for_input(&prompt, current_year)),
        None => prompt_for_input(&prompt, current_year),
    };
    // get day
    let default_day = if current_time.month() == Month::December {
        current_time.day()
    } else {
        1
    };
    let prompt = format!("Enter day(default {}):", default_day);
    let day = if let Some(arg) = args.get(2) {
        arg.parse::<u8>()
            .unwrap_or_else(|_| prompt_for_input(&prompt, default_day))
    } else {
        prompt_for_input(&prompt, default_day)
    };
    // get part
    let part = if let Some(arg) = args.get(3) {
        arg.parse::<u8>()
            .unwrap_or_else(|_| prompt_for_input("Enter part(default 1):", default_day))
    } else {
        prompt_for_input("Enter part(default 1):", 1)
    };
    SolutionDay::create(year, day, part)
}

fn prompt_for_input<T: std::str::FromStr>(prompt: &str, default: T) -> T {
    let mut stdin = String::new();
    println!("{}", prompt);
    std::io::stdin().read_line(&mut stdin).unwrap();
    stdin.trim().parse::<T>().unwrap_or(default)
}
