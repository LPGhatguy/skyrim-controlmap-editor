use std::collections::HashSet;
use std::fmt::{self, Display};
use std::marker::PhantomData;
use std::mem::take;
use std::str::FromStr;

use anyhow::Context;

use crate::columnar::ColumnPrinter;
use crate::input_codes::InputCode;

#[derive(Debug)]
pub struct ControlMapFile {
    pub sections: Vec<ControlMapSection>,
}

impl ControlMapFile {
    pub fn remove_duplicates(&mut self) {
        for section in &mut self.sections {
            let mut visited = HashSet::new();

            section.body = take(&mut section.body)
                .into_iter()
                .rev()
                .filter(|line| match line {
                    ControlMapLine::Comment(_) => true,
                    ControlMapLine::Entry(entry) => {
                        if visited.contains(&entry.event) {
                            false
                        } else {
                            visited.insert(entry.event.clone());
                            true
                        }
                    }
                })
                .rev()
                .collect();
        }
    }
}

impl FromStr for ControlMapFile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = vec![ControlMapSection::new()];

        for (i, line) in s.lines().enumerate() {
            let line = line.trim();

            if line.is_empty() && !sections.last().unwrap().body.is_empty() {
                sections.push(ControlMapSection::new());
                continue;
            }

            if line.starts_with("//") {
                let comment = (&line[2..]).trim_start().to_owned();
                sections
                    .last_mut()
                    .unwrap()
                    .body
                    .push(ControlMapLine::Comment(comment));
                continue;
            }

            let parsed: ControlMapEntry = line
                .parse()
                .with_context(|| format!("Error on line {}", i + 1))?;
            sections
                .last_mut()
                .unwrap()
                .body
                .push(ControlMapLine::Entry(parsed));
        }

        Ok(Self { sections })
    }
}

impl Display for ControlMapFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, section) in self.sections.iter().enumerate() {
            writeln!(f, "{}", section)?;

            if i < self.sections.len() - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ControlMapSection {
    pub body: Vec<ControlMapLine>,
}

impl Display for ControlMapSection {
    fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = ColumnPrinter::new();

        for line in &self.body {
            match line {
                ControlMapLine::Comment(text) => {
                    printer.finish(&mut f)?;
                    printer = ColumnPrinter::new();
                    writeln!(f, "// {}", text)?;
                }
                ControlMapLine::Entry(entry) => {
                    printer.row();
                    printer.add(&entry.event);
                    printer.add(&entry.keyboard);
                    printer.add(&entry.mouse);
                    printer.add(&entry.gamepad);
                    printer.add(entry.keyboard_mappable as u8);
                    printer.add(entry.mouse_mappable as u8);
                    printer.add(entry.gamepad_mappable as u8);

                    if let Some(flag) = entry.event_flag {
                        printer.add(format_args!("{:#x}", flag));
                    }
                }
            }
        }

        printer.finish(&mut f)?;

        Ok(())
    }
}

impl ControlMapSection {
    fn new() -> Self {
        Self { body: Vec::new() }
    }
}

#[derive(Debug)]
pub enum ControlMapLine {
    Comment(String),
    Entry(ControlMapEntry),
}

#[derive(Debug)]
pub struct ControlMapEntry {
    pub event: String,
    pub keyboard: Binding,
    pub mouse: Binding,
    pub gamepad: Binding,
    pub keyboard_mappable: bool,
    pub mouse_mappable: bool,
    pub gamepad_mappable: bool,
    pub event_flag: Option<u32>,
}

impl FromStr for ControlMapEntry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split('\t').filter(|s| !s.is_empty());
        let event = pieces
            .next()
            .context("Missing event name (first value)")?
            .to_owned();
        let keyboard = pieces
            .next()
            .context("Missing keyboard binding (second value)")?
            .parse()
            .context("Invalid keyboard binding (second value)")?;
        let mouse = pieces
            .next()
            .context("Missing mouse binding (third value)")?
            .parse()
            .context("Invalid mouse binding (third value)")?;
        let gamepad = pieces
            .next()
            .context("Missing gamepad binding (fourth value)")?
            .parse()
            .context("Invalid gamepad binding (fourth value)")?;
        let keyboard_mappable = pieces
            .next()
            .context("Missing keyboard mappable flag (fifth value)")?
            .parse::<u8>()
            .context("Invalid keyboard mappable flag (fifth value)")?
            != 0;
        let mouse_mappable = pieces
            .next()
            .context("Missing mouse mappable flag (sixth value)")?
            .parse::<u8>()
            .context("Invalid mouse mappable flag (sixth value)")?
            != 0;
        let gamepad_mappable = pieces
            .next()
            .context("Missing gamepad mappable flag (seventh value)")?
            .parse::<u8>()
            .context("Invalid gamepad mappable flag (seventh value)")?
            != 0;
        let event_flag = match pieces.next() {
            None => None,
            Some(flag) => {
                Some(parse_hex(flag).context("Invalid event binary flag (eigth value, optional)")?)
            }
        };

        Ok(Self {
            event,
            keyboard,
            mouse,
            gamepad,
            keyboard_mappable,
            mouse_mappable,
            gamepad_mappable,
            event_flag,
        })
    }
}

#[derive(Debug)]
pub struct Binding {
    pub inputs: Vec<Input>,
}

impl FromStr for Binding {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inputs = Vec::new();
        let mut pieces = s.split(',');

        while let Some(piece) = pieces.next() {
            if piece == "0xff" {
                // This value is used to indicate no binding.
                continue;
            }

            if piece.starts_with("!") {
                // This prefix indicates that the rest of this value is an input
                // context ID and the next value should be used as an alias.

                let context = (&piece[1..])
                    .parse()
                    .context("Invalid binding: expected input context ID after !")?;

                let event = pieces
                    .next()
                    .context("Invalid binding: expected event name")?
                    .to_owned();

                inputs.push(Input::Alias { context, event });
                continue;
            }

            let codes = piece
                .split('+')
                .map(parse_hex)
                .collect::<Result<_, _>>()
                .context("Invalid binding")?;

            inputs.push(Input::Inputs(codes));
        }

        Ok(Self { inputs })
    }
}

impl Display for Binding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.inputs.is_empty() {
            write!(f, "0xff")?;
            return Ok(());
        }

        for (i, input) in self.inputs.iter().enumerate() {
            match input {
                Input::Alias { context, event } => {
                    write!(f, "!{},{}", context, event)?;
                }

                Input::Inputs(keys) => {
                    for (i, key) in keys.iter().enumerate() {
                        write!(f, "{:#04x}", key)?;

                        if i < keys.len() - 1 {
                            write!(f, "+")?;
                        }
                    }
                }
            }

            if i < self.inputs.len() - 1 {
                write!(f, ",")?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Input {
    Inputs(Vec<u32>),
    Alias { context: usize, event: String },
}

fn parse_hex(input: &str) -> anyhow::Result<u32> {
    let body = input
        .strip_prefix("0x")
        .with_context(|| format!("Invalid hex value {}, missing 0x prefix", input))?;
    let value = u32::from_str_radix(body, 16)?;
    Ok(value)
}

pub struct PrettyPrintBinding<'a, I> {
    binding: &'a Binding,
    _marker: PhantomData<*const I>,
}

impl<'a, I: InputCode> PrettyPrintBinding<'a, I> {
    pub fn new(binding: &'a Binding) -> Self {
        Self {
            binding,
            _marker: PhantomData,
        }
    }
}

impl<'a, I: InputCode> Display for PrettyPrintBinding<'a, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.binding.inputs.is_empty() {
            write!(f, "<nothing>")?;
        }

        for (i, input) in self.binding.inputs.iter().enumerate() {
            match input {
                Input::Alias { context, event } => write!(f, "[{},{}]", context, event)?,
                Input::Inputs(values) => {
                    for (i, value) in values.iter().enumerate() {
                        if let Some(decoded) = I::from_u32(*value) {
                            write!(f, "{:?}", decoded)?;
                        } else {
                            write!(f, "<unknown>")?;
                        }

                        if i < values.len() - 1 {
                            write!(f, "+")?;
                        }
                    }
                }
            }

            if i < self.binding.inputs.len() - 1 {
                write!(f, " OR ")?;
            }
        }

        Ok(())
    }
}
