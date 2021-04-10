mod cmyk_model;
mod color;
mod color_names;
mod hsv_model;
mod namedcolor;
mod reports;

use crate::color::Color;
use crate::color_names::COLOR_NAMES;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "colore", about = "CLI color-space browser")]
enum Commands {
    /// Manages color objects
    Color {
        #[structopt(subcommand)]
        cmd: ColorCommands,
    },

    /// Manages color lists
    List {
        #[structopt(subcommand)]
        cmd: ListCommands,
    },
}

#[derive(StructOpt)]
enum ColorCommands {
    /// Choose random colors
    Random {
        #[structopt(short, long, default_value = "1")]
        count: u16,
    },

    /// Prints detailed color information
    Info {
        #[structopt(long, number_of_values = 3)]
        rgb: Vec<u8>,
    },
}

#[derive(StructOpt)]
enum ListCommands {
    /// Dumps color names
    Dump {},
}

#[derive(StructOpt)]
enum ColorSelector {
    
    RGB {
        #[structopt(long, number_of_values = 3)] 
        rgb: Vec<u8>
    },
    NAME(String)
}

fn main() {
    match Commands::from_args() {
        Commands::Color { cmd } => match cmd {
            ColorCommands::Random { count } => {
                if count == 1 {
                    let c = Color::random();
                    println!("{}", reports::full_report(c));
                } else {
                    for i in 1..=count {
                        let c = Color::random();
                        println!("{:3}. {}", i, reports::short_report(c));
                    }
                }
            }
            ColorCommands::Info { rgb } => {
                let c = Color::new(rgb[0], rgb[1], rgb[2]);
                println!("{}", reports::full_report(c));
            }
        },

        Commands::List { cmd } => match cmd {
            ListCommands::Dump {} => {
                for c in 0..COLOR_NAMES.len() {
                    println!("{:6}. {}", c + 1, reports::summary_report(&COLOR_NAMES[c]));
                }
            }
        },
    }
}
