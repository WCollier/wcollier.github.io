use clap::Parser;

#[derive(Copy, Clone, Debug, Parser)]
pub(crate) struct Args {
    #[arg(short, long)]
    pub(crate) dev: bool,
}

pub(crate) fn parse() -> Args {
    Args::parse()
}
