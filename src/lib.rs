//! enum type utilities. Mostly macros.

/// An attribute for generating method for enum type.
///
/// ```rust
/// #[enums::name]
/// enum Enum {
///     This,
///     That,
/// }
///
/// assert_eq!(Enum::This.name(), "This");
/// assert_eq!(Enum::That.name(), "That");
/// ```
///
/// That example will generate:
///
/// ```rust
/// enum Enum {
///     This,
///     That,
/// }
///
/// impl Enum {
///     const fn name(&self) -> &'static str {
///         match self {
///             Enum::This => "this",
///             Enum::That => "that",
///         }
///     }
/// }
/// ```
///
/// Case conversion using heck.
///
/// ```rust
/// #[enums::name(case = "kebab")]
/// enum Kebab {
///     ThisIs,
///     ThatIs,
/// }
///
/// assert_eq!(Kebab::ThisIs.name(), "this-is");
/// assert_eq!(Kebab::ThatIs.name(), "that-is");
/// ```
///
/// Back conversion.
///
/// ```
/// #[derive(Debug, Eq, PartialEq)]
/// #[enums::name(case = "kebab")]
/// enum Kebab {
///     ThisIs,
///     ThatIs,
/// }
///
/// assert_eq!("this-is".parse(), Ok(Kebab::ThisIs));
/// assert_eq!("that-is".parse(), Ok(Kebab::ThatIs));
///
/// #[derive(Debug, Eq, PartialEq)]
/// #[enums::name(case = "lower-camel")]
/// enum LowerCamel {
///     ThisIs,
///     ThatIs,
/// }
///
/// assert_eq!("thisIs".parse(), Ok(LowerCamel::ThisIs));
/// assert_eq!("thatIs".parse(), Ok(LowerCamel::ThatIs));
/// ```
///
/// Available case conversion:
///  - kebab
///  - snake
///  - title
///  - train
///  - lower-camel
///  - upper-camel
///  - shouty-kebab
///  - shouty-camel
pub use enums_macros::name;
