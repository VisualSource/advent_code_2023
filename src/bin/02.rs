advent::solution!(2);

#[derive(Debug)]
struct Color {
    red: usize,
    green: usize,
    blue: usize,
}

impl Color {
    pub fn get_power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let colors = value.split(",").collect::<Vec<&str>>();

        for color in colors {
            let item = color.trim().split(" ").collect::<Vec<&str>>();

            let color_id = item.get(1).unwrap().to_owned();
            let count = item.get(0).unwrap().parse::<usize>().unwrap();

            match color_id {
                "blue" => {
                    blue += count;
                }
                "green" => {
                    green += count;
                }
                "red" => {
                    red += count;
                }
                _ => {}
            }
        }

        Color { red, green, blue }
    }
}

#[derive(Debug)]
struct Game {
    pub id: usize,
    games: Vec<Color>,
}

impl Game {
    pub fn is_valid(&self, max_red: usize, max_green: usize, max_blue: usize) -> bool {
        self.games
            .iter()
            .all(|item| item.red <= max_red && item.green <= max_green && item.blue <= max_blue)
    }

    pub fn max_values(&self) -> Color {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        self.games.iter().for_each(|item| {
            if item.blue > blue {
                blue = item.blue;
            }

            if item.red > red {
                red = item.red;
            }

            if item.green > green {
                green = item.green;
            }
        });

        Color { red, green, blue }
    }
}

fn parse_content(data: &'static str) -> Vec<Game> {
    data.lines()
        .map(|line| {
            let game_content = line.split(":").collect::<Vec<&str>>();

            let game_id = game_content
                .get(0)
                .unwrap()
                .replace("Game ", "")
                .parse::<usize>()
                .unwrap();

            let context = game_content.get(1).unwrap();

            let games = context.split(";").map(|x| x.into()).collect::<Vec<Color>>();

            Game {
                id: game_id,
                games: games,
            }
        })
        .collect::<Vec<Game>>()
}

#[must_use]
fn part_one(data_path: &'static str) -> Option<usize> {
    let games = parse_content(data_path);

    let value = games
        .iter()
        .filter(|x| x.is_valid(12, 13, 14))
        .fold(0, |acc, x| acc + x.id);

    Some(value)
}

#[must_use]
fn part_two(data_path: &'static str) -> Option<usize> {
    let data = parse_content(data_path);

    Some(
        data.iter()
            .fold(0, |acc, e| acc + e.max_values().get_power()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_color_parse() {
        let output = Color::from("8 green, 6 blue, 20 red");

        println!("{:#?}", output);
    }
}
