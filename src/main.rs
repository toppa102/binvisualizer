use clap::Clap;
use image::png::*;
use image::ColorType;
use std::convert::TryFrom;

#[derive(Clap)]
#[clap(version = "0.1", author = "toppa102")]
struct Opts {
    input: String,

    #[clap(short, long)]
    rgb: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut bin_file = std::fs::read(&opts.input).unwrap();
    let bin_size = bin_file.len();

    let mut temp_input = opts.input.clone();
    temp_input.push_str(".png");

    let img_file = std::fs::File::create(temp_input).unwrap();

    let mut img_size: usize = 0;
    let mut color_type: ColorType = ColorType::L8;

    match opts.rgb {
        true => {
            let img_size_ref = &mut img_size;
            *img_size_ref = ((bin_size as f64) / 3.0).sqrt().ceil() as usize;
            color_type = ColorType::Rgb8;

            for _x in 0..(*img_size_ref * *img_size_ref * 3usize)
                .checked_sub(bin_size)
                .unwrap()
            {
                bin_file.push(0);
            }
        }
        false => {
            let img_size_ref = &mut img_size;
            *img_size_ref = (bin_size as f64).sqrt().ceil() as usize;

            for _x in 0..(*img_size_ref * *img_size_ref)
                .checked_sub(bin_size)
                .unwrap()
            {
                bin_file.push(0);
            }
        }
    }

    let encoder = PngEncoder::new(img_file);

    encoder
        .encode(
            &bin_file,
            u32::try_from(img_size).unwrap(),
            u32::try_from(img_size).unwrap(),
            color_type,
        )
        .unwrap();
}
