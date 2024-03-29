
use actix_web::{put, post, delete, HttpResponse, Responder, web};

use crate::gamestate::{Gamestate, GAMESTATE_MUTEX};
use crate::superman;
use crate::models;

#[put("/frosthaven/player")]
pub async fn put_frosthaven_player(player: web::Json<models::player::CreatePlayer>)
								   -> impl Responder
{
	let mut gamestate_lock = GAMESTATE_MUTEX.lock().unwrap();

	match *gamestate_lock
	{
		Gamestate::InitiativeSubmit(i) => { *gamestate_lock = Gamestate::InitiativeSubmit(i + 1) }
		_ => { return HttpResponse::BadRequest().body("Cannot add Player right now"); }
	}

	if !initiative_is_valid(player.initiative)
	{
		return HttpResponse::UnprocessableEntity().body("Initiative must be between 1 and 99");
	}

	match superman::get_player_by_name(player.name.as_str())
	{
		Some(player) =>
			{
				return HttpResponse::BadRequest()
					.body(format!("Player {} already exists", player.name));
			},
		None =>
			{
				let new_player = models::player::Player
				{
					id: 0,
					name: player.name.clone(),
					initiative: player.initiative,
					is_monster: player.is_monster,
					turn_complete: false,
				};

				superman::insert_player(new_player)
			}
	};

	HttpResponse::NoContent().into()
}

#[post("/frosthaven/player/initiative")]
pub async fn post_frosthaven_initiative(player: web::Json<models::player::PlayerInitiative>)
										-> impl Responder
{
	let mut gamestate_lock = GAMESTATE_MUTEX.lock().unwrap();

	match *gamestate_lock
	{
		Gamestate::InitiativeSubmit(i) => { *gamestate_lock = Gamestate::InitiativeSubmit(i + 1) }
		_ =>
			{
				return HttpResponse::BadRequest().body("Cannot update player initiative right now");
			}
	}

	if !initiative_is_valid(player.initiative)
	{
		return HttpResponse::UnprocessableEntity().body("Initiative must be between 1 and 99");
	}

	match superman::get_player_by_name(player.name.as_str())
	{
		Some(p) => superman::update_player_initiative_by_player_id(p.id, player.initiative),
		None =>
			{
				return HttpResponse::BadRequest().body(format!("Player {} does not exist", player.name));
			}
	};

	HttpResponse::NoContent().into()
}

#[delete("/frosthaven/player/initiative")]
pub async fn delete_frosthaven_player_initiative(player_name: web::Json<models::player::PlayerName>)
											  -> impl Responder
{
	let mut gamestate_lock = GAMESTATE_MUTEX.lock().unwrap();

	match *gamestate_lock
	{
		Gamestate::InitiativeSubmit(i) => { *gamestate_lock = Gamestate::InitiativeSubmit(i + 1) }
		_ => { return HttpResponse::BadRequest(); }
	}

	superman::clear_player_initiative_by_player_name(player_name.name.as_str());

	HttpResponse::NoContent()
}

#[delete("/frosthaven/player")]
pub async fn delete_frosthaven_player(player_name: web::Json<models::player::PlayerName>)
									  -> impl Responder
{
	let mut gamestate_lock = GAMESTATE_MUTEX.lock().unwrap();

	match *gamestate_lock
	{
		Gamestate::InitiativeSubmit(i) => { *gamestate_lock = Gamestate::InitiativeSubmit(i + 1) }
		_ => { return HttpResponse::BadRequest(); }
	}

	superman::delete_player_by_player_name(player_name.name.as_str());

	HttpResponse::NoContent()
}

fn initiative_is_valid(initiative: i32) -> bool
{
	initiative > 0 && initiative < 100
}
