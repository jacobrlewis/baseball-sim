use rand::{
    distributions::{Distribution, Standard},
    Rng
};

struct Game {
    inning: i32,

    home_team: Team,
    away_team: Team,

    outs: i32,
    top: bool
}

impl Game {

    fn new(away_team_name: &str, home_team_name: &str) -> Game {
        Game {
            inning: 1,
            away_team: Team::new(away_team_name),
            home_team: Team::new(home_team_name),
            outs: 0,
            top: true
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
        self.print_status();
        while self.outs < 3 {
            self.do_at_bat();
        }
    }

    fn do_inning(&mut self) {
        
        self.top = true;
        self.outs = 0;
        self.do_half_inning();

        self.top = false;
        self.outs = 0;
        self.do_half_inning();
        
        self.inning += 1;
    }

    fn do_at_bat(&mut self) {
        // get an at bat result
        let mut result:AtBat = rand::random();
        result = dbg!(result);
        match result { 
            AtBat::Strikeout => self.outs += 1,
            AtBat::Walk => (),
            AtBat::Contact => {
                // if contact, decide kind of contact
                let mut contact_result: ContactType = rand::random();
                contact_result = dbg!(contact_result);
                match contact_result {
                    ContactType::GroundOut => self.outs += 1,
                    ContactType::InfieldFly => self.outs += 1,
                    ContactType::OutfieldFly => self.outs += 1,
                    ContactType::Single => (),
                    ContactType::Double => (),
                    ContactType::Triple => (),
                    ContactType::Homerun => ()
                }
            }
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

#[allow(dead_code)]
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

fn main() {

    let mut game = Game::new("Red Sox", "Brewers");

    while game.inning <= 9 {
        game.do_inning();
    }
}