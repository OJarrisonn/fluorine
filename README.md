# Fluorine

Fluorine is a Rust library that brings reactive state into Rust.

Fluorine provides 3 primitives:

- `Value`: a simple value whose changes trigger reactive computations
- `Expr`: an expression that may depend on the state of a `Value` and other `Expr` and the value is recalculated whenever needed
- : an piece of code that depends may on `Value` and `Expr` and needs to be recomputated whenever they change

## States

Reactive data can be called _clean_ or _dirty_. A clean reactive data can be used as is. Dirty reactive data can't be used, as its value may be outdated because some data which it depends on has changed.

## State Changes

A `Value` may have its content overwwriten by the method `write`. Whenever it happens, every single reactive data that depends on it needs to me marked dirty (unless its value is still the same). When an `Expr` is marked dirty, all data (ie other `Expr`) that depends on it is also marked dirty and so on so forth.

Reactive data keep tracks on which reactive data is depending on it to propagate a signal warning when changes have been done

## Reactor

A reactor is a kind of central state manager, which keep tracks 