use std::collections::BTreeMap;

#[derive(Debug)]
struct Team {
    name: String,
    win: u32,
    loss: u32,
    draw: u32,
}

impl Team {
    fn new(s: &str) -> Self {
        Team {
            name: String::from(s),
            win: 0,
            loss: 0,
            draw: 0,
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn point(&self) -> u32 {
        self.win * 3 + self.draw
    }

    fn update(&mut self, val: i32) {
        if val > 0 {
            self.win += 1;
        } else if val == 0 {
            self.draw += 1;
        } else {
            self.loss += 1;
        }
    }

    fn count(&self) -> u32 {
        self.win + self.loss + self.draw
    }
}

impl ToString for Team {
    fn to_string(&self) -> String {
        //Devastating Donkeys            |  3 |  2 |  1 |  0 |  7
        format!("{name: <31}|  {count} |  {win} |  {draw} |  {loss} |  {point}",
                name = self.name(), count = self.count(), win = self.win, loss = self.loss, draw = self.draw, point = self.point())
    }
}

pub fn tally(input: &str) -> String {
    let mut teams: Vec<Team> = Vec::new();
    let mut map: BTreeMap<&str, usize> = BTreeMap::new();
    for game in input.split('\n') {
        let game_part = game.split(';').collect::<Vec<&str>>();
        if game_part.len() != 3 {
            continue
        }

        let res = match game_part[2] {
            "win" => Some(1),
            "loss" => Some(-1),
            "draw" => Some(0),
            _ => None,
        };
        if res.is_none() {
            continue;
        }

        let lhs = map.entry(game_part[0]).or_insert_with(|| {
            teams.push(Team::new(game_part[0]));
            teams.len() - 1
        }).clone();
        let rhs = map.entry(game_part[1]).or_insert_with(|| {
            teams.push(Team::new(game_part[1]));
            teams.len() - 1
        }).clone();


        teams[lhs].update(res.unwrap());
        teams[rhs].update(0 - res.unwrap());

    }

    let teams = teams.as_mut_slice();
    teams.sort_by_key(|t| t.name());
    teams.sort_by_key(|t| -(t.point() as i32));

    String::from("Team                           | MP |  W |  D |  L |  P\n") +
        teams.iter().map(|t| t.to_string()).collect::<Vec<_>>().join("\n").as_str()
}
