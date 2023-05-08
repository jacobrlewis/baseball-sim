use rand::{
    distributions::{Distribution, Standard},
    Rng
};

struct Game {
    inning: i32,

    home_team: Team,
    away_team: Team,

    outs: i32,
    top: bool,

    base_runners: BaseRunners
}

impl Game {

    fn new(away_team_name: &str, home_team_name: &str) -> Game {
        Game {
            inning: 1,
            away_team: Team::new(away_team_name),
            home_team: Team::new(home_team_name),
            outs: 0,
            top: true,
            base_runners: BaseRunners::Empty
        }
    }

    fn print_status(&self) {
        println!("====================");
        println!("Away | {} : {}", self.away_team.name, self.away_team.runs);
        println!("Home | {} : {}", self.home_team.name, self.home_team.runs);
        println!("Start {} of the {}.", 
        match self.top {
            true => "Top",
            false => "Bottom"
        },self.inning);
        println!("====================");
    }

    fn do_half_inning(&mut self) {
        self.outs = 0;
        self.base_runners = BaseRunners::Empty;
        self.print_status();
        while self.outs < 3 {
            dbg!(self.base_runners);
            self.do_at_bat();
        }
    }

    fn do_inning(&mut self) {
        
        self.top = true;
        self.do_half_inning();

        self.top = false;
        self.do_half_inning();
        
        self.inning += 1;
    }

    fn score(&mut self, runs: i32) {

        if self.top {
            self.away_team.runs += runs;
        }
        else {
            self.home_team.runs += runs;
        }
    }

    fn do_at_bat(&mut self) {
        // get an at bat result
        let mut result:AtBat = rand::random();
        result = dbg!(result);
        match result { 
            AtBat::Strikeout => self.outs += 1,
            AtBat::Walk => {
                self.base_runners = match self.base_runners {
                    BaseRunners::Empty => BaseRunners::First,
                    BaseRunners::First => BaseRunners::FirstSecond,
                    BaseRunners::Second => BaseRunners::FirstSecond,
                    BaseRunners::Third => BaseRunners::FirstThird,
                    BaseRunners::FirstSecond => BaseRunners::Loaded,
                    BaseRunners::FirstThird => BaseRunners::Loaded,
                    BaseRunners::SecondThird => BaseRunners::Loaded,
                    BaseRunners::Loaded => {
                        self.score(1);
                        BaseRunners::Loaded
                    }
                };
            },
            AtBat::Contact => {
                // if contact, decide kind of contact
                let mut contact_result: ContactType = rand::random();
                contact_result = dbg!(contact_result);
                self.do_contact(contact_result);
            }
        } 
    }

    fn do_contact(&mut self, contact_type: ContactType) {
        // gross :(
        match contact_type {
            ContactType::GroundOut => self.outs += 1,
            ContactType::InfieldFly => self.outs += 1,
            ContactType::OutfieldFly => {
                self.outs += 1;
                if self.outs >= 3 {
                    return
                }
                self.base_runners = match self.base_runners {
                    BaseRunners::Third => {
                        self.score(1);
                        BaseRunners::Empty
                    },
                    BaseRunners::FirstThird => {
                        self.score(1);
                        BaseRunners::First
                    }
                    BaseRunners::SecondThird => {
                        self.score(1);
                        BaseRunners::Third
                    }
                    BaseRunners::Loaded => {
                        self.score(1);
                        BaseRunners::SecondThird
                    }
                    _ => self.base_runners
                };
            },
            ContactType::Single => {
                self.base_runners = match self.base_runners {
                    BaseRunners::Empty => BaseRunners::First,
                    BaseRunners::First => BaseRunners::FirstSecond,
                    BaseRunners::Second => BaseRunners::FirstSecond,
                    BaseRunners::Third => BaseRunners::FirstThird,
                    BaseRunners::FirstSecond => BaseRunners::Loaded,
                    BaseRunners::SecondThird => BaseRunners::Loaded,
                    BaseRunners::FirstThird => BaseRunners::Loaded,
                    BaseRunners::Loaded => {
                        self.score(1);
                        BaseRunners::Loaded
                    },
                };
            },
            ContactType::Double => {
                // assuming for now runners on 1st do not score
                self.base_runners = match self.base_runners {
                    BaseRunners::Empty => BaseRunners::Second,
                    BaseRunners::First => BaseRunners::SecondThird,
                    BaseRunners::Second => {
                        self.score(1);
                        BaseRunners::Second
                    },
                    BaseRunners::Third => {
                        self.score(1);
                        BaseRunners::Second
                    },
                    BaseRunners::FirstSecond => {
                        self.score(1);
                        BaseRunners::SecondThird
                    },
                    BaseRunners::SecondThird => {
                        self.score(2);
                        BaseRunners::Second
                    },
                    BaseRunners::FirstThird => {
                        self.score(1);
                        BaseRunners::SecondThird
                    },
                    BaseRunners::Loaded => {
                        self.score(2);
                        BaseRunners::SecondThird
                    },
                }
            },
            ContactType::Triple => {
                self.base_runners = BaseRunners::Third;
                self.score(match self.base_runners {
                    BaseRunners::Empty => 0,
                    BaseRunners::First | BaseRunners::Second | BaseRunners::Third => 1,
                    BaseRunners::FirstSecond | BaseRunners::SecondThird | BaseRunners::FirstThird => 2,
                    BaseRunners::Loaded => 3,
                }
                )
            },
            ContactType::Homerun => {
                self.base_runners = BaseRunners::Empty;
                self.score(match self.base_runners {
                    BaseRunners::Empty => 1,
                    BaseRunners::First | BaseRunners::Second | BaseRunners::Third => 2,
                    BaseRunners::FirstSecond | BaseRunners::SecondThird | BaseRunners::FirstThird => 3,
                    BaseRunners::Loaded => 4,
                }
                )
            },
        }
    }
}

struct Team {
    runs: i32,
    name: String
}

impl Team {
    fn new(name: &str) -> Team {
        Team {runs: 0, name: String::from(name)}
    }
}

#[derive(Debug)]
enum AtBat {
    Strikeout,
    Walk,
    Contact
}

impl Distribution<AtBat> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AtBat {
        match rng.gen_range(0..=2) {
            0 => AtBat::Strikeout,
            1 => AtBat::Walk,
            _ => AtBat::Contact
        }
    }
}

#[derive(Debug)]
enum ContactType {
    GroundOut,
    InfieldFly,
    OutfieldFly,
    Single,
    Double,
    Triple,
    Homerun,
}

impl Distribution<ContactType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ContactType {
        match rng.gen_range(0..=6) {
            0 => ContactType::GroundOut,
            1 => ContactType::InfieldFly,
            2 => ContactType::OutfieldFly,
            3 => ContactType::Single,
            4 => ContactType::Double,
            5 => ContactType::Triple,
            _ => ContactType::Homerun,
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
enum BaseRunners {
    Empty,
    First,
    Second,
    Third,
    FirstSecond,
    SecondThird,
    FirstThird,
    Loaded
}

fn main() {

    let mut game = Game::new("Red Sox", "Brewers");

    while game.inning <= 9 {
        game.do_inning();
    }

    println!("GAME OVER");
    game.print_status();
}