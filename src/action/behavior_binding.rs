use crate::{action::behavior::Behavior, binding::Source};
use std::time::Instant;

#[derive(Clone)]
pub enum BehaviorBinding {
	Source(SourceBehavior),
	Container(Vec<BehaviorBinding>),
}

#[derive(Clone)]
pub struct SourceBehavior {
	source: Source,
	behaviors: Vec<Box<dyn Behavior + 'static + Send + Sync>>,
	behavior_type_names: Vec<String>,
}

impl std::fmt::Debug for BehaviorBinding {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Source(binding) => write!(
				f,
				"BehaviorBinding({:?}, behaviors={:?})",
				binding.source, binding.behavior_type_names
			),
			Self::Container(bindings) => write!(f, "{:?}", bindings),
		}
	}
}

impl From<Source> for SourceBehavior {
	fn from(source: Source) -> Self {
		Self {
			source,
			behaviors: Vec::new(),
			behavior_type_names: Vec::new(),
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
		TBehavior: Behavior + 'static + Send + Sync + Clone + Sized,
	{
		self.add_behavior(behavior);
		self
	}

	pub fn add_behavior<TBehavior>(&mut self, behavior: TBehavior)
	where
		TBehavior: Behavior + 'static + Send + Sync + Clone + Sized,
	{
		self.behavior_type_names
			.push(std::any::type_name::<TBehavior>().to_owned());
		self.behaviors.push(Box::new(behavior));
	}

	pub(crate) fn process(
		&self,
		source: Source,
		mut value: f64,
		time: &Instant,
		screen_size: &(f64, f64),
	) -> f64 {
		for behavior in self.behaviors.iter() {
			value = behavior.process(source, value, &time, &screen_size);
		}
		value
	}
}

impl BehaviorBinding {
	pub fn with_behavior<TBehavior>(mut self, behavior: TBehavior) -> Self
	where
		TBehavior: Behavior + 'static + Send + Sync + Clone + Sized,
	{
		match &mut self {
			Self::Container(_) => unimplemented!(),
			Self::Source(src_behavior) => {
				src_behavior.add_behavior(behavior);
			}
		}
		self
	}

	pub fn with_binding(mut self, binding: BehaviorBinding) -> Self {
		match &mut self {
			Self::Container(bindings) => {
				bindings.push(binding);
			}
			Self::Source(_) => unimplemented!(),
		}
		self
	}

	pub(crate) fn sources(&self) -> Vec<Source> {
		match self {
			Self::Source(SourceBehavior { source, .. }) => vec![*source],
			Self::Container(bindings) => bindings
				.iter()
				.map(|binding| binding.sources().into_iter())
				.flatten()
				.collect(),
		}
	}

	pub(crate) fn process(
		&self,
		source: Source,
		mut value: f64,
		time: &Instant,
		screen_size: &(f64, f64),
	) -> f64 {
		match self {
			Self::Source(src_behavior) => src_behavior.process(source, value, &time, &screen_size),
			Self::Container(bindings) => {
				for behavior_binding in bindings {
					value = behavior_binding.process(source, value, &time, &screen_size);
				}
				value
			}
		}
	}
}

impl std::ops::Add<Source> for Source {
	type Output = BehaviorBinding;
	fn add(self, rhs: Source) -> Self::Output {
		BehaviorBinding::Container(vec![self.into(), rhs.into()])
	}
}

impl<TBehavior> std::ops::Add<TBehavior> for Source
where
	TBehavior: Behavior + 'static + Send + Sync + Clone + Sized,
{
	type Output = BehaviorBinding;
	fn add(self, rhs: TBehavior) -> Self::Output {
		BehaviorBinding::from(self).with_behavior(rhs)
	}
}

impl<TBehavior> std::ops::Add<TBehavior> for BehaviorBinding
where
	TBehavior: Behavior + 'static + Send + Sync + Clone + Sized,
{
	type Output = Self;
	fn add(self, rhs: TBehavior) -> Self::Output {
		self.with_behavior(rhs)
	}
}

impl std::ops::Add<BehaviorBinding> for BehaviorBinding {
	type Output = Self;
	fn add(self, rhs: BehaviorBinding) -> Self {
		Self::Container(vec![self, rhs])
	}
}

impl std::ops::Add<Source> for BehaviorBinding {
	type Output = Self;
	fn add(mut self, rhs: Source) -> Self {
		match &mut self {
			Self::Container(bindings) => {
				bindings.push(Self::from(rhs));
			}
			Self::Source(_) => unimplemented!(),
		}
		self
	}
}
