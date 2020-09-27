use crate::collections::HashMap;
use crate::shared::Internal;
use crate::Spanned;
use runestick::Span;
use thiserror::Error;

/// A hierarchy of constant scopes.
pub(crate) struct Scopes<T> {
    scopes: Vec<Scope<T>>,
}

impl<T> Scopes<T> {
    /// Get a value out of the scope.
    pub(crate) fn get<'a>(&'a self, name: &str) -> Option<&'a T> {
        for scope in self.scopes.iter().rev() {
            if let Some(current) = scope.locals.get(name) {
                return Some(current);
            }
        }

        None
    }

    /// Clear the current scope.
    pub(crate) fn clear_current<S>(&mut self, spanned: S) -> Result<(), Internal>
    where
        S: Spanned,
    {
        let last = self
            .scopes
            .last_mut()
            .ok_or_else(|| Internal::new(spanned, "expected at least one scope"))?;

        last.locals.clear();
        Ok(())
    }

    /// Declare a value in the scope.
    pub(crate) fn decl<S>(&mut self, name: &str, value: T, spanned: S) -> Result<(), Internal>
    where
        S: Spanned,
    {
        let last = self
            .last_mut()
            .ok_or_else(|| Internal::new(spanned, "expected at least one scope"))?;

        last.locals.insert(name.to_owned(), value);
        Ok(())
    }

    /// Get the given variable.
    pub(crate) fn get_name<'a, S>(&'a self, name: &str, spanned: S) -> Result<&'a T, ScopeError>
    where
        S: Spanned,
    {
        for scope in self.scopes.iter().rev() {
            if let Some(current) = scope.locals.get(name) {
                return Ok(current);
            }
        }

        Err(ScopeError::new(
            spanned,
            ScopeErrorKind::MissingLocal { name: name.into() },
        ))
    }

    /// Get the given variable as mutable.
    pub(crate) fn get_name_mut<'a, S>(
        &'a mut self,
        name: &str,
        spanned: S,
    ) -> Result<&'a mut T, ScopeError>
    where
        S: Spanned,
    {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(current) = scope.locals.get_mut(name) {
                return Ok(current);
            }
        }

        Err(ScopeError::new(
            spanned,
            ScopeErrorKind::MissingLocal { name: name.into() },
        ))
    }

    /// Push a scope and return the guard associated with the scope.
    pub(crate) fn push(&mut self) -> ScopeGuard {
        let length = self.scopes.len();
        self.scopes.push(Scope::default());
        ScopeGuard { length }
    }

    pub(crate) fn pop<S>(&mut self, spanned: S, guard: ScopeGuard) -> Result<(), Internal>
    where
        S: Spanned,
    {
        if self.scopes.pop().is_none() {
            return Err(Internal::new(spanned, "expected at least one scope to pop"));
        }

        if self.scopes.len() != guard.length {
            return Err(Internal::new(spanned, "scope length mismatch"));
        }

        Ok(())
    }

    /// Get the last scope mutably.
    pub(crate) fn last_mut(&mut self) -> Option<&mut Scope<T>> {
        self.scopes.last_mut()
    }
}

impl<T> Default for Scopes<T> {
    fn default() -> Self {
        Self {
            scopes: vec![Scope::default()],
        }
    }
}

#[repr(transparent)]
pub(crate) struct ScopeGuard {
    length: usize,
}

pub(crate) struct Scope<T> {
    /// Locals in the current scope.
    locals: HashMap<String, T>,
}

impl<T> Default for Scope<T> {
    fn default() -> Self {
        Self {
            locals: Default::default(),
        }
    }
}

error! {
    /// An error cause by issues with scoping.
    #[derive(Debug)]
    pub struct ScopeError {
        kind: ScopeErrorKind,
    }
}

/// The kind of the [ScopeError].
#[derive(Debug, Error)]
pub enum ScopeErrorKind {
    /// A local variable was missing.
    #[error("missing local {name}")]
    MissingLocal {
        /// The name that was missing.
        name: Box<str>,
    },
}
