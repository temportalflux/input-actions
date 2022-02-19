use crate::{
	action, binding,
	event::{self, InputReceiver, InputSender},
	Consts, WeakLockConfig,
};
use std::{
	collections::{HashMap, HashSet},
	sync::{Arc, RwLock, Weak},
	time::Instant,
};

pub type ArcLockUser = Arc<RwLock<User>>;
pub type WeakLockUser = Weak<RwLock<User>>;
pub struct User {
	config: WeakLockConfig,
	consts: Weak<RwLock<Consts>>,

	name: String,

	active_layout: binding::LayoutId,
	enabled_action_sets: HashMap<binding::ActionSetId, binding::ActionSet>,
	bound_actions: HashMap<BindingStateKey, action::Id>,
	action_states: HashMap<action::Id, action::ArcLockState>,
	ticking_states: HashSet<action::Id>,

	input_receiver: InputReceiver,
	input_sender: InputSender,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BindingStateKey {
	set_id: binding::ActionSetId,
	layout: binding::LayoutId,
	sources: Vec<binding::Source>,
}

impl BindingStateKey {
	fn contains(&self, source: binding::Source) -> bool {
		self.sources.contains(&source)
	}
}

impl User {
	pub fn new(name: String) -> Self {
		let (input_sender, input_receiver) = crossbeam_channel::unbounded();
		Self {
			config: Weak::new(),
			consts: Weak::new(),
			name,
			active_layout: binding::LayoutId::default(),
			enabled_action_sets: HashMap::new(),
			bound_actions: HashMap::new(),
			action_states: HashMap::new(),
			ticking_states: HashSet::new(),
			input_receiver,
			input_sender,
		}
	}

	pub fn with_config(mut self, config: WeakLockConfig) -> Self {
		self.config = config;
		self
	}

	pub fn with_consts(mut self, consts: Weak<RwLock<Consts>>) -> Self {
		self.consts = consts;
		self
	}

	pub fn arclocked(self) -> ArcLockUser {
		Arc::new(RwLock::new(self))
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub(crate) fn input_sender(&self) -> &event::InputSender {
		&self.input_sender
	}

	/// Sets the layout of a user.
	/// If not called, all user's start with a `None` layout (default layout).
	pub fn set_layout(&mut self, layout: binding::LayoutId) {
		self.active_layout = layout;
		self.bound_actions.clear();
		self.action_states.clear();
		self.ticking_states.clear();
		let set_ids = self.enabled_action_sets.keys().cloned().collect::<Vec<_>>();
		for set_id in set_ids {
			self.add_action_states(set_id);
		}
	}

	/// Enables a provided [`action set`](binding::ActionSet) for a given user.
	/// When enabled, a user will receive input events for the actions in the [`action set`](binding::ActionSet),
	/// until the set is disabled (or until [`crate::DeviceCache::update`] stops being called).
	pub fn enable_action_set(&mut self, id: binding::ActionSetId) {
		if let Some(arc_config) = self.config.upgrade() {
			if let Ok(config) = arc_config.read() {
				if let Some(action_set) = config.get_action_set(&id) {
					self.enabled_action_sets.insert(id, action_set.clone());
					self.add_action_states(id);
				}
			}
		}
	}

	pub fn disable_action_set(&mut self, id: binding::ActionSetId) {
		self.enabled_action_sets.remove(&id);
		self.remove_action_states(&id);
	}

	pub fn get_action_in(user: &ArcLockUser, id: action::Id) -> Option<action::WeakLockState> {
		match user.read() {
			Ok(user) => user.get_action(id),
			_ => None,
		}
	}

	pub fn get_action(&self, id: action::Id) -> Option<action::WeakLockState> {
		self.action_states.get(id).map(|arc| Arc::downgrade(&arc))
	}

	fn add_action_states(&mut self, set_id: binding::ActionSetId) {
		if let Some(action_binding_map) = self
			.enabled_action_sets
			.get(&set_id)
			.unwrap()
			.get(&self.active_layout)
		{
			for (action_id, behavior_binding) in action_binding_map.iter() {
				self.bound_actions.insert(
					BindingStateKey {
						set_id: set_id,
						layout: self.active_layout,
						sources: behavior_binding.sources(),
					},
					action_id,
				);
				let action_state = action::State::new(behavior_binding.clone());
				let must_tick = action_state.requires_updates();
				let arc = action_state.arclocked();
				if must_tick {
					self.ticking_states.insert(action_id);
				}
				self.action_states.insert(action_id, arc);
			}
		}
	}

	fn remove_action_states(&mut self, set_id: &binding::ActionSetId) {
		// TODO: Can use `drain_filter` (https://github.com/rust-lang/rust/issues/59618) when stablized
		let mut retained_actions = HashMap::new();
		let mut removed_actions = HashMap::new();
		for (bound_state_key, action_id) in self.bound_actions.drain() {
			if bound_state_key.set_id == *set_id {
				removed_actions.insert(bound_state_key, action_id);
			} else {
				retained_actions.insert(bound_state_key, action_id);
			}
		}
		self.bound_actions = retained_actions;
		for (_, action_id) in removed_actions {
			self.action_states.remove(action_id);
			self.ticking_states.remove(action_id);
		}
	}

	fn screen_size(&self) -> (f64, f64) {
		let arc_consts = self.consts.upgrade().unwrap();
		let consts = arc_consts.read().unwrap();
		consts.screen_size
	}

	pub(crate) fn process_event(
		&mut self,
		source: binding::Source,
		state: &event::State,
		time: &Instant,
	) {
		let action_ids_bound_to_source =
			self.bound_actions
				.iter()
				.filter_map(|(key, action_id)| match key.contains(source) {
					true => Some(action_id),
					false => None,
				});
		let screen_size = self.screen_size();
		for action_id in action_ids_bound_to_source {
			if let Some(arc_state) = self.action_states.get_mut(action_id) {
				let mut action_state = arc_state.write().unwrap();
				action_state.process_event(source, state.clone(), &time, screen_size);
			}
		}
	}

	pub fn update(&mut self, time: &Instant) {
		while let Ok((source, state)) = self.input_receiver.try_recv() {
			self.process_event(source, &state, &time);
		}

		for action_id in self.ticking_states.iter() {
			let arc_state = self.action_states.get(action_id).unwrap();
			let mut action_state = arc_state.write().unwrap();
			action_state.update(time);
		}
	}
}
