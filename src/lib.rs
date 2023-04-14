/*! Comparison of different implementations of typestates.

There are three primary ways of implementing typestates:
- individual types,
- generic type with phantom types,
- generic type with real types.

In this library we compare the three approaches by implementing the following system.
- An entity gets created with an ID.
- We can add a key `a` or `b` to the entity, but not both.
- After we've added `a` or `b`, we can add `c`.
- The entity is now ready to be used, i.e. printed.

# Usage

# Examples

*/

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod individual_types;
pub mod phantom_types;
pub mod real_types;
