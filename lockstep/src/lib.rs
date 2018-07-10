extern crate serde;

use std::fmt::Debug;
use std::hash::Hash;

use serde::{Serialize, Deserialize};

pub struct PlayerId(u32);

pub trait Command: Eq + PartialEq + Clone + Debug + Deserialize<'static> + Serialize
{
}

pub struct TurnCommands<Command: self::Command>(Vec<(PlayerId, Vec<Command>)>);

pub trait State: Eq + PartialEq + Hash + Clone + Debug + Deserialize<'static> + Serialize
{
    type Command: self::Command;

    fn is_possible(&self, command: &Self::Command) -> bool;

    fn execute(&mut self, commands: &TurnCommands<Self::Command>);
}
