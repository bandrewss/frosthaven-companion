
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Player
{
	pub id: i32,
	pub name: String,
	pub initiative: i32,
	pub is_monster: bool,
	pub turn_complete: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePlayer
{
	pub name: String,
	pub initiative: i32,
	pub is_monster: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerInitiative
{
	pub name: String,
	pub initiative: i32,
}


#[derive(Serialize, Deserialize)]
pub struct PlayerName
{
	pub name: String,
}
