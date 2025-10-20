use clap::Parser;

// No-nonsense grep-like tool.
#[derive(Parser)]
#[command(
    name = "ken",
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
    subcommand_required = false,
    arg_required_else_help = false,
)]
pub struct Cli {
    pub pattern: Option<String>,

    #[arg(short = 'l', long, help = "Show line number")]
    pub show_line_numbers: bool,

    #[arg(
        short = 'c',
        long,
        help = "Set the highlight color (red, green, blue, yellow, dimmed, none). Default is red.",
        default_value = "red"
    )]
    pub hightlight_color: String,

    #[arg(short, long, help = "Show performance metrics")]
    pub metrics: bool,
}
