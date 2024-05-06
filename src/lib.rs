//! Wrappers around Atomics that always use `Ordering::Relaxed`
//!
//! ```
//! // instead of:
//! use std::sync::atomic::{AtomicBool, Ordering};
//! let atomic = AtomicBool::new(false);
//! atomic.store(true, Ordering::Relaxed);
//! assert_eq!(atomic.load(Ordering::Relaxed), true);
//!
//! // you can do:
//! use relaxed::RelaxedBool;
//! let atomic = RelaxedBool::new(false);
//! atomic.set(true);
//! assert_eq!(atomic.get(), true);
//! ```

#![no_std]
#![warn(missing_docs)]

use core::fmt::{Debug, Formatter, Display};
use core::sync::atomic::{
    AtomicBool, AtomicI16, AtomicI32, AtomicI8, AtomicU16, AtomicU32, AtomicU8, Ordering,
};

use atomic_float::AtomicF32;

macro_rules! impls {
    ($name:ident: $atomic:ident, $inner:ty) => {
        /// A Wrapper around a atomic value, that always uses `Ordering::Relaxed` for access.
        #[derive(Default)]
        #[repr(transparent)]
        pub struct $name($atomic);

        impl $name {
            /// Create a new value.
            #[inline(always)]
            pub const fn new(val: $inner) -> Self {
                $name($atomic::new(val))
            }

            /// Performs an atomic load with relaxed ordering.
            #[inline(always)]
            pub fn get(&self) -> $inner {
                self.0.load(Ordering::Relaxed)
            }
            /// Performs an atomic store with relaxed ordering.
            #[inline(always)]
            pub fn set(&self, val: $inner) {
                self.0.store(val, Ordering::Relaxed)
            }
            /// Calls `f` with the current value and stores the value returned by `f`.
            /// **The update is not atomic.**
            #[inline(always)]
            pub fn update(&self, f: impl FnOnce($inner) -> $inner) {
                self.set(f(self.get()))
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl From<$inner> for $name {
            fn from(val: $inner) -> Self {
                $name::new(val)
            }
        }

        #[cfg(feature="serde")]
        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                // Matches the atomic ordering used in libcore for the Debug impl
                self.0.load(Ordering::Relaxed).serialize(serializer)
            }
        }

        #[cfg(feature="serde")]
        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                serde::Deserialize::deserialize(deserializer).map(Self::new)
            }
        }
    };
}

impls!(RelaxedBool: AtomicBool, bool);

impls!(RelaxedU8: AtomicU8, u8);
impls!(RelaxedU16: AtomicU16, u16);
impls!(RelaxedU32: AtomicU32, u32);

impls!(RelaxedI8: AtomicI8, i8);
impls!(RelaxedI16: AtomicI16, i16);
impls!(RelaxedI32: AtomicI32, i32);

impls!(RelaxedF32: AtomicF32, f32);

impl RelaxedBool {
    /// Performs a logical "not" operation on the current value, and sets the new value to the result.
    /// Returns the previous value.
    pub fn fetch_not(&self) -> bool {
        self.0.fetch_not(Ordering::Relaxed)
    }
}
