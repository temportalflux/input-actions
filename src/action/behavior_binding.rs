use crate::{
	action::behavior::{Behavior, Kind},
	binding::Source,
	device,
};
use std::{collections::HashMap, time::Instant};

type BehaviorList = Vec<Box<dyn Behavior + 'static + Send + Sync>>;

#[derive(Clone)]
pub enum BehaviorBinding {
	Source(SourceBehavior),
	Container(Vec<BehaviorBinding>, BehaviorList),
	Select(HashMap<device::Kind, BehaviorBinding>),
}

#[derive(Clone)]
pub struct SourceBehavior {
	source: Source,
	behaviors: BehaviorList,
	latest_value: f64,
}

impl std::fmt::Debug for BehaviorBinding {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Source(binding) => write!(
				f,
				"BehaviorBinding({:?}, behaviors=[{}])",
				binding.source,
				binding
					.behaviors
					.iter()
					.map(|behavior| behavior.debug_string())
					.collect::<Vec<_>>()
					.join(", ")
			),
			Self::Container(bindings, behaviors) => write!(
				f,
				"BehaviorContainer({:?}, behaviors=[{}])",
				bindings,
				behaviors
					.iter()
					.map(|behavior| behavior.debug_string())
					.collect::<Vec<_>>()
					.join(", ")
			),
			Self::Select(bindings) => write!(f, "{:?}", bindings),
		}
	}
}

impl From<Source> for SourceBehavior {
	fn from(source: Source) -> Self {
		Self {
			source,
			behaviors: Vec::new(),
			latest_value: 0.0,
		}
	}
}

impl From<Source> for BehaviorBinding {
	fn from(source: Source) -> Self {
		Self::Source(source.into())
	}
}

impl SourceBehavior {
	pub fn with_behavior<TBehavior>(mut self, behavior: TBehavior) -> Self
	where
		TBehavior: Behavior + 'static + Send + Sync + Clone,
	{
		self.add_behavior(behavior);
		self
	}

	pub fn add_behavior<TBehavior>(&mut self, behavior: TBehavior)
	where
		TBehavior: Behavior + 'static + Send + Sync + Clone,
	{
		self.behaviors.push(Box::new(behavior));
	}

	pub(crate) fn process(
		&mut self,
		source: Source,
		value: f64,
		time: &Instant,
		screen_size: &(f64, f64),
	) -> f64 {
		if self.source == source {
			self.latest_value = value;
			for behavior in self.behaviors.iter() {
				self.latest_value = behavior.map(source, self.latest_value, &time, &screen_size);
			}
		}
		self.latest_value
	}
}

impl BehaviorBinding {
	pub fn select<T>(options: T) -> Self
	where
		T: std::iter::Iterator<Item = (device::Kind, BehaviorBinding)>,
	{
		Self::Select(options.collect())
	}

	fn is_directly_applicable(&self, source: Source) -> bool {
		if let Self::Source(binding) = &self {
			return binding.source == source;
		}
		false
	}

	pub fn with_behavior<TBehavior>(mut self, behavior: TBehavior) -> Self
	where
		TBehavior: Behavior + 'static + Send + Sync + Clone,
	{
		match &mut self {
			Self::Container(_, behaviors) => {
				behaviors.push(Box::new(behavior));
			}
			Self::Source(src_behavior) => {
				src_behavior.add_behavior(behavior);
			}
			Self::Select(_) => unimplemented!(),
		}
		self
	}

	pub fn with_binding(mut self, binding: BehaviorBinding) -> Self {
		match &mut self {
			Self::Container(bindings, _) => {
				bindings.push(binding);
			}
			Self::Source(_) => unimplemented!(),
			Self::Select(_) => unimplemented!(),
		}
		self
	}

	pub(crate) fn sources(&self) -> Vec<Source> {
		match self {
			Self::Source(SourceBehavior { source, .. }) => vec![*source],
			Self::Container(bindings, _) => bindings
				.iter()
				.map(|binding| binding.sources().into_iter())
				.flatten()
				.collect(),
			Self::Select(bindings) => bindings
				.iter()
				.map(|(_, binding)| binding.sources().into_iter())
				.flatten()
				.collect(),
		}
	}

	pub(crate) fn process(
		&mut self,
		source: Source,
		mut value: f64,
		time: &Instant,
		screen_size: &(f64, f64),
	) -> f64 {
		match self {
			Self::Source(src_behavior) => src_behavior.process(source, value, &time, &screen_size),
			Self::Select(bindings) => {
				if let Some(binding) = bindings.get_mut(&source.device_kind()) {
					binding.process(source, value, &time, &screen_size)
				} else {
					0.0
				}
			}
			Self::Container(bindings, behaviors) => {
				let mut values = Vec::with_capacity(bindings.len());
				for behavior_binding in bindings.iter_mut() {
					let v = behavior_binding.process(source, value, &time, &screen_size);
					values.push(v);
					if behaviors.is_empty() && behavior_binding.is_directly_applicable(source) {
						value = v;
					}
				}
				if behaviors.is_empty() {
					value
				} else {
					for behavior in behaviors.iter() {
						match behavior.kind() {
							Kind::Map => {
								for value in values.iter_mut() {
									*value = behavior.map(source, *value, &time, &screen_size);
								}
							}
							Kind::Fold => {
								values = vec![behavior.fold(&values[..])];
							}
						}
					}
					values[0]
				}
			}
		}
	}
}

impl std::ops::Add<Source> for Source {
	type Output = BehaviorBinding;
	fn add(self, rhs: Source) -> Self::Output {
		BehaviorBinding::Container(vec![self.into(), rhs.into()], vec![])
	}
}

impl<TBehavior> std::ops::Add<TBehavior> for Source
where
	TBehavior: Behavior + 'static + Send + Sync + Clone,
{
	type Output = BehaviorBinding;
	fn add(self, rhs: TBehavior) -> Self::Output {
		BehaviorBinding::from(self).with_behavior(rhs)
	}
}

impl<TBehavior> std::ops::Add<TBehavior> for BehaviorBinding
where
	TBehavior: Behavior + 'static + Send + Sync + Clone,
{
	type Output = Self;
	fn add(self, rhs: TBehavior) -> Self::Output {
		self.with_behavior(rhs)
	}
}

impl std::ops::Add<BehaviorBinding> for BehaviorBinding {
	type Output = Self;
	fn add(self, rhs: BehaviorBinding) -> Self {
		Self::Container(vec![self, rhs], vec![])
	}
}

impl std::ops::Add<Source> for BehaviorBinding {
	type Output = Self;
	fn add(mut self, rhs: Source) -> Self {
		match &mut self {
			Self::Container(bindings, _) => {
				bindings.push(Self::from(rhs));
			}
			Self::Source(_) => unimplemented!(),
			Self::Select(_) => unimplemented!(),
		}
		self
	}
}

impl<const N: usize> From<[(device::Kind, BehaviorBinding); N]> for BehaviorBinding {
	fn from(other: [(device::Kind, BehaviorBinding); N]) -> Self {
		Self::select(other.iter().cloned())
	}
}
