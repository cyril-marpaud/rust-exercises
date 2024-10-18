use std::{
	fs::File,
	io::{self, BufRead, BufReader, BufWriter, Write},
};

struct Question {
	statement: String,
	choices: Vec<String>,
	answer: usize,
}

fn read_questions(p: &str) -> Vec<Question> {
	let file = File::open(p).unwrap();
	let reader = BufReader::new(file);

	let mut questions = Vec::new();

	for line in reader.lines() {
		let l = line.unwrap();
		let parts: Vec<&str> = l.split('|').collect();

		let question = Question {
			statement: parts[0].to_string(),
			choices: parts[1..5].iter().map(|&s| s.to_string()).collect(),
			answer: parts[5].parse::<usize>().unwrap() - 1,
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
	response.trim().parse::<usize>().unwrap()
}

fn check_response(user_response: usize, question: &Question) -> bool {
	println!("Chosen response: {}", question.choices[user_response - 1]);
	let correct = user_response == question.answer;
	match correct {
		true => println!("Correct!"),
		false => println!(
			"Incorrect! The correct answer was {}",
			question.choices[question.answer]
		),
	}
	correct
}

fn save_result(p: &str, score: usize) {
	let file = File::create(p).unwrap();
	let mut file = BufWriter::new(file);
	writeln!(file, "Final score: {}", score).unwrap();
}

pub fn play() {
	let mut score = 0;
	for question in read_questions("data/questions.txt") {
		let res = request_response(&question);
		if check_response(res, &question) {
			score += 1;
		}
	}

	println!("Your score is: {score}");
	save_result("data/results.txt", score);
}
