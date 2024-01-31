/// The plan item module contains the definition of a plan item. A plan item is either an action or a durative action.
pub mod action;
/// The durative action module contains the definition of a durative action. A durative action is an action that has a duration. Durative actions are used in temporal planning.
pub mod durative_action;
/// The plan module contains the definition of a plan. A plan is a sequence of actions.
pub mod plan;
/// The action module contains the definition of an action. An action is a function that takes a set of parameters and returns a set of effects.
pub mod simple_action;
