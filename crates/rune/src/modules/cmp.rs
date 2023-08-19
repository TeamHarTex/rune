//! The `std::cmp` module.

use core::cmp::Ordering;

use crate as rune;
use crate::runtime::{Protocol, Value, VmResult};
use crate::{ContextError, Module};

/// Construct the `std::cmp` module.
pub fn module() -> Result<Module, ContextError> {
    let mut m = Module::with_crate_item("std", ["cmp"]);

    {
        let ty = m.ty::<Ordering>()?.docs([
            "An `Ordering` is the result of a comparison between two values.",
            "",
            "# Examples",
            "",
            "```",
            "use std::cmp::Ordering;",
            "",
            "let result = 1.cmp(2);",
            "assert_eq!(Ordering::Less, result);",
            "",
            "let result = 1.cmp(1);",
            "assert_eq!(Ordering::Equal, result);",
            "",
            "let result = 2.cmp(1);",
            "assert_eq!(Ordering::Greater, result);",
            "```",
        ]);

        let mut ty = ty.make_enum(&["Less", "Equal", "Greater"])?;

        ty.variant_mut(0)?
            .make_empty()?
            .constructor(|| Ordering::Less)?
            .docs(["An ordering where a compared value is less than another."]);

        ty.variant_mut(1)?
            .make_empty()?
            .constructor(|| Ordering::Equal)?
            .docs(["An ordering where a compared value is equal to another."]);

        ty.variant_mut(2)?
            .make_empty()?
            .constructor(|| Ordering::Greater)?
            .docs(["An ordering where a compared value is greater than another."]);
    }

    m.associated_function(Protocol::PARTIAL_EQ, |lhs: Ordering, rhs: Ordering| {
        lhs == rhs
    })?;
    m.associated_function(Protocol::EQ, |lhs: Ordering, rhs: Ordering| lhs == rhs)?;
    m.function_meta(min)?;
    m.function_meta(max)?;
    Ok(m)
}

/// Compares and returns the maximum of two values.
///
/// Returns the second argument if the comparison determines them to be equal.
///
/// Internally uses the [`CMP`] protocol.
///
/// # Examples
///
/// ```rune
/// use std::cmp::max;
///
/// assert_eq!(max(1, 2), 2);
/// assert_eq!(max(2, 2), 2);
/// ```
#[rune::function]
fn max(v1: Value, v2: Value) -> VmResult<Value> {
    VmResult::Ok(match vm_try!(Value::cmp(&v1, &v2)) {
        Ordering::Less | Ordering::Equal => v2,
        Ordering::Greater => v1,
    })
}

/// Compares and returns the minimum of two values.
///
/// Returns the first argument if the comparison determines them to be equal.
///
/// Internally uses the [`CMP`] protocol.
///
/// # Examples
///
/// ```rune
/// use std::cmp::min;
///
/// assert_eq!(min(1, 2), 1);
/// assert_eq!(min(2, 2), 2);
/// ```
#[rune::function]
fn min(v1: Value, v2: Value) -> VmResult<Value> {
    VmResult::Ok(match vm_try!(Value::cmp(&v1, &v2)) {
        Ordering::Less | Ordering::Equal => v1,
        Ordering::Greater => v2,
    })
}
