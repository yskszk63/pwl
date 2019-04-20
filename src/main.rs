use clap::{crate_name, crate_version, App, Arg};

use crate::powerline::Powerline;
use crate::segments::Segments;
use crate::shell::Shell;
use crate::theme::Theme;

mod color;
mod powerline;
mod segments;
mod shell;
mod theme;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .arg(Arg::with_name("RC"))
        .get_matches();

    let rc = matches
        .value_of("RC")
        .map(|rc| rc.parse::<i32>().unwrap_or(-1));
    let theme = Theme::default();
    let shell = Shell::Bash;

    let output = std::io::stdout();
    let mut output = std::io::BufWriter::new(output.lock());
    let mut pwl = Powerline::new(rc, theme, shell, &mut output);
    let segments = [
        Segments::Virtualenv,
        Segments::Username,
        Segments::Hostname,
        Segments::Ssh,
        Segments::Cwd,
        Segments::Git,
        Segments::Jobs,
        Segments::Root,
    ];
    pwl.draw(&segments).unwrap();
}
