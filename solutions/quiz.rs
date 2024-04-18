use std::{
	fs::File,
	io::{self, BufRead, BufReader, BufWriter, Write},
	num::ParseIntError,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuizError {
	#[error("Could not open file: {0} ({1})")]
	FileOpen(String, #[source] io::Error),
	#[error("File is malformed or empty")]
	MalformedFile,
	#[error("Failed to read line")]
	LineReadError(#[source] io::Error),
	#[error("Failed to parse the correct answer: {0} ({1})")]
	ParseAnswerError(String, #[source] ParseIntError),
	#[error("Failed to read the response")]
	ReadResponseError(#[source] io::Error),
	#[error("Failed to parse the response: {0} ({1})")]
	ParseResponseError(String, #[source] ParseIntError),
	#[error("Response is out of valid range: {0} (1-4)")]
	ResponseOutOfRange(usize),
	#[error("Failed to create result file: {0} ({1})")]
	FileCreate(String, #[source] io::Error),
	#[error("Failed to write to result file: {0} ({1})")]
	WriteResultError(String, #[source] io::Error),
}

struct Question {
	statement: String,
	choices: Vec<String>,
	answer: usize,
}

fn read_questions(p: &str) -> Result<Vec<Question>, QuizError> {
	let file = File::open(p).map_err(|e| QuizError::FileOpen(p.to_string(), e))?;
	let reader = BufReader::new(file);

	reader
		.lines()
		.map(|line| {
			let l = line.map_err(QuizError::LineReadError)?;

			let parts: Vec<&str> = l.split('|').collect();
			if parts.len() < 6 {
				return Err(QuizError::MalformedFile);
			}

			Ok(Question {
				statement: parts[0].to_string(),
				choices: parts[1..5].iter().map(|&s| s.to_string()).collect(),
				answer: parts[5]
					.parse::<usize>()
					.map_err(|e| QuizError::ParseAnswerError(parts[5].to_string(), e))?
					- 1,
			})
		})
		.collect()
}

fn request_response(question: &Question) -> Result<usize, QuizError> {
	println!("{}", question.statement);
	for (index, option) in question.choices.iter().enumerate() {
		println!("{}: {}", index + 1, option);
	}

	let mut response = String::new();
	io::stdin()
		.read_line(&mut response)
		.map_err(QuizError::ReadResponseError)?;

	response
		.trim()
		.parse::<usize>()
		.map_err(|e| QuizError::ParseResponseError(response.trim().to_string(), e))
}

fn check_response(user_response: usize, question: &Question) -> Result<bool, QuizError> {
	if user_response < 1 || user_response > question.choices.len() {
		return Err(QuizError::ResponseOutOfRange(user_response));
	}

	println!("Chosen response: {}", question.choices[user_response - 1]);

	let correct = user_response == question.answer;
	match correct {
		true => println!("Correct!"),
		false => println!(
			"Incorrect! The correct answer was {}",
			question.choices[question.answer]
		),
	}
	Ok(correct)
}

fn save_result(p: &str, score: usize) -> Result<(), QuizError> {
	let file = File::create(p).map_err(|e| QuizError::FileCreate(p.to_string(), e))?;
	let mut file = BufWriter::new(file);
	writeln!(file, "Final score: {}", score)
		.map_err(|e| QuizError::WriteResultError(p.to_string(), e))
}

pub fn play() -> Result<(), QuizError> {
	let questions = read_questions("data/questions.txt")?;

	let mut score = 0;
	for q in questions {
		let r = request_response(&q)?;
		if check_response(r, &q)? {
			score += 1;
		}
	}

	println!("Your score is: {score}");
	save_result("data/results.txt", score)?;
	Ok(())
}
