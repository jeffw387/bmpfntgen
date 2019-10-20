use bmpfntgen::{CharSets, ImageFormat, MetaFormat};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "bmpfntgen")]
struct CLIOptions {
    #[structopt(short, long)]
    font_path: String,

    #[structopt(short = "n", long, default_value = "output")]
    output_name: String,

    #[structopt(short = "p", long, default_value = "./")]
    output_path: String,

    #[structopt(short, long, default_value = "PNG")]
    image_format: ImageFormat,

    #[structopt(short, long, default_value = "Default")]
    meta_format: MetaFormat,

    #[structopt(
        short,
        long,
        default_value = "TestSet",
        help = "Char set to render",
        long_help = r"Char set to render

Options:
    EnglishLowerCase
    EnglishUpperCase
    Digits
    CommonSymbols
    TestSet"
    )]
    char_set: CharSets,

    #[structopt(short, long, default_value = "16")]
    height: usize,
}

fn main() -> Result<(), bmpfntgen::Error> {
    let cli_options = CLIOptions::from_args();

    let font = bmpfntgen::load_ttf(&cli_options.font_path)?;

    let result = bmpfntgen::layout_and_render(
        &font,
        cli_options.char_set,
        cli_options.height as f32,
    )?;
    bmpfntgen::save(
        &cli_options.output_name,
        &cli_options.output_path,
        result,
        cli_options.meta_format,
        cli_options.image_format,
    )
}
