#[derive(clap::Parser, Debug)]
pub struct Args {
    #[clap(
        short,
        long,
        action,
        help = "List only local branches"
    )]
    pub local: bool,
}