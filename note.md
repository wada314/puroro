
## `PersonOwnedFields`
- `String`
- `i32`
- `Vec<String>`
- `Vec<i32>`
- `Option<Box<Vehicle>>`
- `Vehicle`
- `Vec<Vehicle>`

## `OwnedMessageImpl`
- `Option<&str>`
- `Option<i32>`
- `&[String]`
- `&[i32]`
- `Option<&Vehicle>`
- `&[Vehicle]`

## `MessageImpl`


## `Person`
- `&str`
- `i32`
- `&[impl AsRef<str>]`
- `impl Iterator<Item=impl AsRef<str>>`
- `&[i32]`
- `impl Iterator<Item=i32>`
- `&Vehicle<impl AsMessageImplRef>`
- `&[Vehicle]`
- `impl Iterator<Item=Vehicle<impl AsMessageImplRef>>`

