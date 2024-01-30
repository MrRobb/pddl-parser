/// This module contains the definition of an actionable item. An actionable item can be an action or a durative action.
pub mod action;
/// This module contains the definition of a constant. A constant is a value that is not changed by the actions.
pub mod constant;
/// This module contains the definition of a domain. A domain is a set of actions, predicates, constants, and types.
pub mod domain;
/// This module contains the definition of a durative action. A durative action is a function that takes a set of parameters and returns a set of effects. It also has a duration.
pub mod durative_action;
/// This module contains the definition of an expression. An expression is a function that takes a set of parameters and returns a value.
pub mod expression;
/// This module contains the definition of a parameter. A parameter is a variable that is used in an action or a predicate.
pub mod parameter;
/// This module contains the definition of a predicate. A predicate is a function that takes a set of parameters and returns a boolean.
pub mod predicate;
/// This module contains the definition of a requirement. A requirement is a feature that is required by the domain.
pub mod requirement;
/// This module contains the definition of an action. An action is a function that takes a set of parameters and returns a set of effects.
pub mod simple_action;
/// This module contains the definition of a typed parameter. A typed parameter is a variable that is used in an action or a predicate. The type of the parameter is specified explicitly.
pub mod typed_parameter;
/// This module contains the definition of a typed predicate. A typed predicate is a function that takes a set of parameters and returns a boolean. The type of the parameters is specified explicitly.
pub mod typed_predicate;
/// This module contains the definition of a type definition. It defines the tree structure of the types.
pub mod typedef;
/// This module contains the definition of a type.
pub mod typing;
