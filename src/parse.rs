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
            Self::Curved => [
                ['─', '╰', '─', '╭'],
                ['╮', '│', '╭', '│'],
                ['─', '╯', '─', '╮'],
                ['╯', '│', '╰', '│'],
            ],
            Self::Dots => [
                ['•', '•', '•', '•'],
                ['•', '•', '•', '•'],
                ['•', '•', '•', '•'],
                ['•', '•', '•', '•'],
            ],
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
            // if incorrect value is passed, just use default trail
            _ => Ok(Self::Default)
        }
    }
}

pub struct Config {
    pub number: Option<u8>,
    pub trailkind: Option<TrailKind>,
    pub bold: Option<bool>,
    pub delay: Option<u64>,
    pub turnchance: Option<u8>,
}

impl Config {
    pub fn parse() -> Result<Config, pico_args::Error> {
        let mut args = pico_args::Arguments::from_env();

        let config = Config {
            // get optional values
            number: args.opt_value_from_str("--number")?,
            trailkind: args.opt_value_from_str("--trailkind")?,
            bold: args.opt_value_from_str("--bold")?,
            delay: args.opt_value_from_str("--delay")?,
            turnchance: args.opt_value_from_str("--turnchance")?,
        };

        Ok(config)
    }
}
