mod columnar;
mod format;
mod input_codes;
mod input_context;

use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;

use anyhow::{bail, Context};
use structopt::StructOpt;

use crate::format::{ControlMapFile, ControlMapLine, PrettyPrintBinding};
use crate::input_codes::{Gamepad, Keyboard, Mouse};
use crate::input_context::InputContext;

static DEFAULT_CONTROLMAP: &str = include_str!("../maps/controlmap-default.txt");

#[derive(StructOpt)]
pub struct Options {
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(StructOpt)]
pub enum Subcommand {
    /// Creates a new controlmap file and writes it to the given path.
    New {
        /// Where to write the new controlmap.
        output: PathBuf,
    },

    /// Reformat the given controlmap file to align columns, fix whitespace,
    /// remove unused duplicate bindings, etc.
    Reformat {
        /// The file to reformat.
        input: PathBuf,

        /// Output path. Will overwrite the input path if not given.
        output: Option<PathBuf>,
    },

    /// Print a controlmap file with human-readable descriptions of what all of
    /// the bindings are.
    Explain {
        /// The file to explain.
        input: PathBuf,
    },

    /// Merge multiple controlmap files together, letting later files overwrite
    /// matching entries in earlier files.
    Merge {
        /// The files to merge together, in order.
        inputs: Vec<PathBuf>,

        /// Where to output the merged result.
        #[structopt(long, short)]
        output: PathBuf,
    },
}

fn run() -> anyhow::Result<()> {
    let options = Options::from_args();

    match options.subcommand {
        Subcommand::New { output } => {
            fs_err::write(output, DEFAULT_CONTROLMAP)?;
        }

        Subcommand::Reformat { input, output } => {
            let contents = fs_err::read_to_string(&input)?;
            let mut map: ControlMapFile = contents.parse()?;
            map.remove_duplicates();
            let formatted = map.to_string();

            let output_path = output.as_ref().unwrap_or(&input);
            fs_err::write(output_path, formatted)?;
        }

        Subcommand::Explain { input } => {
            let contents = fs_err::read_to_string(&input)?;
            let map: ControlMapFile = contents.parse()?;

            for (section_id, section) in map.sections.iter().enumerate() {
                if let Some(context) = InputContext::from_u32(section_id as u32) {
                    println!("==== {:?} ====", context);
                } else {
                    println!("==== <unknown section> ====");
                }

                for line in &section.body {
                    if let ControlMapLine::Entry(entry) = line {
                        println!("{} is bound to:", entry.event);
                        println!(
                            "    Keyboard: {}",
                            PrettyPrintBinding::<Keyboard>::new(&entry.keyboard)
                        );
                        println!(
                            "    Mouse:    {}",
                            PrettyPrintBinding::<Mouse>::new(&entry.mouse)
                        );
                        println!(
                            "    Gamepad:  {}",
                            PrettyPrintBinding::<Gamepad>::new(&entry.gamepad)
                        );
                        println!();
                    }
                }
            }
        }

        Subcommand::Merge { inputs, output } => {
            if inputs.is_empty() {
                bail!("No input files specified");
            }

            let mut maps = VecDeque::new();
            for input in inputs {
                let contents = fs_err::read_to_string(&input)?;
                let map: ControlMapFile = contents
                    .parse()
                    .with_context(|| format!("Could not parse {}", input.display()))?;

                maps.push_back(map);
            }

            let mut base_map = maps.pop_front().unwrap();
            let mut patches = Vec::new();

            while let Some(map) = maps.pop_front() {
                for _ in map.sections.len()..patches.len() {
                    patches.push(HashMap::new());
                }

                for (section_id, section) in map.sections.into_iter().enumerate() {
                    let section_patches = &mut patches[section_id];

                    for line in section.body {
                        if let ControlMapLine::Entry(entry) = line {
                            section_patches.insert(entry.event.clone(), entry);
                        }
                    }
                }
            }

            for (section_id, section) in base_map.sections.iter_mut().enumerate() {
                let section_patches = &mut patches[section_id];

                for line in &mut section.body {
                    if let ControlMapLine::Entry(entry) = line {
                        if let Some(patch) = section_patches.remove(&entry.event) {
                            *entry = patch;
                        }
                    }
                }
            }

            base_map.remove_duplicates();
            let encoded = base_map.to_string();
            fs_err::write(output, encoded)?;
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
