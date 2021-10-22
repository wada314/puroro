# v 0.2.0
- Changed the message trait's getter methods entirely.
    - non-repeated field `foo` generates `foo(&self) -> T`, `foo_opt(&self) -> Option<T>`, and `has_foo(&self) -> bool`.
    - repeated field interfaces are not changed.
- Changed the message trait's string & bytes type from an associated type to just a `&str` and `&[u8]`.
    - As an effect of this, the generated code size is shrinked ~20%.

# v 0.1.0
First publish.