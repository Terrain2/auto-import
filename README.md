# magic-import

Improved version of [auto-import](https://crates.io/crates/auto-import)

The ``magic_import::magic!();`` macro expands to whatever
``use`` statements you need to make the rest of the code compile.
It is not intelligent enough to figure out *which is the correct import*
in cases of ambiguity, but it does have some rules to get good imports,
whereas ``auto-import`` would just import the first available option.
If none of the rules could find out which is the best import, it will
pick one at random

The ``stable`` feature flag will work on stable, but only once per crate.
That's the behaviour from ``auto-import``, but ``magic-import``'s default
is to work once per *file*, and that uses the ``proc_macro_span`` unstable
feature. If you want to use this more than once per crate, but also want to
support stable toolchain, consider [nightly-crimes](https://crates.io/crates/nightly-crimes),
a really cool package that might help you with that.

https://twitter.com/Terrain222/status/1528076303890624515

## Example

```rust
// it even resolves macros and prelude items fine
#![no_implicit_prelude]

// magic_import does not try to resolve which crates to import
// these would normally be included by prelude
extern crate magic_import;
extern crate std;
// extern crate core; // for prefer_core

magic_import::magic!();

fn main() {
    let _ = BTreeMap::<File, PathBuf>::new();
    if let Ok(i) = i32::from_str("123") {
        print!("{i}");
    } else {
        unreachable!();
    }
    std::io::stdout().write_all(b"!\n").unwrap();
}
```

```
$ cargo run
   Compiling magic-import v0.2.0
   Injecting std::print
   Injecting std::fs::File
   Injecting std::collections::BTreeMap
   Injecting std::io::Write
   Injecting std::unreachable
   Injecting std::result::Result::Ok
   Injecting std::str::FromStr
   Injecting std::path::PathBuf
    Finished dev [unoptimized + debuginfo] target(s) in 0.73s
     Running `target\debug\examples\example.exe`
123!
```

You can also specify the ``prefer_core`` feature flag to indicate that ``core::`` should be used over ``std::``, instead of the other way around:

```
$ cargo run
   Compiling magic-import v0.2.0
   Injecting std::collections::BTreeMap
   Injecting core::result::Result::Ok
   Injecting std::io::Write
   Injecting std::path::PathBuf
   Injecting std::fs::File
   Injecting std::str::FromStr
   Injecting std::print
   Injecting core::unreachable
    Finished dev [unoptimized + debuginfo] target(s) in 0.65s
     Running `target\debug\examples\example.exe`
123!
```

Note that it may still import from ``std::`` if there is no suitable ``core::`` import