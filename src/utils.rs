// Shamelessly stolen from the Rust source.
// https://doc.rust-lang.org/src/core/internal_macros.rs.html#40-47

// Was modified to allow for generic type constraints in the impl, and to take a different form
// that (in my opinion) is more ergonomic to use. Also it won't break syntax highlighting now :P

/// Based on an existing implementation of "T op U" where both of T and U have `Copy`,
/// this macro will implement "&T op U", "T op &U", and "&T op &U".
macro_rules! impl_for_refs {
    ( impl<$g:ident: $b:ident> $imp:ident for $t:ty { fn $method:ident() -> $u:ty } ) => {
        impl<'a, $g: $b> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl<$g: $b> $imp<&$u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl<$g: $b> $imp<&$u> for &$t {
            type Output = <$t as $imp<$u>>::Output;

            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    }
}