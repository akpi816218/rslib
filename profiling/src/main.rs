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
	const INPUT_S: &str = "1, main, Start
2, A, Start
3, B, Start
4, B, End
5, A, End
6, C, Start
6.4, C, Start
6.5, C, Start
6.6, C, Start
6.62, C, End
6.63, C, Start
6.64, C, End
6.65, C, End
6.7, C, End
7, C, End
8, main, End";

	let entries = map_to_entries(INPUT_S.split("\n").collect());

	let mut stack: Vec<Entry> = Vec::new();

	for entry in entries {
		match entry.typ {
			EventType::Start => {
				print_line(&entry, stack.len());
				stack.push(entry);
				()
			}
			EventType::End => {
				stack.pop();
				print_line(&entry, stack.len());
				()
			}
		}
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

fn print_line(entry: &Entry, depth: usize) {
	println!(
		"{}fn {} {} at {}",
		pad_line(depth),
		entry.name,
		match entry.typ {
			EventType::End => "Ended__",
			EventType::Start => "Started",
		},
		entry.ts
	)
}
