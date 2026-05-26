r[not-unsafe]
# Undesirable behavior not considered `unsafe`

> [!NOTE]
> This section is a non-exhaustive description of undesirable safe behavior. By nature, it cannot encompass all possible undesired safe behavior and does not claim to do so. Any behavior not defined as [unsafe] or [undefined] is safe by definition.

Rust does not consider the following behaviors _[unsafe]_, though a programmer may (or should) find them undesirable, unexpected, or erroneous.

r[not-unsafe.deadlocks]
## Deadlocks and livelocks

Deadlocks occur when certain tasks cannot proceed executing as they are waiting on at least one other resource held by another task. In the simplest case, two tasks 1 and 2 will deadlock when task 1 holds lock A, task 2 holds lock B, task 1 is waiting to acquire lock B, and task 2 is waiting to acquire lock A. Since both are requesting the lock that the other is holding, none can make any progress and they are deadlocked.

<!-- no_run: this program intentionally deadlocks and therefore does not terminate -->

```rust,no_run
# use std::sync::{Arc, Mutex};
# use std::thread;
# use std::time::Duration;
#
# #[allow(unused)]
# fn main() {
let lock_a = Arc::new(Mutex::new(()));
let lock_b = Arc::new(Mutex::new(()));

let task1 = {
#   let lock_a = lock_a.clone();
#   let lock_b = lock_b.clone();
    thread::spawn(move || {
        let obtained_lock_a = lock_a.lock().unwrap();
        // Give process 2 some time to lock B.
        thread::sleep(Duration::from_secs(1));
        let obtained_lock_b = lock_b.lock().unwrap();
    })
};

let task2 = {
#   let lock_a = lock_a.clone();
#   let lock_b = lock_b.clone();
    thread::spawn(move || {
        let obtained_lock_b = lock_b.lock().unwrap();
        thread::sleep(Duration::from_secs(1));
        let obtained_lock_a = lock_a.lock().unwrap();
    })
};

// Neither of these calls will ever return due to the deadlock.
task1.join();
task2.join();
# }
```

In general, determining whether a program has deadlocked requires to solve the [Halting problem], which is impossible. Even though many instances of deadlocks can be detected automatically, doing so is not always practical. Regardless, some multitasking systems provide good deadlock detection capabilities.

r[not-unsafe.livelocks]
Livelocks are a related issue where no real progress is made in a group of tasks, yet they technically continue to run. For instance, using non-blocking synchronization primitives like spinlocks or atomic variables can quickly lead to livelocks. This is in opposition to deadlocks, where tasks are blocked on resource acquisition, which is relatively easy to discern. Therefore, livelocks are much harder to detect than deadlocks, but equally undesirable.

r[not-unsafe.resource-leaks]
## Leaks of memory and other resources

A leaking resource is one that is never freed, even when it is no longer needed. Among the resources commonly leaked are memory, file descriptors, and networking sockets. Resource leaks may cause the program to accumulate unused resources until certain constraints by the environment are reached, such as the system running out of memory (OOM). In this case, a variety of non-normal behavior can occur, such as program termination by the operating system, common for OOM conditions, or failure to acquire further resources of the given type, common for file descriptors and sockets.

> [!EXAMPLE]
> Using reference counting via [Rc](`std::rc::Rc`), it is possible to construct a reference cycle, where two data structures point at each other via `Rc`. Since both hold a reference to the other, the `Rc`s always have a reference count of at least 1, and their inner values are never dropped: a memory leak. Note that while implementing the following scenario requires some unsafe code in the standard library, the safety conditions are never violated, and the program is correct as far as `Rc`'s and `RefCell`'s unsafe code is concerned.
>
> ```rust
> use std::cell::RefCell;
> use std::rc::Rc;
>
> struct Node(Option<Rc<RefCell<Node>>>);
>
> impl Node {
>     fn link_to_other(&mut self, other: Rc<RefCell<Node>>) {
>         self.0 = Some(other);
>     }
> }
>
> fn main() {
>     let a = Rc::new(RefCell::new(Node(None)));
>     let b = Rc::new(RefCell::new(Node(None)));
>     a.borrow_mut().link_to_other(b.clone());
>     b.borrow_mut().link_to_other(a.clone());
>     // a and b are never deallocated, even when dropped here.
> }
> ```

r[not-unsafe.abort]
## Exiting without calling destructors

When [panicking], a program may abort execution; that is, exit immediately without further activities. There are several instances in which abort on panic can happen, such as a double panic or with a compiler option to always use the abort strategy.

Unlike unwinding, which runs [destructors] for each object in each unwound frame, aborting does not run any destructors in any frame. Any code relying on certain destructors to run might therefore not function correctly.

r[not-unsafe.aslr-bypass]
## Exposing randomized executable base addresses through pointer leaks

[Address space layout randomization] (ASLR) is a security mitigation which moves program segments to randomized (virtual) memory addresses each time the program is started. Without knowing the absolute positions of code or data in a program, this complicates binary exploitation.

> [!NOTE]
> Whether the executable has ASLR support depends on the target, compiler flags, linker options, and more. Embedded targets usually do not support ASLR at all, while it is commonplace for "normal" targets on modern operating systems. Rust does not guarantee ASLR support, though the compiler and target documentation should specify their ASLR support.

When a pointer is leaked that has a fixed known offset to a segment base address, such as a [function pointer], an attacker can derive the base address regardless of ASLR, usually bypassing its protections. A leak may occur in many ways, such as by displaying the pointer’s numeric value to the user.

r[not-unsafe.integer-overflow]
## Integer overflow

If a program contains arithmetic overflow, the programmer has made an error. There is a distinction between arithmetic overflow and _wrapping arithmetic_. The first is erroneous, while the second is intentional.

r[not-unsafe.integer-overflow.panic]
When the configuration option [debug_assertions] is enabled (for example, by enabling a non-optimized build), dynamic checks are inserted that `panic` on overflow.

r[not-unsafe.integer-overflow.silent-wrapping]
Other kinds of builds may result in [panic]s or silently wrapped values on overflow. In the case of implicitly-wrapped overflow, the results are well-defined (even if still considered erroneous) by using two's complement overflow conventions.

r[not-unsafe.integer-overflow.intentional-wrapping]
The [integer types] provide inherent methods to allow explicitly performing wrapping arithmetic. For example, [`i32::wrapping_add`] provides two's complement, wrapping addition for 32-bit signed integers.

The standard library also provides a [`Wrapping<T>`](`core::num::Wrapping<T>`) newtype which ensures all standard arithmetic operations for `T` have wrapping semantics.

> [!NOTE]
> See [RFC 560] for error conditions, rationale, and more details about integer overflow.

r[not-unsafe.logic]
## Logic errors

Safe code may impose extra logical constraints that can be checked at neither compile-time nor runtime. If a program breaks such a constraint, the behavior may be _unspecified_ but will not result in [undefined] behavior. This could include [panic]s, incorrect results, aborts, and non-termination. The behavior may also differ between runs, builds, or kinds of build.

> [!EXAMPLE]
> Implementing both [`Hash`](`core::hash::Hash`) and [`Eq`] requires that values considered equal have equal hashes. This promise is broken in the following code, and using `Wrapper` in types like [`HashMap`](`std::collections::HashMap`) will lead to unexpected behavior.
>
> <!-- no_run: exposing unpredictable HashMap behavior reliably (and in an understandable way) is hard -->
>
> ```rust,no_run
> use std::hash::{Hash, Hasher};
>
> #[derive(PartialEq, Eq)]
> struct Wrapper(i32);
>
> impl Hash for Wrapper {
>     fn hash<H>(&self, hasher: &mut H) where H: Hasher {
>         Hash::hash(&0i32, hasher);
>     }
> }
> ```
>
> Related, data structures like [`BinaryHeap`](`alloc::collections::binary_heap::BinaryHeap`), [`BTreeMap`](`alloc::collections::btree_map::BTreeMap`), [`BTreeSet`](`alloc::collections::btree_set::BTreeSet`), [`HashMap`](`std::collections::HashMap`), and [`HashSet`](`std::collections::HashSet`) describe constraints on the modification of their keys while they are in the data structure. Violating such constraints is not considered unsafe, yet the program is considered erroneous and its behavior unpredictable.

[RFC 560]: https://github.com/rust-lang/rfcs/blob/master/text/0560-integer-overflow.md
[unsafe]: safety.unsafe-ops
[undefined]: undefined
[panicking]: panic
[debug_assertions]: cfg.debug_assertions
[integer types]: type.numeric.int
[Halting problem]: https://en.wikipedia.org/wiki/Halting_problem
[Address space layout randomization]: https://en.wikipedia.org/wiki/Address_space_layout_randomization
[function pointer]: type.fn-pointer
