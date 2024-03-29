

use actix_web::{get, HttpResponse, Responder};
use tera::Context;

use crate::gamestate::{Gamestate, GAMESTATE_MUTEX, GAME_TURN};
use crate::TEMPLATES;
use crate::superman;

#[get("/frosthaven/master-display")]
pub async fn get_frosthaven_master_display() -> impl Responder
{
	let gamestate_lock = GAMESTATE_MUTEX.lock().unwrap();

	let mut tera_context = Context::new();
	tera_context.insert("gamestate", &(*gamestate_lock.to_string()));

	match *gamestate_lock
	{
		Gamestate::PreGame => (),
		Gamestate::Standby =>
		{
			let game_turn_lock = GAME_TURN.lock().unwrap();

			tera_context.insert("gameturn", &(*game_turn_lock));
		},
		Gamestate::InitiativeSubmit(_) =>
		{
			let players = superman::get_all_players_by_initiative();

			tera_context.insert("players", &players);
		},
		Gamestate::PreTurn =>
		{
			let game_turn_lock = GAME_TURN.lock().unwrap();

			tera_context.insert("gameturn", &(*game_turn_lock));
		},
		Gamestate::Turn(_) =>
		{
			let players = superman::get_all_players_by_initiative();

			tera_context.insert("players", &players);
		},
		Gamestate::PostTurn =>
		{
			let game_turn_lock = GAME_TURN.lock().unwrap();

			tera_context.insert("gameturn", &(*game_turn_lock));
		},
	};

	let page_name = get_page_name(&(*gamestate_lock));
	let render = TEMPLATES.render(page_name.as_str(), &tera_context).unwrap();

	HttpResponse::Ok().body(render)
}

fn get_page_name(gamestate: &Gamestate) -> String
{
	let dir = "master_display";
	let extension = "tera.html";

	match gamestate
	{
		Gamestate::PreGame => format!("{}/pre_game.{}", dir, extension),
		Gamestate::Standby => format!("{}/standby.{}", dir, extension),
		Gamestate::InitiativeSubmit(_) => format!("{}/initiative_submit.{}", dir, extension),
		Gamestate::PreTurn => format!("{}/pre_turn.{}", dir, extension),
		Gamestate::Turn(_) => format!("{}/turn.{}", dir, extension),
		Gamestate::PostTurn => format!("{}/post_turn.{}", dir, extension),
	}
}