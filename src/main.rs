#![allow(unused)]
// TODO: REMOVE THIS ^^^


// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use clap::{Subcommand, Parser};

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let args = Args::parse();
    args.command.execute()
}

#[derive(Debug)]
struct CropVal { x: u32, y: u32, width: u32, height: u32 }

#[derive(Debug, Subcommand)]
enum Command {
    /// blur a file by a given percentage
    Blur      {
        infile: String,
        outfile: String,
        value: f32,
    },
    /* TODO All these should be real structs
    Don't use tuple structs for this case, they are WAY less readable
    Brighten  { String, String, i32,     ),
    Crop      ( String, String, CropVal, ),
    Rotate    ( String, String, Degrees  ),
    Generate  ( String, i32              ),
    Invert    ( String, String           ),
    Grayscale ( String, String,          ),
    Fractal   ( String, u32, u32         ), */
}

impl Command {
    fn execute(self) -> Result<()> {
        match self {
            Self::Blur{infile: i_file, outfile: o_file, value: val} => {
                let img = image::open(&i_file).context(format!("Failed to open {}", i_file))?;
                let img = img.blur(val);
                img.save(&o_file).context(format!("Failed writing {}.", o_file))
            },
/* TODO...
            Self::Brighten (i_file, o_file, val) => {
                let img = image::open(i_file).expect(format!("Failed to open {}", i_file).as_str());
                let img = img.brighten(*val);
                img.save(o_file).expect(format!("Failed writing {}.", o_file).as_str());
            },

            Self::Crop (i_file, o_file, val) => {
                let mut img = image::open(i_file).expect(format!("Failed to open {}", i_file).as_str());
                let img = img.crop(val.x, val.y, val.width, val.height);
                img.save(o_file).expect(format!("Failed writing {}.", o_file).as_str());
            },

            Self::Rotate (i_file, o_file, deg) => {
                let img = image::open(i_file).expect(format!("Failed to open {}", i_file).as_str());
                // This is doing two things but that's just the way the problem was setup
                // as part of the udemy class. Ideally I suppose it would have two
                // commands: --huerotate and --rotate. This is just an exercise to learn
                // rust so I idn't do that.
                let img = img.huerotate(deg.val());
                let img = match deg {
                    Degrees::_90  => img.rotate90(),
                    Degrees::_180 => img.rotate180(),
                    Degrees::_270 => img.rotate270(),
                };
                img.save(o_file).expect(format!("Failed writing {}.", o_file).as_str());
            },

            Self::Invert (i_file, o_file) => {
                let mut img = image::open(i_file).expect(format!("Failed to open {}", i_file).as_str());
                img.invert();
                img.save(o_file).expect(format!("Failed writing {}.", o_file).as_str());
            },

            Self::Grayscale (i_file, o_file) => {
                let img = image::open(i_file).expect(format!("Failed to open {}", i_file).as_str());
                let img = img.grayscale();
                img.save(o_file).expect(format!("Failed writing {}.", o_file).as_str());
            },
            Self::Fractal (f, w, h) => fractal(f, *w, *h),
            Self::Generate (f, v)   => generate(f, *v),
        } // match
    } // fn execute
    */
        } 
    }
}

enum Degrees { _90, _180, _270 }

impl Degrees {
    fn val(&self) -> i32 {
        match self {
            Degrees::_90  => 90,
            Degrees::_180 => 180,
            Degrees::_270 => 270
        }
    }
}

fn generate(outfile: &String, color: i32) {

    println!("\nGenerate: file={}, color={} is not yet implemented", outfile, color);

    // TODO impl
    // Create an ImageBuffer -- see fractal() for an example

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example

    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!

    // See blur() for an example of how to save the image
}

// This code was adapted from https://github.com/PistonDevelopers/image
/* fix this to return Result<()> 
fn fractal(outfile: &String, width: u32, height: u32) {

    println!("fractal: f:{outfile}, w:{width}, h:{height}");
    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    match imgbuf.save(outfile)
    {
        Err (msg) => println!("\n{}: Error: writing output {}:{}", *PROGRAM_NAME, outfile, msg),
        Ok  (_)   => {},
    }
}
*/

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
