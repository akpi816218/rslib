use std::{
	fmt::Debug,
	fs::File,
	io::{Read, Write as _},
	path::PathBuf,
};

use clap::Parser;

#[derive(Parser)]
#[command(name = "rstrace")]
#[command(about = "A simple tool to trace function call times", long_about = None)]
struct CLIArgs {
	#[arg(
		value_name = "INPUT_FILE",
		short,
		long,
		help = "File to read from, or stdin if omitted"
	)]
	input: Option<PathBuf>,
	#[arg(
		value_name = "OUTPUT_FILE",
		short,
		long,
		help = "File to write to, or stdout if omitted"
	)]
	output: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug)]
enum EventType {
	Start,
	End,
}

impl std::fmt::Display for EventType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				EventType::End => "End",
				EventType::Start => "Start",
			}
		)
	}
}

#[derive(Debug)]
struct Entry<'a> {
	ts: f64,
	name: &'a str,
	typ: EventType,
}

fn main() {
	let cli: CLIArgs = CLIArgs::parse();

	let mut input_b: Vec<u8> = Vec::new();

	if cli.input.is_none() {
		let _ = std::io::stdin().lock().read_to_end(&mut input_b);
		println!("");
	} else {
		let mut file: File = File::open(cli.input.unwrap()).expect("Problem opening file");
		let _ = file.read_to_end(&mut input_b);
	}

	let _binding = String::from_utf8(input_b).unwrap();
	let input_s = _binding.trim();

	let mut outfile: Option<File> = None;
	let mut outfile_name: Option<String> = None;

	if cli.output.is_some() {
		let path = cli.output.unwrap();
		outfile_name = Some(path.to_str().unwrap().to_string());
		outfile = Some(File::create(path).expect("Failed to create file"));
	}

	let entries = map_to_entries(input_s.split("\n").collect());

	let mut stack: Vec<Entry> = Vec::new();

	for entry in entries {
		match entry.typ {
			EventType::Start => {
				print_line(&entry, stack.len(), &mut outfile);
				stack.push(entry);
				()
			}
			EventType::End => {
				stack.pop();
				print_line(&entry, stack.len(), &mut outfile);
				()
			}
		}
	}

	if outfile.is_some() {
		println!("Finished writing output to {}", outfile_name.unwrap());
	}
}

fn map_to_entries(arr: Vec<&str>) -> Vec<Entry> {
	let mut stack: Vec<Entry> = Vec::new();
	for row in arr {
		let cols: Vec<&str> = row.split(", ").collect();
		stack.push(Entry {
			ts: match cols[0].parse::<f64>() {
				Err(_) => panic!("Bad data"),
				Ok(res) => res,
			},
			name: cols[1],
			typ: match cols[2] {
				"Start" => EventType::Start,
				"End" => EventType::End,
				_ => panic!("Bad data"),
			},
		})
	}
	stack
}

fn pad_line(depth: usize) -> String {
	"> ".repeat(depth)
}

fn print_line(entry: &Entry, depth: usize, output: &mut Option<File>) -> () {
	let line = format!(
		"{}fn {} {} at {}",
		pad_line(depth),
		entry.name,
		match entry.typ {
			EventType::End => "Ended__",
			EventType::Start => "Started",
		},
		entry.ts
	);
	match output {
		Some(f) => {
			f.write_all(format!("{line}\n").as_bytes())
				.expect("Failed to write to file");
			()
		}
		None => {
			println!("{line}");
			()
		}
	}
}
