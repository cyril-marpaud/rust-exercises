use std::{
	fs::File,
	io::{self, BufWriter, Read, Write},
	path::PathBuf,
};

struct Question {
	statement: String,
	choices: Vec<String>,
	answer: usize,
}

fn read_questions(p: &PathBuf) -> Vec<Question> {
	let mut questions = Vec::new();

	let mut file_content = String::new();
	let mut file = File::open(p).unwrap();
	file.read_to_string(&mut file_content).unwrap();

	for l in file_content.lines() {
		let parts: Vec<&str> = l.split('|').collect();

		let question = Question {
			statement: parts[0].to_string(),
			choices: parts[1..5].iter().map(|&s| s.to_string()).collect(),
			answer: parts[5].parse().unwrap(),
		};
		questions.push(question);
	}

	questions
}

fn request_response(question: &Question) -> usize {
	println!("{}", question.statement);

	for (index, option) in question.choices.iter().enumerate() {
		println!("{}: {option}", index + 1);
	}

	let mut response = String::new();
	io::stdin().read_line(&mut response).unwrap();
	response.trim().parse().unwrap()
}

fn check_response(user_response: usize, question: &Question) -> bool {
	println!("Chosen response: {}", question.choices[user_response - 1]);
	let correct = user_response == question.answer;
	match correct {
		true => println!("Correct!"),
		false => println!(
			"Incorrect! The correct answer was {}",
			question.choices[question.answer - 1]
		),
	}
	correct
}

fn save_result(p: &PathBuf, score: usize) {
	let file = File::create(p).unwrap();
	let mut file = BufWriter::new(file);
	writeln!(file, "Final score: {score}",).unwrap();
}

pub fn play() {
	let mut score = 0;

	let data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
	let q_path = data_path.join("questions.txt");
	let r_path = data_path.join("results.txt");

	for question in read_questions(&q_path) {
		let res = request_response(&question);
		if check_response(res, &question) {
			score += 1;
		}
	}

	println!("Your score is: {score}");
	save_result(&r_path, score);
}

#[test]
fn no_unwrap_or_expect() {
	use std::{fs::read_to_string, path::PathBuf};
	let last_line = line!() - 4;

	let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/e04_1_quiz/mod.rs");

	let content = read_to_string(file_path)
		.expect("Failed to read source file")
		.lines()
		.take(last_line as usize)
		.collect::<String>();

	let forbidden = ["unwrap(", "expect("];

	forbidden.iter().for_each(|f| {
		assert!(!content.contains(f));
	});
}

#[test]
fn thiserror_in_deps() {
	use std::fs::read_to_string;
	use toml::Value;

	let cargo_toml = read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");
	let cargo: Value = toml::from_str(&cargo_toml).expect("Failed to parse Cargo.toml as TOML");

	let deps = cargo
		.get("dependencies")
		.and_then(|v| v.as_table())
		.expect("[dependencies] section missing or malformed");

	assert!(
		deps.contains_key("thiserror"),
		"The 'thiserror' dependency is missing from [dependencies]"
	);
}
