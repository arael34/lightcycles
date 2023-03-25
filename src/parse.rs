pub enum TrailKind {
    Default,
    Light,
    Outline,
    Curved,
    Dots,
}

impl TrailKind {
    pub fn into_chars(self) -> [[char; 4]; 4] {
        match self {
            Self::Default => [
                ['━', '┗', '━', '┏'],
                ['┓', '┃', '┏', '┃'],
                ['━', '┛', '━', '┓'],
                ['┛', '┃', '┗', '┃'],
            ],
            Self::Light => [
                ['─', '└', '─', '┌'],
                ['┐', '│', '┌', '│'],
                ['─', '┘', '─', '┐'],
                ['┘', '│', '└', '│'],
            ],
            Self::Outline => [
                ['═', '╚', '═', '╔'],
                ['╗', '║', '╔', '║'],
                ['═', '╝', '═', '╗'],
                ['╝', '║', '╚', '║'],
            ],
            _ => panic!("not implemented yet!"),
        }
    }
}

impl std::str::FromStr for TrailKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(Self::Default),
            "light" => Ok(Self::Light),
            "outline" => Ok(Self::Outline),
            "curved" => Ok(Self::Curved),
            "dots" => Ok(Self::Dots),
            _ => todo!(),
        }
    }
}

pub struct Config {

    pub number: Option<u8>,

    pub trailkind: Option<TrailKind>,

    pub bold: Option<bool>,

    pub delay: Option<u16>,

    pub turnchance: Option<u8>,
}

impl Config {
    pub fn number(&self) -> u8 {
        self.number.unwrap_or(5)
    }
    pub fn trailkind(&self) -> TrailKind {
        self.trailkind.unwrap_or(TrailKind::Default)
    }
    pub fn bold(&self) -> bool {
        self.bold.unwrap_or(false)
    }
    pub fn delay(&self) -> u16 {
        self.delay.unwrap_or(30)
    }
    pub fn turnchance(&self) -> u8 {
        self.turnchance.unwrap_or(10)
    }
}

pub fn parse_args() -> Result<Config, pico_args::Error> {

}
