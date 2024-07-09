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

#[derive(Debug)]
struct LogLine<'a> {
	ts: f64,
	name: &'a str,
	typ: EventType,
	depth: usize,
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

	let input_arr: Vec<&str> = INPUT_S.split("\n").collect();

	let entries: Vec<Entry> = map_to_entries(input_arr);

	let mut stack: Vec<Entry> = Vec::new();

	let mut log_stack: Vec<LogLine> = Vec::new();

	for entry in entries {
		match entry.typ {
			EventType::Start => {
				log_stack.push(LogLine {
					ts: entry.ts,
					name: entry.name,
					typ: entry.typ,
					depth: stack.len(),
				});
				stack.push(entry);
				()
			}
			EventType::End => {
				stack.pop();
				log_stack.push(LogLine {
					ts: entry.ts,
					name: entry.name,
					typ: entry.typ,
					depth: stack.len(),
				});
				()
			}
		}
	}

	for line in log_stack {
		println!(
			"{}fn {} {} at {}",
			pad_line(line.depth),
			line.name,
			match line.typ {
				EventType::End => "Ended__",
				EventType::Start => "Started",
			},
			line.ts
		)
	}
}

fn map_to_entries(arr: Vec<&str>) -> Vec<Entry> {
	let mut stack: Vec<Entry> = Vec::new();
	for row in arr {
		let cols: Vec<&str> = row.split(", ").collect();
		let ts: f64 = match cols[0].parse::<f64>() {
			Err(_) => panic!("Bad input"),
			Ok(res) => res,
		};
		let name: &str = cols[1];
		let typ: EventType = match cols[2] {
			"Start" => EventType::Start,
			"End" => EventType::End,
			_ => panic!("Bad data"),
		};

		stack.push(Entry { ts, name, typ })
	}
	stack
}

fn pad_line(depth: usize) -> String {
	"\t".repeat(depth)
}
