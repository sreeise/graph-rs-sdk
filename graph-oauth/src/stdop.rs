use std::any::Any;
use std::any::TypeId;
use std::option;

/// Methods for checking type ids of generic types.
/// This is mostly used internally to cut down on matching Option types.
trait GenericTypeId {
    fn type_id_ref_of<T: std::any::Any>(_: &T) -> std::any::TypeId {
        TypeId::of::<T>()
    }

    fn type_id_of<T: std::any::Any>(_: T) -> std::any::TypeId {
        TypeId::of::<T>()
    }
}

///!Enforces that a generic type is wrapped in an Option or &Option
pub trait OptionType {}
impl<T> OptionType for Option<T> where T: std::marker::Sized {}
impl<T> OptionType for &Option<T> where T: std::marker::Sized {}

/// Useful methods for the Rust standard library Option.
/// This is mostly used internally to cut down on matching Option types.
#[derive(Debug, Eq, PartialEq)]
pub struct StdOp;

impl GenericTypeId for StdOp {}

impl StdOp {
    /// Converts a Option<&str> to an Option<String>
    /// Returns None if the given Option is None.
    pub fn from(t: option::Option<&str>) -> option::Option<String> {
        t.map(std::string::ToString::to_string).or(None)
    }

    pub fn from_u32(t: option::Option<u32>) -> u32 {
        t.unwrap_or(0)
    }

    pub fn convert_to_string(t: option::Option<String>) -> String {
        t.map(|t| t.to_string()).unwrap_or_default()
    }

    pub fn op_ref_to_string(t: option::Option<&String>) -> String {
        t.map(std::string::ToString::to_string).unwrap_or_default()
    }

    /// Evaluates whether T in &Option<T> is the same type as U in &Option<U>
    /// Both types must be wrapped in an &Option (Must be a reference).
    /// Cannot be None type.
    #[allow(dead_code)]
    pub fn is_op_ref_type<T: Any, U: Any>(t: &T, u: &U) -> bool
    where
        T: Sized + OptionType,
        U: Sized + OptionType,
    {
        StdOp::type_id_ref_of(t) == StdOp::type_id_ref_of(u)
    }

    /// Evaluates whether T in Option<T> is the same type as U in Option<U>
    /// Both types must be wrapped in an Option (Cannot be a reference).
    /// Cannot be None type.
    #[allow(dead_code)]
    pub fn is_op_type<T: Any, U: Any>(t: T, u: U) -> bool
    where
        T: Sized + OptionType,
        U: Sized + OptionType,
    {
        StdOp::type_id_of(t) == StdOp::type_id_of(u)
    }

    // Evaluates whether the &T is equal to &Option<T> where T: String
    #[allow(dead_code)]
    pub fn is_op_string_ref<T: ?Sized + Any>(_s: &T) -> bool {
        TypeId::of::<&Option<String>>() == TypeId::of::<T>()
    }

    // Evaluates whether the &T is equal to &Option<T> where T: &str
    #[allow(dead_code)]
    pub fn is_op_str<T: ?Sized + Any>(_s: &T) -> bool {
        TypeId::of::<Option<&str>>() == TypeId::of::<T>()
    }

    // Evaluates whether the &T is equal to &Option<T> where T: String
    #[allow(dead_code)]
    pub fn is_op_string<T: Any>(_s: T) -> bool
    where
        T: Sized,
    {
        TypeId::of::<Option<String>>() == TypeId::of::<T>()
    }
}
