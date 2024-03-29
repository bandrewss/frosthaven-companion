#[macro_use]
extern crate lazy_static;

use actix_files::Files;
use actix_web::{App, HttpServer};
use tera::Tera;

mod superman;
mod models;
mod gamestate;
mod rest;

lazy_static!
{
	pub static ref TEMPLATES: Tera =
	{
		// maybe make this mut?
		let tera = match Tera::new("./tera/*/*.tera.html")
		{
			Ok(t) => t,
			Err(e) =>
			{
				println!("Parsing error(s): {}", e);
				::std::process::exit(1);
			}
		};
		tera
	};
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	superman::build_db();

	HttpServer::new(||
		{
			App::new()
				.service(Files::new("/frosthaven/javascript", "./static/javascript/").show_files_listing())
				.service(Files::new("/frosthaven/css", "./static/css/").show_files_listing())
				.service(rest::master_display::get_frosthaven_master_display)
				.service(rest::king_display::get_frosthaven_king_display)
				.service(rest::gamestate::get_frosthaven_gamestate_current)
				.service(rest::gamestate::delete_frosthaven_session)
				.service(rest::gamestate::post_frosthaven_gamestate_transition)
				.service(rest::gamestate::post_frosthaven_gamestate_next_player)
				.service(rest::player::put_frosthaven_player)
				.service(rest::player::post_frosthaven_initiative)
				.service(rest::player::delete_frosthaven_player_initiative)
				.service(rest::player::delete_frosthaven_player)
		})
		.bind(("localhost", 8080))?
		.run()
		.await
}
