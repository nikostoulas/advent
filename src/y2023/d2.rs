#[derive(Debug)]
pub struct Game {
    pub sets: Vec<Vec<(Qube, u32)>>,
    pub game: u32,
}

pub struct TotalQubes {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(Debug)]
pub enum Qube {
    Red,
    Green,
    Blue,
}

impl Qube {
    pub fn create(s: String) -> Qube {
        match s.as_str() {
            "red" => Qube::Red,
            "green" => Qube::Green,
            "blue" => Qube::Blue,
            _ => panic!("Invalid qube"),
        }
    }
}

impl Game {
    pub fn from_line(line: &str) -> Game {
        let mut sets: Vec<Vec<(Qube, u32)>> = Vec::new();
        let game = line
            .split("Game")
            .nth(1)
            .unwrap()
            .split(":")
            .next()
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        line.split(':')
            .nth(1)
            .unwrap()
            .split(';')
            .for_each(|set_str| {
                let mut set: Vec<(Qube, u32)> = Vec::new();
                set_str.split(',').for_each(|s| {
                    let number: u32 = s.trim().split(' ').next().unwrap().parse().unwrap();
                    let qube_str = s.trim().split(' ').nth(1).unwrap();
                    let qube = Qube::create(qube_str.to_string());
                    set.push((qube, number));
                });
                sets.push(set);
            });
        Game { sets, game }
    }
}

impl TotalQubes {
    pub fn create(input: &Game) -> TotalQubes {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        input.sets.iter().for_each(|set| {
            let mut local_red = 0;
            let mut local_green = 0;
            let mut local_blue = 0;
            set.iter().for_each(|(qube, number)| match qube {
                Qube::Red => local_red += number,
                Qube::Green => local_green += number,
                Qube::Blue => local_blue += number,
            });
            red = red.max(local_red);
            green = green.max(local_green);
            blue = blue.max(local_blue);
        });
        TotalQubes { red, green, blue }
    }
}

pub fn part1(input: String) -> String {
    let lines = input
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut games: Vec<Game> = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let input = Game::from_line(&line);
        games.push(input)
    }
    let total_game = Game {
        sets: vec![vec![(Qube::Red, 12), (Qube::Blue, 14), (Qube::Green, 13)]],
        game: 0,
    };
    let total = TotalQubes::create(&total_game);
    let mut sum = 0;
    for game in games {
        let total_qube = TotalQubes::create(&game);
        if total_qube.red <= total.red
            && total_qube.green <= total.green
            && total_qube.blue <= total.blue
        {
            sum += game.game;
        }
    }
    sum.to_string()
}

pub fn part2(input: String) -> String {
    let lines = input
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut games: Vec<Game> = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }

        let input = Game::from_line(&line);
        games.push(input)
    }

    let mut sum_part2 = 0;
    for game in games {
        let total_qube = TotalQubes::create(&game);
        sum_part2 += total_qube.red * total_qube.green * total_qube.blue;
    }
    sum_part2.to_string()
}
