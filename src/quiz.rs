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

	reader
		.lines()
		.map(|line| {
			let l = line.unwrap();
			let parts: Vec<&str> = l.split('|').collect();
			Question {
				statement: parts[0].to_string(),
				choices: parts[1..5].iter().map(|&s| s.to_string()).collect(),
				answer: parts[5].parse::<usize>().unwrap() - 1,
			}
		})
		.collect()
}

fn request_response(question: &Question) -> usize {
	println!("{}", question.statement);
	for (index, option) in question.choices.iter().enumerate() {
		println!("{}: {}", index + 1, option);
	}
	let mut response = String::new();
	io::stdin().read_line(&mut response).unwrap();
	response.trim().parse::<usize>().unwrap() - 1
}

fn check_response(user_response: usize, question: &Question) -> bool {
	println!("Chosen response: {}", question.choices[user_response]);
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
	let score = read_questions("data/questions.txt")
		.iter()
		.map(|q| (q, request_response(q)))
		.filter(|(q, r)| check_response(*r, q))
		.count();

	println!("Your score is: {}", score);
	save_result("data/results.txt", score);
}
