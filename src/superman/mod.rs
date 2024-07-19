
use r2d2_sqlite::SqliteConnectionManager;


use crate::models::player::Player;

lazy_static!
{
	static ref SQLITE: r2d2::Pool<SqliteConnectionManager> =
	{
		let manager = SqliteConnectionManager::file("frosthaven.db");

		r2d2::Pool::new(manager).unwrap()
	};
}

pub fn build_db()
{

	SQLITE.get().unwrap().execute
	("
		CREATE TABLE IF NOT EXISTS players
		(
			id          INTEGER PRIMARY KEY,
			name        TEXT NOT NULL,
			initiative  INTEGER NOT NULL,
			is_monster  BOOLEAN NOT NULL,
			turn_complete BOOLEAN NOT NULL
		);
	", ()).unwrap();
}

pub fn clear_db()
{
	SQLITE.get().unwrap().execute
	("
		DELETE FROM players;
	", ()).unwrap();
}

pub fn insert_player(player: &Player)
{
	SQLITE.get().unwrap().execute
	("
		INSERT INTO players (name, initiative, is_monster, turn_complete)
		VALUES (?1, ?2, ?3, FALSE)
	", (&player.name, &player.initiative, &player.is_monster)).unwrap();
}

pub fn update_player_initiative_by_player_id(id: i32, initiative: i32)
{
	SQLITE.get().unwrap().execute
	("
		UPDATE players
		SET initiative = ?1
		WHERE id = ?2
	", (initiative, id)).unwrap();
}

pub fn clear_all_player_initiative()
{

	SQLITE.get().unwrap().execute
	("
		UPDATE players
		SET initiative = -1;
	", ()).unwrap();
}

pub fn clear_player_initiative_by_player_name(player_name: &str)
{
	SQLITE.get().unwrap().execute
	("
		UPDATE players
		SET initiative = -1
		WHERE name = ?1
	", [player_name]).unwrap();
}

pub fn delete_player_by_player_name(player_name: &str)
{
	SQLITE.get().unwrap().execute
	("
		DELETE FROM players
		WHERE name = ?1
	", [player_name]).unwrap();
}

pub fn get_all_players_by_initiative() -> Vec<Player>
{
	let pool = SQLITE.get().unwrap();
	let mut statement = pool.prepare
	("
		SELECT id, name, initiative, is_monster, turn_complete
		FROM players
		ORDER BY initiative
	").unwrap();

	let player_iter = statement.query_map([], |row|
		{
			Ok
			(
				Player
				{
					id: row.get(0).unwrap(),
					name: row.get(1).unwrap(),
					initiative: row.get(2).unwrap(),
					is_monster: row.get(3).unwrap(),
					turn_complete: row.get(4).unwrap(),
				}
			)
		}).unwrap();

	let mut players = Vec::new();

	for player in player_iter
	{
		players.push(player.unwrap());
	}

	players
}

pub fn get_player_by_name(name: &str) -> Option<Player>
{

	let pool = SQLITE.get().unwrap();
	let mut statement = pool.prepare
	("
		SELECT id, name, initiative, is_monster, turn_complete
		FROM players p
		WHERE p.name = ?1
	").unwrap();

	let player_iter = statement.query_map([name], |row|
		{
			Ok
				(
					Player
					{
						id: row.get(0).unwrap(),
						name: row.get(1).unwrap(),
						initiative: row.get(2).unwrap(),
						is_monster: row.get(3).unwrap(),
						turn_complete: row.get(4).unwrap(),
					}
				)
		}).unwrap();

	match player_iter.last()
	{
		Some(player) => Some(player.unwrap()),
		None => None,
	}
}

pub fn get_current_player() -> Option<Player>
{
	let pool = SQLITE.get().unwrap();
	let mut statement = pool.prepare
	("
		SELECT id, name, initiative, is_monster, turn_complete
		FROM players p
		WHERE p.turn_complete IS FALSE
		ORDER BY initiative
		LIMIT 1
	").unwrap();

	let player_iter = statement.query_map([], |row|
		{
			Ok
				(
					Player
					{
						id: row.get(0).unwrap(),
						name: row.get(1).unwrap(),
						initiative: row.get(2).unwrap(),
						is_monster: row.get(3).unwrap(),
						turn_complete: row.get(4).unwrap(),
					}
				)
		}).unwrap();

	match player_iter.last()
	{
		Some(player) => Some(player.unwrap()),
		None => None,
	}
}

pub fn mark_player_turn_complete(player_id: i32)
{
	SQLITE.get().unwrap().execute
	("
		UPDATE players
		SET turn_complete = TRUE
		WHERE id = ?1
	", [player_id]).unwrap();
}

pub fn mark_all_player_turns_incomplete()
{
	SQLITE.get().unwrap().execute
	("
		UPDATE players
		SET turn_complete = FALSE
	", ()).unwrap();
}