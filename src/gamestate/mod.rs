use std::fmt;
use std::sync::Arc;
use std::sync::Mutex;

lazy_static!
{
	pub static ref GAMESTATE_MUTEX: Arc<Mutex<Gamestate >> = Arc::new(Mutex::new(Gamestate::PreGame));
}

lazy_static!
{
	pub static ref GAME_TURN: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
}

pub enum Gamestate
{
	PreGame,
	Standby,
	InitiativeSubmit(i32), // count of how many initiatives have been submitted
	PreTurn,
	Turn(i32), // // could turn count into a subturn state object
	PostTurn,
}

impl fmt::Display for Gamestate
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let display = match self
		{
			Gamestate::PreGame => "PRE_GAME".to_owned(),
			Gamestate::Standby => "STANDBY".to_owned(),
			Gamestate::InitiativeSubmit(i) => format!("INITIATIVE_SUBMIT_{}", i),
			Gamestate::PreTurn => "PRE_TURN".to_owned(),
			Gamestate::Turn(i) => format!("TURN_{}", i),
			Gamestate::PostTurn => "POST_TURN".to_owned(),
		};

		write!(f, "{}", display)
	}
}