use clap::{crate_name, crate_version, App, Arg, arg_enum, values_t_or_exit};

use crate::powerline::Powerline;
use crate::segments::Segments;
use crate::shell::Shell;
use crate::theme::Theme;

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
        .arg(Arg::with_name("segments")
             .short("s")
             .possible_values(&CliSegments::variants())
             .value_name("SEGMENTS")
             .value_delimiter(",")
             .case_insensitive(true)
             .default_value(&x))
        .get_matches();

    let rc = matches
        .value_of("RC")
        .map(|rc| rc.parse::<i32>().unwrap_or(-1));
    let segments = values_t_or_exit!(matches, "segments", CliSegments);
    let segments = segments.into_iter().map(move |s| s.into_segments()).collect::<Vec<_>>();
    let theme = Theme::default();
    let shell = Shell::Bash;

    let output = std::io::stdout();
    let mut output = std::io::BufWriter::new(output.lock());
    let mut pwl = Powerline::new(rc, theme, shell, &mut output);

    pwl.draw(&segments).unwrap();
}
