# Trait objects

> **<sup>Syntax</sup>**\
> _TraitObjectType_ :\
> &nbsp;&nbsp; `dyn`<sup>?</sup> [_TypeParamBounds_]
>
> _TraitObjectTypeOneBound_ :\
> &nbsp;&nbsp; `dyn`<sup>?</sup> [_TraitBound_]

A *trait object* is a powerful concept allowing you to work with values of
different types as long as they implement a specified set of traits. This set
consists of a primary, [object safe] *base trait*, and it can be extended with
any number of [auto traits].

A trait object represents a value of one type that adheres to the traits
specified in its definition. It not only implements the base trait but also
encompasses any associated [auto traits] and [supertraits] of the base trait.

## Syntax and Examples

The syntax for trait objects is concise:

```rust
dyn? TypeParamBounds
```

Here, `dyn` is the keyword, and `TypeParamBounds` represents the set of traits
the object implements. While expressing trait bounds, some rules apply: only the
first trait can be non-auto, there should be at most one lifetime, and opt-out
bounds like `?Sized` are not allowed. Parentheses can be used to specify paths
to traits.

For example, given a trait `Trait`, here are some trait objects:

- `dyn Trait`
- `dyn Trait + Send`
- `dyn Trait + Send + Sync`
- `dyn Trait + 'static`
- `dyn Trait + Send + 'static`
- `dyn Trait +`
- `dyn 'static + Trait`
- `dyn (Trait)`

> **Edition Differences**: Before the 2021 edition, `dyn` could be omitted. However,
> for clarity, it is recommended to always use `dyn` unless your codebase supports
> Rust 1.26 or lower.

> In the 2015 edition, if the first bound of the trait object is a path starting
> with `::`, then the `dyn` is considered part of the path. In such cases, use
> parentheses to avoid issues.

> Starting from the 2018 edition, `dyn` is a true keyword, and parentheses are not
> necessary.

Two trait object types are considered identical if their base traits match, auto
traits are the same, and the lifetime bounds are identical. For instance, 
`dyn Trait + Send + UnwindSafe` is equivalent to `dyn Trait + UnwindSafe + Send`.

## Trait Objects in Action

Due to the opaque nature of trait objects, they are considered
[dynamically sized types (DSTs)]. They are typically used with pointers like
`&dyn SomeTrait` or `Box<dyn SomeTrait>`. A pointer to a trait object includes a
pointer to an instance of the implementing type and a _virtual method table_
(vtable)containing function pointers for each method the type implements.

The primary purpose of trait objects is to enable "late binding" of methods.
Invoking a method on a trait object results in virtual dispatch at runtime.
The correct function implementation is determined by loading a function pointer
from the trait object's vtable.

## Example Code

Let's consider some examples about trait objects:

### 1. **Trait Object with Multiple Traits**

```rust
trait Shape {
    fn area(&self) -> f64;
}

trait Drawable {
    fn draw(&self);
}

trait ShapeAndDrawable: Shape + Drawable {}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle");
    }
}

impl ShapeAndDrawable for Circle {}

fn process_shapes(shapes: Vec<Box<dyn ShapeAndDrawable>>) {
    for shape in shapes {
        println!("Area: {:.2}", shape.area());
        shape.draw();
    }
}

fn main() {
    let circle = Circle { radius: 3.0 };

    let circle_trait_object: Box<dyn ShapeAndDrawable> = Box::new(circle);

    process_shapes(vec![circle_trait_object]);
}

// Output

// Area: 28.27
// Drawing a circle
```

This example defines two traits, `Shape` and `Drawable`, representing geometric shapes
and drawable objects. The `Circle` struct implements both traits. The `process_shapes`
function accepts a vector of trait objects that combine `Shape` and `Drawable`. In `main`,
we create a `Circle` instance, convert it to a trait object, and process it using the
generic function.

### 2. **Dynamic Dispatch with Generics**

```rust
trait Processor<T> {
    fn process(&self, data: T);
}

struct StringProcessor;

impl Processor<&str> for StringProcessor {
    fn process(&self, data: &str) {
        println!("Processing string: {}", data);
    }
}

fn process_data<T>(processor: Box<dyn Processor<T>>, data: T) {
    processor.process(data);
}

fn main() {
    let string_processor = StringProcessor;

    let dynamic_processor: Box<dyn Processor<&str>> = Box::new(string_processor);

    process_data(dynamic_processor, "Hello, Trait Objects!");
}

// Output

// Processing string: Hello, Trait Objects!
```

This example demonstrates dynamic dispatch with generics. The `Processor` trait has a
generic method, and we implement it for a specific type (`&str`). In `main`, we create
a `StringProcessor` instance, convert it to a trait object with dynamic dispatch, and
use it to process a string.

### 3. **Trait Objects with Default Implementations**

```rust
trait Logger {
    fn log(&self, message: &str) {
        println!("Default Log: {}", message);
    }
}

struct AppLogger;

impl Logger for AppLogger {
    fn log(&self, message: &str) {
        println!("App Log: {}", message);
    }
}

fn main() {
    let app_logger = AppLogger;

    let logger_trait_object: Box<dyn Logger> = Box::new(app_logger);

    logger_trait_object.log("Custom Log Message");
}

// Output

// App Log: Custom Log Message
```

In this example, the `Logger` trait has a default implementation for the `log` method.
The `AppLogger` struct implements this trait, and in `main`, we create an instance,
convert it to a trait object, and use the default implementation to log a custom message.

### 4. **Trait Objects and Associated Types**

```rust
trait Transformer {
    type Output;
    fn transform(&self) -> Self::Output;
}

struct UppercaseString(String);

impl Transformer for UppercaseString {
    type Output = String;
    fn transform(&self) -> Self::Output {
        self.0.to_uppercase()
    }
}

fn main() {
    let uppercase_string = UppercaseString("hello".to_string());

    let transformer_trait_object: Box<dyn Transformer<Output = String>> =
        Box::new(uppercase_string);

    let result = transformer_trait_object.transform();
    println!("Transformed: {}", result);
}

// Output

// Transformed: HELLO
```

This example introduces a trait with an associated type. The `Transformer` trait has a
method `transform` with an associated type `Output`. The `UppercaseString` struct
implements this trait, and in `main`, we create an instance, convert it to a trait
object, and use it to transform a string.

These examples cover a range of scenarios, from combining multiple traits to dealing
with generics, default implementations, and associated types, showcasing the flexibility
and power of trait objects in Rust.

## Trait Object Lifetime Bounds

Since a trait object can contain references, the lifetimes of those references
need to be expressed as part of the trait object. This lifetime is written as
`Trait + 'a`. There are [defaults] that allow this lifetime to usually be
inferred with a sensible choice.

[_TraitBound_]: ../trait-bounds.md
[_TypeParamBounds_]: ../trait-bounds.md
[auto traits]: ../special-types-and-traits.md#auto-traits
[defaults]: ../lifetime-elision.md#default-trait-object-lifetimes
[dynamically sized types (DSTs)]: ../dynamically-sized-types.md
[object safe]: ../items/traits.md#object-safety
[supertraits]: ../items/traits.md#supertraits
