use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "Image Channel Mixer")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hsl(Hsl), // to hsv
    Rgb(Rgb), // to rgb
}

#[derive(Args)]
struct Hsl {
    /// input image file for the hue of the output image
    #[arg(long = "hf")]
    hue_file: String,

    /// channel of input file to use for output hue
    #[arg(long = "hc")]
    hue_channel: Channel,

    /// input image file for saturation of the output image
    #[arg(long = "sf")]
    saturation_file: String,

    /// channel of input file to use for output saturation
    #[arg(long = "sc")]
    saturation_channel: Channel,

    /// input image file for lightness of the output image
    #[arg(long = "lf")]
    lightness_file: String,

    /// channel of input file to use for output lightness
    #[arg(long = "lc")]
    lightness_channel: Channel,

    /// height of output image
    #[arg(short = 't', long, default_value_t = 1080)]
    height: u16,

    /// width of output image
    #[arg(short, long, default_value_t = 1920)]
    width: u16,

    /// path of output image
    #[arg(short, long)]
    output: String,
}

/// image channel
#[derive(ValueEnum, Clone)]
enum Channel {
    /// hue channel from input image
    H,
    /// saturation channel from input image
    S,
    /// value channel from input image
    V,
    /// lightness channel from input image
    L,
    /// red channel from input image
    R,
    /// green channel from input image
    G,
    /// blue channel from input image
    B,
    /// alpha channel from input image
    A,
}

#[derive(Args)]
struct Rgb {
    #[arg(short, long)]
    red: String,
    #[arg(short, long)]
    green: String,
    #[arg(short, long)]
    blue: String,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Hsl(args) => {
            println!("hsv yo");
        }
        Commands::Rgb(args) => {
            println!("rgb yo");
        }
    }
    println!("image channel mixing complete");
}
