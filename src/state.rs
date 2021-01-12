use std::collections::HashMap;
use gdnative::prelude::*;

use crate::empty_signal;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StateMachine {
	states: HashMap<String, State>,
	current_state: Option<String>
}

#[methods]
impl StateMachine {

	pub fn new(_owner: &Node) -> StateMachine {
		StateMachine { states: HashMap::new(), current_state: None }
	}
	
	pub fn add(&mut self, from: String, to: String, when: String) {
		
		if !self.current_state.is_some() {
			self.current_state = Some(from.clone());
		}
		
		if !self.states.contains_key(&from.clone()) {
			self.states.insert(from.clone(), State::from_string(from.clone()));
		}
		if !self.states.contains_key(&to) {
			self.states.insert(to.clone(), State::from_string(to.clone()));
		}

		self.states.get_mut(&from).unwrap().add(when, to.clone());
	}

	pub fn get_state(&self) -> Option<String> {
		self.current_state.clone()
	}

	pub fn transition(&mut self, event: String) -> Option<String> {
		
		match self.get_state() {
			Some(state) =>	match self.states.get(&state).unwrap().transitions.get(&event) {
				Some(new_state) => self.current_state = Some(new_state.to_string()),
				None => ()
			},
			None => ()
		};

		self.get_state()
	}

	/* Returns whether the current state would transition to a DIFFERENT state if a given event is passed into the state machine */
	pub fn check_transition(&mut self, event: String) -> bool {
		match &self.current_state {
			Some(state) => self.states.get(state).unwrap().transitions.contains_key(&event),
			None => false
		}
	}

	pub fn validate(&self) -> (Vec<String>, Vec<String>) {
		let mut dead_ends = Vec::new();
		let orphans = self.states.values().map(|x| x.name.clone());

		let mut found = Vec::new();

		for state in self.states.values() {
			for target in state.transitions.values() {
				found.push(target);
			}
			if state.transitions.len() == 0 {
				dead_ends.push(state.name.clone());
			}
		}

		(dead_ends, orphans.filter(|x| !found.contains(&x)).collect())
	}
	
	fn register(builder: &ClassBuilder<Self>) {
		empty_signal!(builder, "on_state_changed"); // Fired when the State Machine enters a new state, contains the old ad the new states names
		empty_signal!(builder, "on_state_remained"); // Fired when the State Machine does not enter a new state after an event, contains the old states name

		empty_signal!(builder, "on_event"); // Fired when receiving any event, contains the event name
		empty_signal!(builder, "on_known_event"); // Fired when an known event is received, contains the event name
		empty_signal!(builder, "on_unknown_event"); // Fired when an unknown event is received, contains the event name
	}
}

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
	name: String,
	transitions: HashMap<String, String>
}

#[methods]
impl State {
	pub fn new(owner: &Node) -> State {
		Self { name: owner.get("name").to_string(), transitions: HashMap::new() }
	}

	pub fn from_string(name: String) -> State {
		Self { name, transitions: HashMap::new() }
	}

	fn add(&mut self, event: String, target: String) {
		self.transitions.insert(event, target);
	}

	fn register(builder: &ClassBuilder<Self>) {
		builder.add_property::<Dictionary>("transitions").done();
	}
}