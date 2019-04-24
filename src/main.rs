use clap::{arg_enum, crate_name, crate_version, value_t_or_exit, values_t_or_exit, App, Arg};

use crate::powerline::Powerline;
use crate::segments::Segments;
use crate::shell::Shell;

mod color;
mod powerline;
mod segments;
mod shell;
mod theme;

arg_enum! {
    pub enum CliSegments {
        Virtualenv,
        Username,
        Hostname,
        Ssh,
        Cwd,
        Git,
        Jobs,
        Root,
    }
}

arg_enum! {
    pub enum CliTheme {
        Default,
        SolarizedLight,
    }
}

impl CliSegments {
    pub fn into_segments(self) -> Segments {
        match self {
            CliSegments::Root => Segments::Root,
            CliSegments::Cwd => Segments::Cwd,
            CliSegments::Jobs => Segments::Jobs,
            CliSegments::Virtualenv => Segments::Virtualenv,
            CliSegments::Username => Segments::Username,
            CliSegments::Hostname => Segments::Hostname,
            CliSegments::Ssh => Segments::Ssh,
            CliSegments::Git => Segments::Git,
        }
    }
}

fn main() {
    let x = CliSegments::variants().join(",");

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .arg(Arg::with_name("RC"))
        .arg(
            Arg::with_name("segments")
                .short("s")
                .long("segments")
                .possible_values(&CliSegments::variants())
                .value_name("SEGMENTS")
                .value_delimiter(",")
                .case_insensitive(true)
                .default_value(&x),
        )
        .arg(
            Arg::with_name("theme")
                .short("t")
                .long("theme")
                .possible_values(&CliTheme::variants())
                .value_name("THEME")
                .case_insensitive(true)
                .default_value("default"),
        )
        .arg(Arg::with_name("cwd-short").long("cwd-short"))
        .get_matches();

    let rc = matches
        .value_of("RC")
        .map(|rc| rc.parse::<i32>().unwrap_or(-1));
    let segments = values_t_or_exit!(matches, "segments", CliSegments);
    let segments = segments
        .into_iter()
        .map(CliSegments::into_segments)
        .collect::<Vec<_>>();
    let theme = match value_t_or_exit!(matches, "theme", CliTheme) {
        CliTheme::Default => Default::default(),
        CliTheme::SolarizedLight => theme::solarized_light(),
    };
    let cwd_short = matches.is_present("cwd-short");
    let shell = Shell::Bash;

    let output = std::io::stdout();
    let mut output = std::io::BufWriter::new(output.lock());
    let mut pwl = Powerline::new(rc, cwd_short, theme, shell, &mut output);

    pwl.draw(&segments).unwrap();
}
