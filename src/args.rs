use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[arg(short, long)]
    pub day: u32,

    #[arg(short, long)]
    pub challenge: u8,
}
