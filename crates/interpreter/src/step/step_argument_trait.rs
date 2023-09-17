use super::StepArgumentsError;
use shared::Step;

/// A StepArgument can turn into a [`Step`]
pub trait StepArgumentTrait {
    fn to_step(&self) -> Result<Box<dyn Step>, StepArgumentsError>;
}
