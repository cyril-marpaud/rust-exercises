use axum::{
	Router,
	extract::Json,
	http::{HeaderMap, StatusCode},
	response::IntoResponse,
	routing::{delete, get, post, put},
};
use axum_extra::TypedHeader;
use headers::{Authorization, authorization::Basic};
use serde::Deserialize;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
	let router = Router::new()
		.route("/", get(home))
		.route("/explore", post(explore))
		.route("/cave", put(cave))
		.route("/chest", get(chest))
		.route("/open_chest", post(open_chest))
		.route("/boat", delete(boat))
		.route("/return", get(return_home));

	let listener = tokio::net::TcpListener::bind("0.0.0.0:1337").await.unwrap();

	println!("The server is up and running ! Do not close this terminal.");
	println!("You can start by sending a GET request to http://localhost:1337/");
	println!("Follow the instructions in the responses to progress through the adventure.");

	axum::serve(listener, router).await.unwrap();
}

async fn home() -> impl IntoResponse {
	(
		StatusCode::OK,
		"You wake up stranded on a deserted island, with nothing but a compass in your pocket. To explore the surroundings, send a POST request to /explore with the body containing \"north\".",
	)
}

async fn explore(body: String) -> impl IntoResponse {
	match body.trim() == "north" {
		true => (
			StatusCode::OK,
			"While walking, you find a map showing the location of a cave and the number 3141. To go there, send a PUT request to /cave containing the JSON object { \"password\": \"coconut\" }.",
		),
		false => (
			StatusCode::BAD_REQUEST,
			"To continue, send { \"direction\": \"north\" }.",
		),
	}
}

#[derive(Deserialize)]
struct CaveReq {
	password: String,
}
async fn cave(Json(req): Json<CaveReq>) -> impl IntoResponse {
	match req.password == "coconut" {
		true => (
			StatusCode::OK,
			"The password works, the cave opens! You stumble upon a locked chest. Send a GET request to /chest with the header X-CHEST: 'inspect' to examine the chest.",
		),
		false => (
			StatusCode::BAD_REQUEST,
			"The password is incorrect. Try 'coconut'.",
		),
	}
}

async fn chest(headers: HeaderMap) -> impl IntoResponse {
	match headers.get("X-CHEST") {
		Some(h) if h == "inspect" => (
			StatusCode::OK,
			"The chest has a four-digit combination. Send a POST request to /open_chest with a form field 'code=3141' to try to open it.",
		),
		_ => (
			StatusCode::BAD_REQUEST,
			"To inspect the chest, add the header X-CHEST: inspect.",
		),
	}
}

async fn open_chest(
	axum::extract::Form(form): axum::extract::Form<HashMap<String, String>>,
) -> impl IntoResponse {
	if let Some("3141") = form.get("code").map(String::as_str) {
		(
			StatusCode::OK,
			"The chest opens: inside, you find treasure and a new map showing the location of a hidden boat, along with a note: captain/crustacean. Send a DELETE request to /boat authenticating with username captain and password crustacean (HTTP Basic authentication) to start the boat.",
		)
	} else {
		(
			StatusCode::BAD_REQUEST,
			"Wrong code. Try 'code=3141' in the form.",
		)
	}
}

async fn boat(TypedHeader(auth): TypedHeader<Authorization<Basic>>) -> impl IntoResponse {
	match auth.username() == "captain" && auth.password() == "crustacean" {
		true => (
			StatusCode::OK,
			"You can start the boat thanks to the map and the correct credentials. To set course for home, send a GET request to /return with the header X-CAP: home.",
		),
		false => (
			StatusCode::BAD_REQUEST,
			"You need the captain/crustacean credentials in basic_auth.",
		),
	}
}

async fn return_home(headers: HeaderMap) -> impl IntoResponse {
	match headers.get("X-CAP") {
		Some(h) if h == "home" => (
			StatusCode::OK,
			"You return home safely, treasure in hand. The adventure is over!",
		),
		_ => (
			StatusCode::BAD_REQUEST,
			"To return home, add the header X-CAP: home.",
		),
	}
}
