# quickscope

[![Crates.io](https://img.shields.io/crates/v/quickscope)](https://crates.io/crates/quickscope)
[![Docs.rs](https://docs.rs/quickscope/badge.svg)](https://docs.rs/quickscope)

Multi-layer `HashMap` and `HashSet` implementations for performant representation of variable scopes.

## Installation

Please use [cargo-edit](https://crates.io/crates/cargo-edit) to always add the latest version of this library:

```cmd
cargo add quickscope
```

## Purpose

This crate contains two data structures, `ScopeMap` and `ScopeSet`, for representing variable scopes and, in the case of `ScopeMap`, associated variable values. Access operations are such that variables in higher (i.e. more specific) scopes override variables in lower (i.e. more general) scopes.

## Rationale (i.e. "why _another_ one of these?")

I know I'm not the first one to do this! There are other, equally usable crates for the same purpose (see: [hash-chain](https://crates.io/crates/hash-chain), [chainmap](https://crates.io/crates/chainmap), [chain-map](https://crates.io/crates/chain-map)), but their implementations are such that lookups are O(n) worst-case with respect to layer count. I found that this didn't suit my needs.

This crate is optimized so that lookups are O(1) on average with respect to layer count. The trade-off is that popping layers is an O(n) operation with respect to the number of keys stored in the removed layer. I think this is an acceptable compromise in use cases requiring fast lookups across a large number of layers.

## Example

```rust
let mut vars = ScopeMap::new();

// Define two variables in main scope
vars.define("a", 1);
vars.define("b", 2);

// Add a child scope
vars.push_layer();

// Override value of `a`
vars.define("a", 3);

assert_eq!(Some(&3), vars.get("a"));
assert_eq!(Some(&2), vars.get("b"));

// Remove child scope
vars.pop_layer();

// Value of `a` is no longer overridden
assert_eq!(Some(&1), vars.get("a"));
```

## Why is it missing X feature from the regular HashMap/HashSet?

`ScopeMap` and `ScopeSet` are optimized for representing variable scopes. As such, they are missing many of the typical methods found in more general-purpose HashMap/HashSet implementations. If there's a feature that you feel should be added, feel free to submit a PR or post an issue about it.

## License

This library is distributed under the MIT License. See [LICENSE](./LICENSE) for details.