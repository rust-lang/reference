# Appendix: Formal scoping model

This appendix presents a formal model of temporary scoping rules in Rust. It compares the current stable behavior with that of [Rust PR #146098] / [Reference PR #2051], and it compares different formalizations of *borrow scopes* and *extended subexpressions*.

The model is written in [Agda](https://github.com/agda/agda), a dependently typed functional programming language. This chapter is executable [literate Agda].

```agda
module scoping-model (FnCtxt ConstCtxt : Set) where

open import Data.Maybe using (Maybe; just; nothing)
open import Data.Nat using (ℕ)
open import Function using (_|>_)
open import Relation.Binary.PropositionalEquality using (_≡_; refl)
```

## Reading guide

> [!NOTE]
> For readers familiar with Rust but new to Agda, here is a brief guide to the notation used in this model.
>
> - `data ... where`: Similar to a Rust `enum`. Each line in the `where` block is a variant (constructor).
> - `record`: Similar to a Rust `struct`.
> - `{...}`: Implicit arguments. Agda infers these, similar to generic type inference or lifetime elision in Rust.
> - `_▷_`: Underscores in names indicate where arguments go (infix operators). `_▷_` is an infix constructor.
> - `refl`: Stands for "reflexivity"; used to prove that two values are equal.
> - Pattern matching: Definitions are often spread across multiple equations, similar to a `match` expression at the function level.

## Common definitions

### Syntax and contexts

To focus on scoping, this appendix models a simplified abstract syntax tree (AST).

- `Array` (`⟦…,●,…⟧`) represents all *extending expressions* (structs, tuples, arrays).
- `Indexing` (`●⟦…⟧`) represents all *place base contexts* (derefs, fields, indexes).

The *context* of a syntax-tree node uniquely identifies where in the source it appears. This is defined inductively.

```agda
data NodeKind : Set where
  Stmt : NodeKind
  Expr : NodeKind
  FnDecl : NodeKind
  ConstDecl : NodeKind
  Toplevel : NodeKind

-- Patterns are either extending patterns (like `ref x`) or not.
data Pat : Set where
  pat-ext : Pat
  pat-non : Pat

-- Each sub-node is the child of a particular kind of parent node.
data _SubnodeOf_ : NodeKind → NodeKind → Set where
  -- A function.
  node-fn : (fn : FnCtxt) → FnDecl SubnodeOf Toplevel
  -- A constant.
  node-const : (const : ConstCtxt) → ConstDecl SubnodeOf Toplevel
  -- The nᵗʰ statement in a block:
  ⦃…●…⦄ : (n : ℕ) → Stmt SubnodeOf Expr
  -- The body expression of a function:
  |…|● : Expr SubnodeOf FnDecl
  -- The body expression of a constant item:
  const…=●⨾ : Expr SubnodeOf ConstDecl
  -- The initializer expression of a `let` statement:
  let…=●⨾ : (pat : Pat) → Expr SubnodeOf Stmt
  -- The expression of an expression statement:
  ●⨾ : Expr SubnodeOf Stmt
  -- The tail expression of a block:
  ⦃…⨾●⦄ : Expr SubnodeOf Expr
  -- The nᵗʰ argument expression to a call:
  …⦅…,●,…⦆ : (n : ℕ) → Expr SubnodeOf Expr
  -- The nᵗʰ operand expression to an array constructor:
  ⟦…,●,…⟧ : (n : ℕ) → Expr SubnodeOf Expr
  -- The indexed array operand of an indexing operator expression:
  ●⟦…⟧ : Expr SubnodeOf Expr
  -- The operand of a borrow expression:
  &● : Expr SubnodeOf Expr

infixl 5 _▷_

data Context : NodeKind → Set where
  ctxt-top : Context Toplevel
  _▷_ : ∀ {k₁ k₂} → Context k₁ → k₂ SubnodeOf k₁ → Context k₂
```

This formulation allows constructs like `const X: &T = f(&●);` to be represented as a chain of contexts:

```agda
exampleContext : ConstCtxt → Context Expr
exampleContext const =
  ctxt-top ▷ node-const const ▷ const…=●⨾ ▷ …⦅…,●,…⦆ 0 ▷ &●
```

Helpers can also be defined to navigate the AST, such as finding the block containing a statement:

```agda
blockOf : Context Stmt → Context Expr
blockOf (e-block ▷ ⦃…●…⦄ _) = e-block
```

### Scopes

A *scope* is the syntactic context in which a temporary or variable lives. When execution leaves that context, the value is dropped.

> [!NOTE]
> In `rustc`, scopes are more granular (e.g., individual statements have remainder scopes). This model simplifies this by associating scopes directly with AST nodes.

```agda
record Scope : Set where
  constructor scope
  field
    {kind} : NodeKind
    ctxt : Context kind
```

#### Scope hierarchy

Scopes form a hierarchy. The `Hierarchy` module defines the ancestry relation, where `_≻_` is the parent relation and `_≻*_` is the reflexive transitive closure (ancestor relation).

```agda
module Hierarchy where
  open import Relation.Binary.Construct.Closure.ReflexiveTransitive
    using (Star)

  data _≻_ : Scope → Scope → Set where
    parent≻child : ∀ x {k} (n : k SubnodeOf Scope.kind x) →
      x ≻ scope (Scope.ctxt x ▷ n)

  _≻*_ : Scope → Scope → Set
  _≻*_ = Star _≻_
```

## The enclosing temporary scope

The temporary scope of non-extended temporaries is the closest ancestor temporary scope boundary, as defined by [destructors.scope.temporary.enclosing].

```agda
enclosingTempScope : Context Expr → Scope
-- Body expressions are temporary scope boundaries.
enclosingTempScope e-body@(_ ▷ |…|●) = scope e-body
enclosingTempScope e-body@(_ ▷ const…=●⨾) = scope e-body
-- Statements are temporary scope boundaries.
enclosingTempScope (s-let ▷ let…=●⨾ _) = scope s-let
enclosingTempScope (s-expr ▷ ●⨾) = scope s-expr
-- Block tails are temporary scope boundaries
enclosingTempScope e-tail@(_ ▷ ⦃…⨾●⦄) = scope e-tail
-- Our other contexts are not temporary scope boundaries.
enclosingTempScope (e-call ▷ …⦅…,●,…⦆ _) = enclosingTempScope e-call
enclosingTempScope (e-array ▷ ⟦…,●,…⟧ _) = enclosingTempScope e-array
enclosingTempScope (e-indexing ▷ ●⟦…⟧) = enclosingTempScope e-indexing
enclosingTempScope (e-ref ▷ &●) = enclosingTempScope e-ref
```

## Stable Rust

First, an auxiliary definition is required: *extending contexts* correspond to [destructors.scope.lifetime-extension.exprs.extending].

The operand of an extending borrow expression has an extended temporary scope. The `extendingScope` function determines if a context is extending, and if so, what scope it extends a borrowed temporary to.

```agda
extendingScope : Context Expr → Maybe Scope
-- `let` statements extend temporaries to the end of the block.
extendingScope (s-let ▷ let…=●⨾ _) = just (scope (blockOf s-let))
-- Constant items extend temporaries to the end of the program.
extendingScope (_ ▷ const…=●⨾) = just (scope ctxt-top)
-- Block tails, array expression operands, and borrow expression
-- operands are extending if their parents are, and extend to the same
-- scope.
extendingScope (e-block ▷ ⦃…⨾●⦄) = extendingScope e-block
extendingScope (e-array ▷ ⟦…,●,…⟧ _) = extendingScope e-array
extendingScope (e-ref ▷ &●) = extendingScope e-ref
-- These expressions aren't extending.
extendingScope (_ ▷ |…|●) = nothing
extendingScope (_ ▷ ●⨾) = nothing
extendingScope (_ ▷ …⦅…,●,…⦆ _) = nothing
extendingScope (_ ▷ ●⟦…⟧) = nothing
```

There is another consideration in the definition of temporary scopes: [destructors.scope.lifetime-extension.sub-expressions], also known as *the subexpressions rule*.

This is interpreted as defining a grammar of *subexpression contexts*. The temporary scope of a subexpression context is the same as its parent's temporary scope.

```agda
tempScope-stable : Context Expr → Scope
-- `let ref x = ●;` has an extended temporary scope.
tempScope-stable (s-let ▷ let…=●⨾ pat-ext) = scope (blockOf s-let)
-- The subexpressions rule: `●⟦…⟧` has the same temporary scope as
-- its parent, and `&●` does as well in "subexpression contexts".
tempScope-stable (e-indexing ▷ ●⟦…⟧) = tempScope-stable e-indexing
tempScope-stable (e-ref@(_ ▷ ●⟦…⟧) ▷ &●) = tempScope-stable e-ref
tempScope-stable (e-inner-ref@(_ ▷ &●) ▷ &●) =
  tempScope-stable e-inner-ref
-- The scope of `&●` in other contexts depends on whether its parent
-- is extending. If so, its temporary scope is extended by
-- `extendingScope`.  Otherwise, its temporary scope is its enclosing
-- temporary scope.
{-# CATCHALL #-}
tempScope-stable e-operand@(e-ref ▷ &●) with extendingScope e-ref
… | just x-extended = x-extended
… | nothing = enclosingTempScope e-operand
-- Our other contexts don't have extended temporary scopes.  Their
-- temporary scopes are their enclosing temporary scopes.
{-# CATCHALL #-}
tempScope-stable e-other = enclosingTempScope e-other
```

### Alternate formulation: expanded subexpressions rule

The subexpressions rule can be reinterpreted as applying more broadly and uniformly: the temporary scope of `e ▷ &●` for non-extending `e` is the temporary scope of `e`.

```agda
tempScope-stable′ : Context Expr → Scope
-- `let ref x = ●;` has an extended temporary scope.
tempScope-stable′ (s-let ▷ let…=●⨾ pat-ext) = scope (blockOf s-let)
-- `●[…]` has the same temporary scope as its parent, per the sub-expr
-- rule.
tempScope-stable′ (e-indexing ▷ ●⟦…⟧) = tempScope-stable′ e-indexing
-- The scope of `&●` depends on whether the borrow is extending.  If
-- it's extending, its temporary scope is extended by
-- `extendingScope`.  Otherwise, its temporary scope is the same as
-- its parent's, per the expanded subexpressions rule.
tempScope-stable′ (e-ref ▷ &●) with extendingScope e-ref
… | just x-extended = x-extended
… | nothing = tempScope-stable′ e-ref
-- Our other contexts don't have extended temporary scopes.  Their
-- temporary scopes are their enclosing temporary scopes.
{-# CATCHALL #-}
tempScope-stable′ e-other = enclosingTempScope e-other
```

This is equivalent to the definition of `tempScope-stable` above: `∀ e → tempScope-stable e ≡ tempScope-stable′ e`.

#### Proof

**Lemma**: If `e` is an extending context (`extendingScope e ≡ just x`), then `tempScope-stable (e ▷ &●)` is `x`. This requires induction because `tempScope-stable` recurses on `&●` chains.

I.e., "if a context `e` extends its temporaries to scope `x`, then the temporary scope of a borrow in that context is `x`".

```agda
lemma-extending : ∀ e {x}
  → (extendingScope e ≡ just x) → (tempScope-stable (e ▷ &●) ≡ x)
lemma-extending (e ▷ ⦃…⨾●⦄) p rewrite p = refl
lemma-extending (e ▷ ⟦…,●,…⟧ n) p rewrite p = refl
lemma-extending (e ▷ &●) p = lemma-extending e p
lemma-extending (s ▷ let…=●⨾ pat) refl = refl
lemma-extending (c ▷ const…=●⨾) refl = refl
lemma-extending (fn ▷ |…|●) ()
lemma-extending (s ▷ ●⨾) ()
lemma-extending (e ▷ …⦅…,●,…⦆ n) ()
lemma-extending (e ▷ ●⟦…⟧) ()
```

Proof that `tempScope-stable` and `tempScope-stable′` are equivalent.

```agda
proof-tempScope-equiv :
  ∀ e → tempScope-stable e ≡ tempScope-stable′ e
proof-tempScope-equiv (e ▷ |…|●) = refl
proof-tempScope-equiv (e ▷ const…=●⨾) = refl
proof-tempScope-equiv (e ▷ let…=●⨾ pat-ext) = refl
proof-tempScope-equiv (e ▷ let…=●⨾ pat-non) = refl
proof-tempScope-equiv (e ▷ ●⨾) = refl
proof-tempScope-equiv (e ▷ ⦃…⨾●⦄) = refl
proof-tempScope-equiv (e ▷ …⦅…,●,…⦆ n) = refl
proof-tempScope-equiv (e ▷ ⟦…,●,…⟧ n) = refl
proof-tempScope-equiv (e ▷ ●⟦…⟧) = proof-tempScope-equiv e
proof-tempScope-equiv ((e ▷ ●⟦…⟧) ▷ &●) = proof-tempScope-equiv e
proof-tempScope-equiv ((e ▷ &●) ▷ &●)
  -- Adding the recursive call to the with-abstraction is done as
  -- otherwise Agda's termination checker will fail to validate this
  -- as terminating.  See:
  --
  -- - <https://agda.readthedocs.io/en/stable/language/with-abstraction.html#termination-checking>
  with proof-tempScope-equiv (e ▷ &●) | extendingScope e in eq
… | _ | just _ = lemma-extending e eq
… | rec | nothing = rec
proof-tempScope-equiv ((_ ▷ |…|●) ▷ &●) = refl
proof-tempScope-equiv ((_ ▷ const…=●⨾) ▷ &●) = refl
proof-tempScope-equiv ((_ ▷ let…=●⨾ _) ▷ &●) = refl
proof-tempScope-equiv ((_ ▷ ●⨾) ▷ &●) = refl
proof-tempScope-equiv ((e ▷ ⦃…⨾●⦄) ▷ &●) with extendingScope e
… | just _ = refl
… | nothing = refl
proof-tempScope-equiv ((_ ▷ …⦅…,●,…⦆ _) ▷ &●) = refl
proof-tempScope-equiv ((e ▷ ⟦…,●,…⟧ _) ▷ &●) with extendingScope e
… | just _ = refl
… | nothing = refl
```

## Compiler PR #146098

For [Rust PR #146098], the partial definition of extending expressions (`extendingScope`) is replaced with a total function that determines the scope of borrow expressions' operands in any non-subexpression context. This is what was called the "extended scope" in [Reference PR #2051].

`refScope-146098 e` is the scope of `e ▷ &●` when `e` is not a subexpression context.

```agda
refScope-146098 : Context Expr → Scope
-- `let x = &●;` has an extended temporary scope.
refScope-146098 (s-let ▷ let…=●⨾ _) = scope (blockOf s-let)
-- `const … = &●;` has an extended temporary scope.
refScope-146098 (_ ▷ const…=●⨾) = scope ctxt-top
-- Extending subexpressions preserve `&●`'s temporary scope.
refScope-146098 (e-block ▷ ⦃…⨾●⦄) = refScope-146098 e-block
refScope-146098 (e-array ▷ ⟦…,●,…⟧ _) = refScope-146098 e-array
refScope-146098 (e-ref-outer ▷ &●) = refScope-146098 e-ref-outer
-- In other contexts, the temp scope of `&●` is the enclosing temp
-- scope.
{-# CATCHALL #-}
refScope-146098 e-other = enclosingTempScope e-other
```

Temporary scoping with this new rule can then be defined analogously to the definition for stable Rust:

```agda
tempScope-146098 : Context Expr → Scope
-- `let ref x = ●;` has an extended temporary scope.
tempScope-146098 (s-let ▷ let…=●⨾ pat-ext) = scope (blockOf s-let)
-- The subexpressions rule: `●⟦…⟧` has the same temporary scope as
-- its parent, and `&●` does as well in "subexpression contexts".
tempScope-146098 (e-indexing ▷ ●⟦…⟧) = tempScope-146098 e-indexing
tempScope-146098 (e-ref@(_ ▷ ●⟦…⟧) ▷ &●) = tempScope-146098 e-ref
tempScope-146098 (e-inner-ref@(_ ▷ &●) ▷ &●) = tempScope-146098 e-inner-ref
-- The scope of `&●` in other contexts is given by `refScope-146098`.
{-# CATCHALL #-}
tempScope-146098 (e-ref ▷ &●) = refScope-146098 e-ref
-- Our other contexts don't have extended temporary scopes.
-- Their temporary scopes are their enclosing temporary scopes.
{-# CATCHALL #-}
tempScope-146098 e-other = enclosingTempScope e-other
```

This still splits temporary scoping into two steps: first, apply the subexpressions rule; second, if you end up in an `&●` you take the `refScope-146098`, and otherwise you take the `enclosingTempScope`. The separation is important: it means [Rust PR #146098] only extends temporaries past statement/`const` boundaries when stable Rust does.

## The expanded subexpressions rule

In [this commit to the Reference PR](https://github.com/rust-lang/reference/pull/2051/commits/6536b3be7957261bfe91567af8737454b6c34d40), an alternative interpretation of the subexpressions rule is considered: that the temporary scope of `&●` in a non-extending context is the temporary scope of its parent. This is achieved by defining `tempScope-refPr` and `refScope-refPr` through mutual recursion:

```agda
tempScope-refPr : Context Expr → Scope
refScope-refPr : Context Expr → Scope

-- `let ref x = ●;` has an extended temporary scope.
tempScope-refPr (s-let ▷ let…=●⨾ pat-ext) = scope (blockOf s-let)
-- `●[…]` has the same temporary scope as its parent, per the expanded
-- subexpressions rule.
tempScope-refPr (e-indexing ▷ ●⟦…⟧) = tempScope-refPr e-indexing
-- The temporary scope of `&●` is given by `refScope-refPr`.
tempScope-refPr (e-ref ▷ &●) = refScope-refPr e-ref
-- Our other contexts don't have extended temporary scopes.  Their
-- temporary scopes are their enclosing temporary scopes.
{-# CATCHALL #-}
tempScope-refPr e-other = enclosingTempScope e-other

-- `let x = &●;` has an extended temporary scope.
refScope-refPr (s-let ▷ let…=●⨾ _) = scope (blockOf s-let)
-- `const … = &●;` has an extended temporary scope.
refScope-refPr (_ ▷ const…=●⨾) = scope ctxt-top
-- Extending subexpressions preserve `&●`'s temporary scope.
refScope-refPr (e-block ▷ ⦃…⨾●⦄) = refScope-refPr e-block
refScope-refPr (e-array ▷ ⟦…,●,…⟧ _) = refScope-refPr e-array
refScope-refPr (e-ref-outer ▷ &●) = refScope-refPr e-ref-outer
-- `&●` outside of an extending subexpression has the same temporary
-- scope as its parent, per the expanded subexpressions rule.
{-# CATCHALL #-}
refScope-refPr e-ref-other = tempScope-refPr e-ref-other
```

Similar to stable Rust, the scoping rules for `e ▷ &●` no longer depend on whether `e` is a subexpression context. This does however mean that it allows more programs than [Rust PR #146098] does by extending some temporaries past statement boundaries. Consider the expression context `let x = &[&●][0];`:

```agda
let＿=&⟦&●⟧⟦0⟧⨾ : Context Stmt → Context Expr
let＿=&⟦&●⟧⟦0⟧⨾ s = s ▷ let…=●⨾ pat-non ▷ &● ▷ ●⟦…⟧ ▷ ⟦…,●,…⟧ 0 ▷ &●
```

In stable Rust and under [Rust PR #146098], the context's temporary scope is not extended, meaning any later use of `x` would result in a borrow-checking error:

```agda
scope-let＿=&⟦&●⟧⟦0⟧⨾-stable : ∀ s-let →
  tempScope-stable (s-let |> let＿=&⟦&●⟧⟦0⟧⨾) ≡ scope s-let
scope-let＿=&⟦&●⟧⟦0⟧⨾-stable _ = refl

scope-let＿=&⟦&●⟧⟦0⟧⨾-146098 : ∀ s-let →
  tempScope-146098 (s-let |> let＿=&⟦&●⟧⟦0⟧⨾) ≡ scope s-let
scope-let＿=&⟦&●⟧⟦0⟧⨾-146098 _ = refl
```

However, with the expanded subexpressions rule, this is extended to the end of the block:

```agda
scope-let＿=&⟦&●⟧⟦0⟧⨾-refPr : ∀ s-let →
  tempScope-refPr (s-let |> let＿=&⟦&●⟧⟦0⟧⨾) ≡ scope (blockOf s-let)
scope-let＿=&⟦&●⟧⟦0⟧⨾-refPr _ = refl
```

## Alternate formulation: single function

These rules can also be expressed using a single function:

```agda
tempScope-refPr′ : Context Expr → Scope
-- `let ref x = ●;` and `let _ = &●;` have extended temporary scopes.
tempScope-refPr′ (s-let ▷ let…=●⨾ pat-ext) = scope (blockOf s-let)
tempScope-refPr′ (s-let ▷ let…=●⨾ _ ▷ &●) = scope (blockOf s-let)
-- `const … = &●;` has an extended temporary scope.
tempScope-refPr′ (_ ▷ const…=●⨾ ▷ &●) = scope ctxt-top
-- `●[…]` has the same temporary scope as its parent, per the expanded
-- subexpressions rule.
tempScope-refPr′ (e-indexing ▷ ●⟦…⟧) = tempScope-refPr′ e-indexing
-- Extending subexpressions preserve `&●`'s temporary scope.
tempScope-refPr′ (e-block ▷ ⦃…⨾●⦄ ▷ &●) =
  tempScope-refPr′ (e-block ▷ &●)
tempScope-refPr′ (e-array ▷ ⟦…,●,…⟧ _ ▷ &●) =
  tempScope-refPr′ (e-array ▷ &●)
tempScope-refPr′ (e-ref-outer ▷ &● ▷ &●) =
  tempScope-refPr′ (e-ref-outer ▷ &●)
-- `&●` outside of an extending subexpression has the same temporary
-- scope as its parent, per the expanded subexpressions rule.
{-# CATCHALL #-}
tempScope-refPr′ (e-ref-other ▷ &●) = tempScope-refPr′ e-ref-other
-- Our other contexts don't have extended temporary scopes.
-- Their temporary scopes are their enclosing temporary scopes.
{-# CATCHALL #-}
tempScope-refPr′ e-other = enclosingTempScope e-other
```

This offers a more visual interpretation of extending subexpressions: to find the temporary scope of `&●` within an extending subexpression, imagine moving the `&●` up as though it was in its parent's place. `tempScope-146098` can also be expressed in this way, but it still requires two function definitions or some extra bit of state to encode whether the subexpressions rule has been applied already.

### Proof

```agda
≡refScope-refPr'|e▷&● : ∀ e
  → refScope-refPr e ≡ tempScope-refPr′ (e ▷ &●)
≡tempScope-refPr′ : ∀ e → tempScope-refPr e ≡ tempScope-refPr′ e

≡refScope-refPr'|e▷&● (e ▷ |…|●) = refl
≡refScope-refPr'|e▷&● (e ▷ const…=●⨾) = refl
≡refScope-refPr'|e▷&● (e ▷ let…=●⨾ pat) = refl
≡refScope-refPr'|e▷&● (e ▷ ●⨾) = refl
≡refScope-refPr'|e▷&● (e ▷ ⦃…⨾●⦄) = ≡refScope-refPr'|e▷&● e
≡refScope-refPr'|e▷&● (e ▷ …⦅…,●,…⦆ n) = refl
≡refScope-refPr'|e▷&● (e ▷ ⟦…,●,…⟧ n) = ≡refScope-refPr'|e▷&● e
≡refScope-refPr'|e▷&● (e ▷ ●⟦…⟧) = ≡tempScope-refPr′ e
≡refScope-refPr'|e▷&● (e ▷ &●) = ≡refScope-refPr'|e▷&● e

≡tempScope-refPr′ (e ▷ |…|●) = refl
≡tempScope-refPr′ (e ▷ const…=●⨾) = refl
≡tempScope-refPr′ (e ▷ let…=●⨾ pat-ext) = refl
≡tempScope-refPr′ (e ▷ let…=●⨾ pat-non) = refl
≡tempScope-refPr′ (e ▷ ●⨾) = refl
≡tempScope-refPr′ (e ▷ ⦃…⨾●⦄) = refl
≡tempScope-refPr′ (e ▷ …⦅…,●,…⦆ n) = refl
≡tempScope-refPr′ (e ▷ ⟦…,●,…⟧ n) = refl
≡tempScope-refPr′ (e ▷ ●⟦…⟧) = ≡tempScope-refPr′ e
≡tempScope-refPr′ (e ▷ |…|● ▷ &●) = refl
≡tempScope-refPr′ (e ▷ const…=●⨾ ▷ &●) = refl
≡tempScope-refPr′ (e ▷ let…=●⨾ pat ▷ &●) = refl
≡tempScope-refPr′ (e ▷ ●⨾ ▷ &●) = refl
≡tempScope-refPr′ (e ▷ ⦃…⨾●⦄ ▷ &●) = ≡refScope-refPr'|e▷&● e
≡tempScope-refPr′ (e ▷ …⦅…,●,…⦆ n ▷ &●) = refl
≡tempScope-refPr′ (e ▷ ⟦…,●,…⟧ n ▷ &●) = ≡refScope-refPr'|e▷&● e
≡tempScope-refPr′ (e ▷ ●⟦…⟧ ▷ &●) = ≡tempScope-refPr′ e
≡tempScope-refPr′ (e ▷ &● ▷ &●) = ≡refScope-refPr'|e▷&● e
```

[literate Agda]: https://agda.readthedocs.io/en/latest/tools/literate-programming.html
[Reference PR #2051]: https://github.com/rust-lang/reference/pull/2051
[Rust PR #146098]: https://github.com/rust-lang/rust/pull/146098
