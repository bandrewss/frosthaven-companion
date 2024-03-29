use actix_web::{get, post, delete, HttpResponse, Responder};

use crate::gamestate::{Gamestate, GAMESTATE_MUTEX, GAME_TURN};
use crate::superman;

#[get("/frosthaven/gamestate/current")]
pub async fn get_frosthaven_gamestate_current() -> impl Responder
{
	let gamestate_lock = GAMESTATE_MUTEX.lock().unwrap();

	HttpResponse::Ok().body(format!("{}", *gamestate_lock))
}

#[delete("/frosthaven/gamestate")]
pub async fn delete_frosthaven_session() -> impl Responder
{
	superman::clear_db();

	let mut gamestate_lock = GAMESTATE_MUTEX.lock().unwrap();
	let mut game_turn = GAME_TURN.lock().unwrap();

	*gamestate_lock = Gamestate::PreGame;
	*game_turn = 0;

	HttpResponse::NoContent()
}

#[post("/frosthaven/gamestate/transition")]
pub async fn post_frosthaven_gamestate_transition() -> impl Responder
{
	let mut gamestate_lock = GAMESTATE_MUTEX.lock().unwrap();

	*gamestate_lock = match *gamestate_lock
	{
		Gamestate::PreGame => Gamestate::Standby,
		Gamestate::Standby =>
			{
				superman::clear_all_player_initiative();

				Gamestate::InitiativeSubmit(0)
			}
		Gamestate::InitiativeSubmit(_) => Gamestate::PreTurn,
		Gamestate::PreTurn =>
			{
				superman::mark_all_player_turns_incomplete();

				Gamestate::Turn(0)
			}
		Gamestate::Turn(_) =>
			{
				match superman::get_current_player()
				{
					Some(_) => { return HttpResponse::BadRequest(); }
					None => Gamestate::PostTurn,
				}
			}
		Gamestate::PostTurn =>
			{
				let mut game_turn_lock = GAME_TURN.lock().unwrap();
				*game_turn_lock += 1;

				Gamestate::Standby
			}
	};

	HttpResponse::NoContent()
}

#[post("/frosthaven/gamestate/next-player")]
pub async fn post_frosthaven_gamestate_next_player() -> impl Responder
{
	let mut gamestate_lock = GAMESTATE_MUTEX.lock().unwrap();

	match *gamestate_lock
	{
		Gamestate::Turn(i) => *gamestate_lock = Gamestate::Turn(i + 1),
		_ => return HttpResponse::BadRequest(),
	}

	let player = match superman::get_current_player()
	{
		None => return HttpResponse::BadRequest(),
		Some(p) => p,
	};

	superman::mark_player_turn_complete(player.id);

	HttpResponse::NoContent()
}
