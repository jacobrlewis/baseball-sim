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
    fn print_status(&self) {
        println!("====================");
        println!("Away| {} : {}", self.away_team.name, self.away_team.runs);
        println!("Home| {} : {}", self.home_team.name, self.home_team.runs);
        println!("{} of the {}.", 
        match self.top {
            true => "Top",
            false => "Bottom"
        },self.inning);
        println!("====================");
    }

    fn do_half_inning(&mut self) {

        while self.outs < 3 {
            self.do_at_bat();
        }

        // go to next
        self.outs = 0;
        match self.top {
            true => self.top = false,
            false => {
                self.top = true;
                self.inning += 1;
            }
        }
    }

    fn do_inning(&mut self) {
        self.print_status();
        self.do_half_inning();
        self.print_status();
        self.do_half_inning();
    }

    fn do_at_bat(&mut self) {
        let result:AtBat = rand::random();
        match result { 
            AtBat::Strikeout => self.outs += 1,
            AtBat::Walk => (),
            AtBat::Contact { contact_result } => {
                _ = contact_result
            }
        } 
    }
}

struct Team {
    runs: i32,
    name: String
}

enum AtBat {
    Strikeout,
    Walk,
    Contact {contact_result: ContactType}
}

impl Distribution<AtBat> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AtBat {
        match rng.gen_range(0..2) {
            0 => AtBat::Strikeout,
            1 => AtBat::Walk,
            _ => AtBat::Contact { contact_result: ContactType::GroundOut }
        }
    }
}

enum ContactType {
    GroundOut,
    InfieldFly,
    OutfieldFly,
    Single,
    Double,
    Triple,
    Homerun,
}

fn main() {

    let red_sox = Team {
        runs: 0, 
        name: String::from("Red Sox")
    };
    let brewers = Team {
        runs: 0,
        name: String::from("Brewers")
    };

    let mut game = Game {
        inning: 1,
        home_team: brewers,
        away_team: red_sox,
        outs: 0,
        top: true
    };


    while game.inning <= 9 {
        game.do_inning();
    }
}
