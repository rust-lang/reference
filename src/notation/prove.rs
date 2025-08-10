/// Given a set of antecedent type system predicates, assert that a
/// set of consequent predicates are (`=>`) or are not (`?=>`) proven
/// by rustc.
///
/// See the documentation in `notation.tools.prove`.
#[rustfmt::skip]
macro_rules! prove {
    ($(for<$($ls:lifetime),* $(,)? $($($ps:ident),+ $(,)?)?>)?
     { $($antecedents:tt)* } => { $($consequents:tt)* }
    ) => {const _: () = {
        trait _Assert<T: ?Sized> { fn _f(); }
        impl<$($($ls,)* $($($ps),+)?)?> _Assert<($($($($ps),+,)?)?)>
            for () where $($antecedents)*
        { fn _f() where $($consequents)* {} }
    };};
    ($(for<$($ls:lifetime),* $(,)? $($($ps:ident),+ $(,)?)?>)?
     { $($($antecedents:tt)+)? }
     ?=> { $($consequents:tt)* }
    ) => {const _: () = {
        struct _W<T: ?Sized>(T); struct _True; struct _False;
        trait _Fallback { fn f(&self) -> _False { _False } }
        impl<T: ?Sized> _Fallback for _W<T> {}
        trait _Test { fn f(&self) -> _True { _True } }
        impl<$($($ls,)* $($($ps),+)?)?> _Test
            for &_W<($($($($ps),+,)?)?)>
        where $($($antecedents)+,)? $($consequents)* {}
        fn _f<$($($ls,)* $($($ps),+)?)?>(
            x: &&_W<($($($($ps),+,)?)?)>
        ) -> _False where $($($antecedents)+)?
        { x.f() }
    };};
}
