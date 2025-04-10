r[type.trait-object]
# Trait objects

r[type.trait-object.syntax]
```grammar,types
TraitObjectType -> `dyn`? TypeParamBounds

TraitObjectTypeOneBound -> `dyn`? TraitBound
```

r[type.trait-object.intro]
A *trait object* is an opaque value of another type that implements a set of
traits. The set of traits is made up of a [dyn compatible] *base trait* plus any
number of [auto traits].

r[type.trait-object.impls]
Trait objects implement the base trait, its auto traits, and any [supertraits]
of the base trait.

r[type.trait-object.name]
Trait objects are written as the keyword `dyn` followed by a set of trait
bounds, but with the following restrictions on the trait bounds.

r[type.trait-object.constraint]
There may not be more than one non-auto trait, no more than one
lifetime, and opt-out bounds (e.g. `?Sized`) are not allowed. Furthermore,
paths to traits may be parenthesized.

For example, given a trait `Trait`, the following are all trait objects:

* `dyn Trait`
* `dyn Trait + Send`
* `dyn Trait + Send + Sync`
* `dyn Trait + 'static`
* `dyn Trait + Send + 'static`
* `dyn Trait +`
* `dyn 'static + Trait`.
* `dyn (Trait)`

r[type.trait-object.syntax-edition2021]
> [!EDITION-2021]
> Before the 2021 edition, the `dyn` keyword may be omitted.

r[type.trait-object.syntax-edition2018]
> [!EDITION-2018]
> In the 2015 edition, if the first bound of the trait object is a path that starts with `::`, then the `dyn` will be treated as a part of the path. The first path can be put in parenthesis to get around this. As such, if you want a trait object with the trait `::your_module::Trait`, you should write it as `dyn (::your_module::Trait)`.
>
> Beginning in the 2018 edition, `dyn` is a true keyword and is not allowed in paths, so the parentheses are not necessary.

r[type.trait-object.alias]
Two trait object types alias each other if the base traits alias each other and
if the sets of auto traits are the same and the lifetime bounds are the same.
For example, `dyn Trait + Send + UnwindSafe` is the same as
`dyn Trait + UnwindSafe + Send`.

r[type.trait-object.unsized]
Due to the opaqueness of which concrete type the value is of, trait objects are
[dynamically sized types]. Like all
<abbr title="dynamically sized types">DSTs</abbr>, trait objects are used
behind some type of pointer; for example `&dyn SomeTrait` or
`Box<dyn SomeTrait>`. Each instance of a pointer to a trait object includes:

 - a pointer to an instance of a type `T` that implements `SomeTrait`
 - a _virtual method table_, often just called a _vtable_, which contains, for
   each method of `SomeTrait` and its [supertraits] that `T` implements, a
   pointer to `T`'s implementation (i.e. a function pointer).

The purpose of trait objects is to permit "late binding" of methods. Calling a
method on a trait object results in virtual dispatch at runtime: that is, a
function pointer is loaded from the trait object vtable and invoked indirectly.
The actual implementation for each vtable entry can vary on an object-by-object
basis.

An example of a trait object:

```rust
trait Printable {
    fn stringify(&self) -> String;
}

impl Printable for i32 {
    fn stringify(&self) -> String { self.to_string() }
}

fn print(a: Box<dyn Printable>) {
    println!("{}", a.stringify());
}

fn main() {
    print(Box::new(10) as Box<dyn Printable>);
}
```

In this example, the trait `Printable` occurs as a trait object in both the
type signature of `print`, and the cast expression in `main`.

r[type.trait-object.lifetime-bounds]
## Trait Object Lifetime Bounds

Since a trait object can contain references, the lifetimes of those references
need to be expressed as part of the trait object. This lifetime is written as
`Trait + 'a`. There are [defaults] that allow this lifetime to usually be
inferred with a sensible choice.

[auto traits]: ../special-types-and-traits.md#auto-traits
[defaults]: ../lifetime-elision.md#default-trait-object-lifetimes
[dyn compatible]: ../items/traits.md#dyn-compatibility
[dynamically sized types]: ../dynamically-sized-types.md
[supertraits]: ../items/traits.md#supertraits
