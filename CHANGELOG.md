
# v 0.8.0
- Mostly internal code refactoring.

# v 0.7.0
- Refactored the code generator so that it generates `TokenStream` instead of `String`.

# v 0.6.0
- Re-worked the internal dependencies. I messed up a lot so starting a new version number...

# v 0.5.0
- Restarted again from the scratch.
- Regression. No message traits and builders anymore.
  - Planned to be implemented again, maybe...

# v 0.4.1
- Fixed the bug that [packed=...] option was ignored.

# v 0.4.0
- So many changes.
- The message struct does not have public fields anymore. Use the methods instead.
- Added getter and setter methods for the normal message structs and the bumpalo structs.
- Changed the internal implementation of the normal message struct and bumpalo struct.
These uses a `bitvec` to manage `optional` fields' presence bit.

# v 0.3.1
- Fixed that the compile does not pass in the latest rustc nightly (1.58).

# v 0.3.0
- Changed the serializer method to use heap memory
- Changed the builder's `append_my_i32_field()` methods for numerical types to accept `Into<i32>` types.

# v 0.2.0
- Changed the message trait's getter methods entirely.
    - non-repeated field `foo` generates `foo(&self) -> T`, `foo_opt(&self) -> Option<T>`, and `has_foo(&self) -> bool`.
    - repeated field interfaces are not changed.
- Changed the message trait's string & bytes type from an associated type to just a `&str` and `&[u8]`.
    - As an effect of this, the generated code size is shrinked ~20%.

# v 0.1.0
First publish.