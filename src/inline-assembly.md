# Inline assembly

r[asm]

r[asm.macros]
The macros [`core::arch::asm!`] and [`core::arch::global_asm!`] expand to inline assembly syntax when used in the expression position and item position respectively. The macros shall not be expanded in any other context.

> [!NOTE]
> The expansion of the macros has no stable syntax equivalent. This section will refer to the expansion of the macro, rather than the surface syntax.

r[asm.safety]
The macro [`core::arch::asm!`] shall be expanded only within an `unsafe` block.

> [!NOTE]
> Inline assembly is inherently unsafe.
> It requires asserting various constraints to the compiler that it cannot check, and can perform operations equivalent to calling a foreign function.


```rust,compile_fail
# #[cfg(target_arch = "x86_64")] {
    use core::arch::asm;
    asm!("/*inline assembly is inherently unsafe*/");
# }
```

r[asm.support]
Inline assembly is supported only when compiling for a target using one of the following architectures. A program that contains inline assembly is ill-formed on any other target:
- x86 and x86-64
- ARM
- AArch64
- RISC-V
- LoongArch


```rust
# #[cfg(target_arch = "x86_64")] {
use std::arch::asm;

// Multiply x by 6 using shifts and adds
let mut x: u64 = 4;
unsafe {
    asm!(
        "mov {tmp}, {x}",
        "shl {tmp}, 1",
        "shl {x}, 2",
        "add {x}, {tmp}",
        x = inout(reg) x,
        tmp = out(reg) _,
    );
}
assert_eq!(x, 4 * 6);
# }
```

## Syntax 

r[asm.syntax]

```abnf
format_string := STRING_LITERAL / RAW_STRING_LITERAL
dir_spec := "in" / "out" / "lateout" / "inout" / "inlateout"
reg_spec := <register class> / "\"" <explicit register> "\""
input_expr := expr
output_expr := expr / "_"
inout_expr := input_expr ["=>" output_expr]
operand_expr := input_expr / output_expr / inout_expr
sym_expr := path
reg_operand := [ident "="] dir_spec "(" reg_spec ")" operand_expr
sym_operand := [ident "="] "sym" sym_expr
clobber_abi := "clobber_abi(" <abi> *("," <abi>) [","] ")"
option := "pure" / "nomem" / "readonly" / "preserves_flags" / "noreturn" / "nostack" / "att_syntax" / "raw"
options := "options(" option *("," option) [","] ")"
operand := reg_operand / sym_operand / clobber_abi / options
asm_inner := format_string *("," format_string) *("," operand) [","]
asm := "asm!(" asm_inner ")"
global_asm := "global_asm!(" asm_inner ")"

non_format_char := ANY_CHAR // except "{" and "}"
operand_specifier := ident / DEC_LITERAL
expansion_specifier := *non_format_char
format_specifier := "{" [operand_specifier] [":" *expansion_specifier]  "}"
format_escape := "{{" / "}}"
asm_string_piece := non_format_char / format_specifier / format_escape
asm_string_content := [*asm_string_piece]
```

## Invocation 

r[asm.invocation]

r[asm.invocation.asm]
The [`core::arch::asm!`] macro shall be expanded in an expression context only. The input tokens shall match the `asm_inner` production. The expansion is [`unsafe`][static.expr.safety] and has type `()`, unless the option `noreturn` is specified, in which case it has type `!`.

```rust
pub fn main() {
  # #[cfg(target_arch = "x86_64")] 
  unsafe{
    core::arch::asm!("")
  }
}
```

r[asm.invocation.global_asm]
The [`core::arch::global_asm!`] macro shall be expanded in an item context only. The input tokens shall match the `asm_inner` production. If the macro is expanded in a function, the program is ill-formed. 

<!--TODO: Test `global_asm!`-->

```rust,ignore
# #[cfg(target_arch = "x86_64")]
core::arch::global_asm!(".rodata", "FOO:", ".ascii \"Hello World\"");
```

```rust,compile_fail
pub fn main() {
# #[cfg(target_arch = "x86_64")] 
# {
    core::arch::global_asm!("FOO:", ".ascii \"Hello World\"");
# }
}
# #[cfg(not(target_arch = "x86_64"))]
# core::compile_error!("asm tests are not yet available off of x86_64"); 
```

r[asm.invocation.format-string]
Unless the `raw` option is specified, each `format_string` input to the [`core::arch::asm!`] and [`core::arch::global_asm!`] macros shall be an expanded string literal for which the content matches the `asm_string_piece` production.

> [!NOTE]
> an expanded string literal is a string literal (after expanding macros like [`core::concat!`]) that has had every unicode escape sequence replaced with the (appropriately escaped as needed) matching character, and which has been normalized from a raw string literal.


r[asm.invocation.concat]
If multiple `format_string` inputs are provided, then they are concatenated as though by the [`core::concat!`] macro, separating each `format_string` with a string containing a single newline character. If any `format_string` begins a `format_specifier` that is not terminated before the end of the `format_string`, the program is ill-formed. The resulting string is known as the *joined asm-string*

```rust
# #[cfg(target_arch = "x86_64")] {unsafe{
let mut x: i32;
// The following lines are equivalent
core::arch::asm!("mov rax, 5", "mov rcx, rax", out("rax") x, out("rcx") _); 
core::arch::asm!("mov rax, 5\nmov rcx, rax", out("rax") x, out("rcx") _);  
# }}
```

r[asm.invocation.operands]
Each operand, other than an explicit register operand ([asm.operands.register]) shall be mentioned by at least one format_specifier in the *joined asm-string*. Explicit registers may not be referred to be a format_specifier.

```rust,compile_fail
# #[cfg(target_arch = "x86_64")] { unsafe{
core::arch::asm!("", in(reg) 5i64);
# }}
# #[cfg(not(target_arch = "x86_64"))]
# core::compile_error!("asm tests are not yet available off of x86_64"); 
```

r[asm.invocation.positional]
A `format_specifier` that does not specify an `operand_specifier` is called a positional specifier, and refers to the `nth` successive positional operand, where `n` is `0` for the first positional specifier in the *joined asm-string* and increases by 1 for each successive positional specifier in the *joined asm-string*.

```rust
# #[cfg(target_arch = "x86_64")] { unsafe{
let x: i32;
core::arch::asm!("mov rax, {}", in(reg) 5i64, out("eax") x);
# }}
```

r[asm.invocation.explicit-positional]
A `format_specifier` that has an `operand_specifier` which is a DEC_LITERAL is called an explicit positional specifier, and refers to the `nth` successive positional operand, where `n` is the value of the DEC_LITERAL.

```rust
# #[cfg(target_arch = "x86_64")] { unsafe{
let x: i32;
core::arch::asm!("mov {1}, {0}", in(reg) 5i64, out(reg) x);
# }}
```

r[asm.invocation.named]
A `format_specifier` that has an `operand_specifier` which is an ident is called a named specifier, and refers to the named operand with the specified name.

```rust
# #[cfg(target_arch = "x86_64")] { unsafe{
let x: i32;
core::arch::asm!("mov {output}, {input}", input = in(reg) 5i64, output = out(reg) x);
# }}
```

r[asm.invocation.expansion]
If the `raw` option is not specified, the *joined asm-string* is expanded as defined in [asm.operands.expansion], replacing each `format_specifier` with the appropriate expansion for the operand. The resulting string is called the *expanded asm-string*. If the `raw` option is specified, the *expanded asm-string* is the *joined asm-string* verbatim. 

r[asm.invocation.syntax]
The syntax of the *expanded asm-string* is a subset of the GNU AS syntax for the target. Invoking the macro with a *expanded asm-string* that does not match syntax requirements is *conditionally supported* and has *assembler dependent behaviour*. Invoking a directive that is not specified by [asm.directives] is *conditionally supported* and has *assembler dependent behaviour*.

> [!TARGET-SPECIFIC]
> On x86 and x86_64 targets, the syntax of the *expanded asm-string* acts as though the directive `.intel_syntax noprefix` is issued before parsing the *expanded asm-string*, except that the `option(att_syntax)` causes the syntax to act as though the directive `.att_syntax prefix` is issued before parsing the *expanded asm-string* instead.
> On ARM targets, the syntax of the *expanded asm-string* acts as though the directive `.syntax unified` is issued before parsing the *expanded asm-string*.

r[asm.invocation.duplication]
The number of times, locations, and the order in which a given invocation of [`core::arch::asm!`] is expanded is unspecified.

```rust,ignore
// The following code may have suprising results, and may fail to compile or link. 
// The results, including whether it succesfully compiles, may depend on non-local use sites of the function, and on optimization settings.
# #[cfg(target_arch = "x86_64")] { unsafe{
let x: i32;
core::arch::asm!("foo: jmp foo");
# }}
```

> [!NOTE]
> In particular, an asm block may be duplicated, for example if the containing function is inlined, or omitted from the output entirely.
> As a consequence, asm blocks should not use directives that have non-idempotent non-local effects, or named labels and symbol definitions. 
> Additionally, two asm blocks may not rely upon being adjacent in executable memory, even if they are adjacent in the source.

> [!NOTE]
> Local Labels (a decimal literal) may be used freely if the asm block needs to define a label. Due to a bug, literals that solely consist of 1s and 0s are not valid local labels.
> See [The GNU AS Manual on Local Labels](https://sourceware.org/binutils/docs/as/Symbol-Names.html) for details on local labels.
> It is not guaranteed that a local label defined in one asm block will be accessible from an adjacent asm block.

```rust,no_run
# #[cfg(target_arch = "x86_64")] { unsafe{
let x: i32;
core::arch::asm!("2: jmp 2b");
# }}
```

r[asm.invocation.global-order]
The order in which invocations of [`core::arch::global_asm!`] are expanded is unspecified.

r[asm.invocation.directive-state]
The *expanded asm-string* shall not issue a directive that modifies the global state of the assembler for processing inputs unless it issues a directive to restore that state it had upon entering the block. No diagnostic is required.

> [!NOTE]
> This include state such as the current section of the assembler, the syntax mode, or the kind of assembly output being generated.
> Failing to obey this requirement can have significant impact on code generation, including code unrelated to the asm block. For example, an asm block that issues a `.data` directive without resetting to the appropriate section for the function can cause the following code in the function to be generated in the `.data` section, and for execution to fall off the asm block into improper memory.

r[asm.invocation.global-section]
The *expanded asm-string* of a [`core::arch::global_asm!`] invocation acts as though a `.section` directive is issued before the *expanded asm-string*  which causes code to be generated in the default section on the target for executable code.

> [!NOTE]
> This section is typically named `.text`. 


r[asm.invocation.prefix-instr]
An *expanded asm-string* shall not end with an instruction that is interpreted as a prefix on the architecture. No Diagnostic is required

> [!TARGET-SPECIFIC]
> On x86 and x86-64, the `lock`, `repnz`, `rep`, `repz`, as well as GNU AS specific address-size, data-size, and explicit rex, vex, and evex prefixes.


```rust,ignore
// The following snippet is ill-formed
# #[cfg(target_arch = "x86_64")] { unsafe{
core::arch::asm!("lock");
# }}
```

## Operand types 

r[asm.operands]

r[asm.operands.positional]
Operands that do not specify an ident and are not explicit register operands are known as positional operands. Positional operands may be referred to only by positional operand specifiers and explicit positional operand specifiers, and each Positional operand must be specified before Named Operands or Explicit Register Operands.
```rust
# #[cfg(target_arch = "x86_64")] { unsafe{
let mut x: i32;
core::arch::asm!("mov rax, {}", in(reg) 5i64, out("eax") x);
core::arch::asm!("mov {1}, {0}", in(reg) 5i64, out(reg) x);
# }}
```


r[asm.operands.named]
Operands that specify an ident are named operands. A named operand shall not specify an explicit register `reg_spec`. Named operand specifiers may be referred to only by named operand specifiers.
```rust
# #[cfg(target_arch = "x86_64")] { unsafe{
let x: i32;
core::arch::asm!("mov rax, {input}", input = in(reg) 5i64, out("eax") x);
# }}
```

r[asm.operands.registers]
Operands that specify an explicit register `reg_spec` are explicit register operands. 

```rust
# #[cfg(target_arch = "x86_64")] { unsafe{
let x: i32;
core::arch::asm!("mov eax, ecx", in("rcx") 5i64, out("eax") x);
# }}
```

> [!NOTE]
> Explicit Register Operands have no `ident` name and cannot be referred to by an operand specifier

r[asm.operands.types]
Each operand, other than a placeholder expression shall be of an integer type, floating-point type, function pointer type, pointer type, target-specific vector type, or [`MaybeUninit<T>`][core::mem::MaybeUninit] where `T` is an *asm operand type* other than [`MaybeUninit`][core::mem::MaybeUninit]. These types are collectively called *asm operand types*. A pointer type is an *asm operand type* only if the pointee type has no metadata-type.

```rust,compile_fail
# #[cfg(target_arch = "x86_64")] { unsafe{
struct Foo{x: i32}
// Complex types like structs can't be used for asm
let x: Foo;
core::arch::asm!("mov {output}, {input}", input = in(reg) 5i64, out("eax") x);
# }}
# #[cfg(not(target_arch = "x86_64"))] compile_error!("Inline Assembly Tests are not supported off of x86_64");
```

```rust,compile_fail
# #[cfg(target_arch = "x86_64")] { unsafe{
// ... nor can wide pointers
let x: *mut [i32];
core::arch::asm!("mov {output}, {input}", input = in(reg) 5i64, out("eax") x);
# }}
# #[cfg(not(target_arch = "x86_64"))] compile_error!("Inline Assembly Tests are not supported off of x86_64");
```


> [!TARGET-SPECIFIC]
> On x86 platforms, the types [`__m128`], [`__m256`], and variants of those types are *asm operand types*.

[`__m128`]: https://doc.rust-lang.org/core/arch/x86_64/struct.__m128.html
[`__m256`]: https://doc.rust-lang.org/core/arch/x86_64/struct.__m256.html

```rust
# #[cfg(target_arch = "x86_64")] { unsafe{
# use core::arch::x86_64::__m128;
// But vector types are allowed.
let x: __m128;
core::arch::asm!("xorps xmm0, xmm0", out("xmm0") x);
# }}
```

r[asm.operands.input-coerceable-types]
Each reference type, where the pointee type has no metadata-type, and each function item type are collectively called *input coerceable types*.

```rust
# #[cfg(target_arch = "x86_64")] { unsafe{
let x = 5;
let y: i32;
core::arch::asm!("mov eax, dword ptr [{}]", in(reg) &x, out("eax") y); // equivalent to asm!("mov eax, dword ptr [{}]", in(reg) (&x) as *const i32, out("eax") y); 
#}}
```

```rust,compile_fail
# #[cfg(target_arch = "x86_64")] { unsafe{
let y: &mut i32;
core::arch::asm!("mov {}, 0", out(reg) 5); 
#}}
# #[cfg(not(target_arch = "x86_64"))] compile_error!("Inline Assembly Tests are not supported off of x86_64");
```


r[asm.operands.in-expr]
An `input_expr` shall be a value expression of an *asm operand type* or an *input coerceable type*. If the expression is of an *input coerceable type*, it is coerced to an *asm operand type*. 

r[asm.operands.out-expr]
An `output_expr` shall be the placeholder expression `_` or a (potentially unitialized) place expression of an *asm operand type*. If the place expression is initialized, it shall be a mutable place.

```rust,compile_fail
# #[cfg(target_arch = "x86_64")] { unsafe{
let x: i32 = 0;
core::arch::asm!("", out("eax") x);
# }}
# #[cfg(not(target_arch = "x86_64"))] compile_error!("Inline Assembly Tests are not supported off of x86_64");
```

r[asm.operands.inout-expr]
An `inout_expr` shall either be an (initialized) place expression of an *asm operand type*, or shall specify both an `input_expr` and an `output_expr`. If only a single expression is specified, it is treated as both the `input_expr` and `output_expr` of the operand.

> [!NOTE]
> When a single expression is specified, it must be an initialized mutable place expression.

r[asm.operands.in]
An `in` operand is an reg_operand with the `in` dir_spec. The `operand_expr` of the operand shall be an `input_expr`. The `input_expr` initializes the value of the register before entering the asm block.

r[asm.operands.out]
An `out` operand is a reg_operand with the `out` dir_spec, and a `lateout` operand is a reg_operand with the `lateout` dir_spec. The `operand_expr` of an `out` operand or `lateout` operand shall be an `output_expr`. The value of the register at the exit of the asm block is written to the `output_expr` place if it is not a placeholder expression

> [!NOTE]
> A `lateout` operand differs from an `out` operand only in that the implementation may assume that no `in`, `inout`, or `inlateout` operands are read after a `lateout` operand is modified by the *expanded asm-string*.

r[asm.operands.inout]
An `inout` operand is a reg_operand with the `inout` dir_spec, and a `inlateout` operand is a reg_operand with the `inlateout` dir_spec. The `operand_expr` of an `inout` operand or an `inlateout` operand shall be an `inout_expr`. The `input_expr` and `output_expr` of an `inout` or `inlateout` operand is used as though the `inout` operand is replaced with a separate `in` and `out` operand, and the `inlateout` operand is replaced with a separate `in` and `lateout` operand, except that both have the same position if they are positional, or the same name if they are named operands, and both refer to the same register.

> [!NOTE]
> An `inlateout` operand differs from an `inout` operand only in that implementation may assume that no other `in`, `inout`, or `inlateout` operands are read after an `inlateout` operand is modified by the *expanded asm-string*. 


r[asm.operands.clobbers]
An `output_expr` that is the placeholder expression `_` is a clobbers output. The resulting value of the register is discarded. An `out` operand that is a clobbers output shall be an *explicit register operand*. 

> [!NOTE]
> Some registers and register classes cannot be used as an operand, other than as a clobber operand.

```rust
# #[cfg(target_arch = "x86_64")] { unsafe{
let mut x: i32;
core::arch::asm!("mov eax, 5", out("eax") _);
# }}
```

r[asm.operands.sym-expr]
A sym-expr is a path-expr. If the `path-expr` does not refer to a `static` item or a `fn` item, the program is ill-formed.

> [!NOTE]
> the path-expr may have any type, including a type that isn't an *asm operand type*, and may be either mutable or immutable.

r[asm.operand.sym]
A sym operand is an operand that uses the `sym` keyword. The operand contains a `sym-expr` that specifies the item the symbol refers to.

```rust
# #[cfg(target_arch = "x86_64")] { unsafe{
# use core::mem::MaybeUninit;
static FOO: MaybeUninit<i32> = MaybeUninit::zeroed();
let x: i32;
core::arch::asm!("mov eax, dword ptr [{}+rip]", sym FOO, out("eax") x);
# }}
```

r[asm.operands.expansion]
Each operand_spec is expanded in the *joined asm-string* according to the modifiers in `modifier_spec` and the operand. Each reg_operand is assigned to a register according to the reg_spec, and expands to the appropriate version of the `reg_operand`, in the format expected by the asm syntax in effect to specify the appropriate register. A sym operand expand to the linkage name ([dynamic.linkage.name]) of the item referred to by the `path-expr`, if it has either the `#[no_mangle]` or `#[export_name]` attribute, or is defined in an `extern` block, and otherwise, it expands to an unspecified string that can be used within the *expanded asm-string* to refer to the item. 

> [!NOTE]
> The name given to an item used by a sym-expr that does not have a linkage name may be known as the "mangled" name of the item.

> [!NOTE]
> A sym operand does not include any relocation modifiers such as `@plt` or `@tpoff`. The *joined asm-string* is responsible for including these as required.

> [!TARGET-SPECIFIC]
> On x86 and x86_64 targets, the register name is expanded as-is if the `options(att_syntax)` is not used, and with the `%` prefix if `options(att_syntax)` is used. 

r[asm.operands.global]
The program shall not use an operand, other than a sym operand, in the expansion of the [`core::arch::global_asm!`] macro.

<!--TODO: Test `global_asm!`-->

```rust,compile_fail,ignore
# #[cfg(target_arch = "x86_64")]
core::arch::global_asm!("", in("eax") 5);
# #[cfg(not(target_arch = "x86_64"))] compile_error!("Inline Assembly Tests are not supported off of x86_64");
```

```rust,ignore
static FOO: () = ();
# #[cfg(target_arch = "x86_64")]
core::arch::global_asm!("/*{}*/", sym FOO);
```

r[asm.operands.clobbers_abi]
A special operand `clobbers_abi` may be specified. If the `clobers_abi` operand is specified, then no `out`, `lateout`, `inout`, or `inlateout` reg_operand, other than an *explicit register operand*, shall be specified. When specified, it accepts a string literal which shall belong to a subset of the string literals accepted for an `extern` calling convention specification. The `clobbers_abi` special operand acts as though it is replaced by a `lateout` operand with an out-expr of `_` for each register considered by the specified calling convention to not be preserved by a function call. 


> [!NOTE]
> Multiple `clobbers_abi` operands may be specified. If a register is considered clobbered by multiple `clobbers_abi` operands, it acts as though only one of those `clobbers_abi` operands specifies that register.

> [!TARGET-SPECIFIC]
> The list of supported ABI strings and current list of clobbered registers are
> | Architecture | ABI name | Clobbered registers |
> | ------------ | -------- | ------------------- |
> | x86-32 | `"C"`, `"system"`, `"efiapi"`, `"cdecl"`, `"stdcall"`, `"fastcall"` | `ax`, `cx`, `dx`, `xmm[0-7]`, `mm[0-7]`, `k[0-7]`, `st([0-7])` |
> | x86-64 | `"C"`, `"system"` (on Windows), `"efiapi"`, `"win64"` | `ax`, `cx`, `dx`, `r[8-11]`, `xmm[0-31]`, `mm[0-7]`, `k[0-7]`, `st([0-7])`, `tmm[0-7]` |
> | x86-64 | `"C"`, `"system"` (on non-Windows), `"sysv64"` | `ax`, `cx`, `dx`, `si`, `di`, `r[8-11]`, `xmm[0-31]`, `mm[0-7]`, `k[0-7]`, `st([0-7])`, `tmm[0-7]` |
> | AArch64 | `"C"`, `"system"`, `"efiapi"` | `x[0-17]`, `x18`\*, `x30`, `v[0-31]`, `p[0-15]`, `ffr` |
> | ARM | `"C"`, `"system"`, `"efiapi"`, `"aapcs"` | `r[0-3]`, `r12`, `r14`, `s[0-15]`, `d[0-7]`, `d[16-31]` |
> | RISC-V | `"C"`, `"system"`, `"efiapi"` | `x1`, `x[5-7]`, `x[10-17]`, `x[28-31]`, `f[0-7]`, `f[10-17]`, `f[28-31]`, `v[0-31]` |
> | LoongArch | `"C"`, `"system"`, `"efiapi"` | `$r1`, `$r[4-20]`, `$f[0-23]` |

> [!NOTE]
> - On AArch64 `x18` only included in the clobber list if it is not considered as a reserved register on the target.

```rust
# #[cfg(target_arch = "x86_64")] { unsafe{
core::arch::asm!("", clobber_abi("C"));
# }}
```

r[asm.operands.clobbers_abi_ref]
A `clobbers_abi` special operand shall be specified after all positional operands, and shall not be a named operand. A `clobbers_abi` special operand cannot be referred to by an operand_specifier

## Register operands 

r[asm.registers]

r[asm.registers.explicit]
An explicit register operand specifies the name of a valid operand register that is not a reserved register, or an alias name of a valid operand register. Multiple explicit register operands shall not specify the same register or aliases of the same register. 

```rust
# #[cfg(target_arch = "x86_64")] { unsafe{
let x: i64;
core::arch::asm!("mov eax, 5", out("eax") x);
# }}
```

r[asm.registers.class]
A register operand that is not an explicit register operand specifies the name of a register class as an identifier. When a register class is specified, the implementation assigns an unspecified register belonging to that class to the operand.

```rust
# #[cfg(target_arch = "x86_64")] { unsafe{
let x: i64;
core::arch::asm!("mov {}, 5", out(reg) x);
# }}
```

r[asm.registers.valid-types]
Each register class, and the explicit registers within those classes, may restrict the set of types allowed for operands referring to that class or those registers. 

> [!NOTE]
> The types `isize`, `usize`, and function pointer types are considered valid for a given register class if and only if an integer type of the same width is considered valid.
> When a signed integer is considered valid for a given register class, the corresponding unsigned integer is also considered valid.

r[asm.registers.target-feature]
Each register class, and the explicit registers within that class may require that a specified target_feature is enabled in the ambient target_feature set, or by using the `target_feature` function attribute. The program shall not specify such registers or register classes, except as clobber output, when the feature is not enabled. Additionally specific types may be valid only if certain additional features are enabled.

> [!NOTE]
> The set of features in the ambient target_feature set are implementation-defined, and may be queried by [parse.macros.cfg].

r[asm.registers.class-list]

> [!TARGET-SPECIFIC]
> The list of valid register classes, the constituent registers, the required target feature (if any), and the valid types for those classes are:
> | Architecture | Register class | Registers | Target feature | Allowed types |
> | ------------ | -------------- | --------- | -------------- | ------------- |
> | x86-32 | `reg` | `ax`, `bx`, `cx`, `dx`, `si`, `di`, `bp` | None | `i16`, `i32`, `f32` |
> | x86-64 | `reg` | `ax`, `bx`, `cx`, `dx`, `si`, `di`, `bp`,  `r[8-15]` | None | `i16`, `i32`, `f32`, `i64`, `f64` |
> | x86-32 | `reg_abcd` | `ax`, `bx`, `cx`, `dx` | None | `i16`, `i32`, `f32` |
> | x86-64 | `reg_abcd` | `ax`, `bx`, `cx`, `dx` | None | `i16`, `i32`, `f32`, `i64`, `f64` |
> | x86-32 | `reg_byte` | `al`, `bl`, `cl`, `dl`, `ah`, `bh`, `ch`, `dh` | None | `i8` |
> | x86-64 | `reg_byte`\* | `al`, `bl`, `cl`, `dl`, `sil`, `dil`, `bpl`, `r[8-15]b` | None | `i8` |
> | x86 | `xmm_reg` | `xmm[0-7]` (x86) `xmm[0-15]` (x86-64) | `sse` | `i32`, `f32`, `i64`, `f64`, <br> `i8x16`, `i16x8`, `i32x4`, `i64x2`, `f32x4`, `f64x2` |
> | x86 | `ymm_reg` | `ymm[0-7]` (x86) `ymm[0-15]` (x86-64) | `avx` | `i32`, `f32`, `i64`, `f64`, <br> `i8x16`, `i16x8`, `i32x4`, `i64x2`, `f32x4`, `f64x2` <br> `i8x32`, `i16x16`, `i32x8`, `i64x4`, `f32x8`, `f64x4` |
> | x86 | `zmm_reg` | `zmm[0-7]` (x86) `zmm[0-31]` (x86-64) | `avx512f` | `i32`, `f32`, `i64`, `f64`, <br> `i8x16`, `i16x8`, `i32x4`, `i64x2`, `f32x4`, `f64x2` <br> `i8x32`, `i16x16`, `i32x8`, `i64x4`, `f32x8`, `f64x4` <br> `i8x64`, `i16x32`, `i32x16`, `i64x8`, `f32x16`, `f64x8` |
> | x86 | `kreg` | `k[1-7]` | `avx512f` | `i8`, `i16`, `i32` (requires `avx512bw`), `i64` (requires `avx512bw`) |
> | x86 | `kreg0` | `k0` | N/A | Only clobbers |
> | x86 | `x87_reg` | `st([0-7])` | N/A | Only clobbers |
> | x86 | `mmx_reg` | `mm[0-7]` | N/A | Only clobbers |
> | x86-64 | `tmm_reg` | `tmm[0-7]` | N/A | Only clobbers |
> | AArch64 | `reg` | `x[0-30]` | None | `i8`, `i16`, `i32`, `f32`, `i64`, `f64` |
> | AArch64 | `vreg` | `v[0-31]` | `neon` | `i8`, `i16`, `i32`, `f32`, `i64`, `f64`, <br> `i8x8`, `i16x4`, `i32x2`, `i64x1`, `f32x2`, `f64x1`, <br> `i8x16`, `i16x8`, `i32x4`, `i64x2`, `f32x4`, `f64x2` |
> | AArch64 | `vreg_low16` | `v[0-15]` | `neon` | `i8`, `i16`, `i32`, `f32`, `i64`, `f64`, <br> `i8x8`, `i16x4`, `i32x2`, `i64x1`, `f32x2`, `f64x1`, <br> `i8x16`, `i16x8`, `i32x4`, `i64x2`, `f32x4`, `f64x2` |
> | AArch64 | `preg` | `p[0-15]`, `ffr` | N/A | Only clobbers |
> | ARM (ARM/Thumb2) | `reg` | `r[0-12]`, `r14` | None | `i8`, `i16`, `i32`, `f32` |
> | ARM (Thumb1) | `reg` | `r[0-7]` | None | `i8`, `i16`, `i32`, `f32` |
> | ARM | `sreg` | `s[0-31]` | `vfp2` | `i32`, `f32` |
> | ARM | `sreg_low16` | `s[0-15]` | `vfp2` | `i32`, `f32` |
> | ARM | `dreg` | `d[0-31]` | `vfp2` | `i64`, `f64`, `i8x8`, `i16x4`, `i32x2`, `i64x1`, `f32x2` |
> | ARM | `dreg_low16` | `d[0-15]` | `vfp2` | `i64`, `f64`, `i8x8`, `i16x4`, `i32x2`, `i64x1`, `f32x2` |
> | ARM | `dreg_low8` | `d[0-8]` | `vfp2` | `i64`, `f64`, `i8x8`, `i16x4`, `i32x2`, `i64x1`, `f32x2` |
> | ARM | `qreg` | `q[0-15]` | `neon` | `i8x16`, `i16x8`, `i32x4`, `i64x2`, `f32x4` |
> | ARM | `qreg_low8` | `q[0-7]` | `neon` | `i8x16`, `i16x8`, `i32x4`, `i64x2`, `f32x4` |
> | ARM | `qreg_low4` | `q[0-3]` | `neon` | `i8x16`, `i16x8`, `i32x4`, `i64x2`, `f32x4` |
> | RISC-V32 | `reg` | `x1`, `x[5-7]`, `x[9-15]`, `x[16-31]` (non-RV32E) | None | `i8`, `i16`, `i32`, `f32` |
> | RISC-V64 | `reg` | `x1`, `x[5-7]`, `x[9-15]`, `x[16-31]`  | None | `i8`, `i16`, `i32`, `f32`, `i64`, `f64` |
> | RISC-V | `freg` | `f[0-31]` | `f` | `f32`, `f64` (requires `d`) |
> | RISC-V | `vreg` | `v[0-31]` | N/A | Only clobbers |
> | LoongArch | `reg` | `$r1`, `$r[4-20]`, `$r[23,30]` | None | `i8`, `i16`, `i32`, `i64`, `f32`, `f64` |
> | LoongArch | `freg` | `$f[0-31]` | None | `f32`, `f64` |

> **Notes**:
> - On x86 we treat `reg_byte` differently from `reg` because the compiler can allocate `al` and `ah` separately whereas `reg` reserves the whole register.
>
> - On x86-64 the high byte registers (e.g. `ah`) are not available in the `reg_byte` register class.
>

r[asm.register.clobbers_only]
Certain registers and register classes are *clobbers only*. Such register names or register classes shall not be specified by an operand, other than a clobbers output.

> [!TARGET-SPECIFIC]
> The list of such classes and registers are:
> * On x86 and x86-64: the `kreg0`, `x87_reg`, `mmx_reg`, and `tmm_reg` classes, as well as the registers belonging to these classes
> * On AArch64: the `preg` class, and the registers belonging to that class
> * On RISC-V: The `vreg` class, and the registers belonging to that class.

```rust,compile_fail
# #[cfg(target_arch = "x86_64")] { unsafe{
let x: i64;
core::arch::asm!("mov {}, 5", out("k0") x);
# }}
# #[cfg(not(target_arch = "x86_64"))] compile_error!("Inline Assembly Tests are not supported off of x86_64");
```

r[asm.register.small-values]
If a register input is specified with a type that has a smaller width than the register class according to the target, the remaining bits of the register are set to an unspecified value.

> [!TARGET-SPECIFIC]
> On RISC-V, in the case of an `freg` input of type `f32`, the upper bits are instead set to all 1s according to the `D` extension of the RISC-V specification.

```rust,ignore
// The following code may have unpredictable results
# #[cfg(target_arch = "x86_64")] { unsafe{
let x: i32 = 0;
let y: i64;
core::arch::asm!("mov {}, {}", out(reg) y, in(reg) x);
println!("{y}");
# }}
```

r[asm.register.aliases]
Certain explicit register names have defined aliases. These register names are considered identical to canonical register name and may be specified in place of the canonical name in an explicit register operand

> [!TARGET-SPECIFIC]
> The List of register alias names is:
> | Architecture | Base register | Aliases |
> | ------------ | ------------- | ------- |
> | x86 | `ax` | `eax`, `rax` |
> | x86 | `bx` | `ebx`, `rbx` |
> | x86 | `cx` | `ecx`, `rcx` |
> | x86 | `dx` | `edx`, `rdx` |
> | x86 | `si` | `esi`, `rsi` |
> | x86 | `di` | `edi`, `rdi` |
> | x86 | `bp` | `bpl`, `ebp`, `rbp` |
> | x86 | `sp` | `spl`, `esp`, `rsp` |
> | x86 | `ip` | `eip`, `rip` |
> | x86 | `st(0)` | `st` |
> | x86 | `r[8-15]` | `r[8-15]b`, `r[8-15]w`, `r[8-15]d` |
> | x86 | `xmm[0-31]` | `ymm[0-31]`, `zmm[0-31]` |
> | AArch64 | `x[0-30]` | `w[0-30]` |
> | AArch64 | `x29` | `fp` |
> | AArch64 | `x30` | `lr` |
> | AArch64 | `sp` | `wsp` |
> | AArch64 | `xzr` | `wzr` |
> | AArch64 | `v[0-31]` | `b[0-31]`, `h[0-31]`, `s[0-31]`, `d[0-31]`, `q[0-31]` |
> | ARM | `r[0-3]` | `a[1-4]` |
> | ARM | `r[4-9]` | `v[1-6]` |
> | ARM | `r9` | `rfp` |
> | ARM | `r10` | `sl` |
> | ARM | `r11` | `fp` |
> | ARM | `r12` | `ip` |
> | ARM | `r13` | `sp` |
> | ARM | `r14` | `lr` |
> | ARM | `r15` | `pc` |
> | RISC-V | `x0` | `zero` |
> | RISC-V | `x1` | `ra` |
> | RISC-V | `x2` | `sp` |
> | RISC-V | `x3` | `gp` |
> | RISC-V | `x4` | `tp` |
> | RISC-V | `x[5-7]` | `t[0-2]` |
> | RISC-V | `x8` | `fp`, `s0` |
> | RISC-V | `x9` | `s1` |
> | RISC-V | `x[10-17]` | `a[0-7]` |
> | RISC-V | `x[18-27]` | `s[2-11]` |
> | RISC-V | `x[28-31]` | `t[3-6]` |
> | RISC-V | `f[0-7]` | `ft[0-7]` |
> | RISC-V | `f[8-9]` | `fs[0-1]` |
> | RISC-V | `f[10-17]` | `fa[0-7]` |
> | RISC-V | `f[18-27]` | `fs[2-11]` |
> | RISC-V | `f[28-31]` | `ft[8-11]` |
> | LoongArch | `$r0` | `$zero` |
> | LoongArch | `$r1` | `$ra` |
> | LoongArch | `$r2` | `$tp` |
> | LoongArch | `$r3` | `$sp` |
> | LoongArch | `$r[4-11]` | `$a[0-7]` |
> | LoongArch | `$r[12-20]` | `$t[0-8]` |
> | LoongArch | `$r21` | |
> | LoongArch | `$r22` | `$fp`, `$s9` |
> | LoongArch | `$r[23-31]` | `$s[0-8]` |
> | LoongArch | `$f[0-7]` | `$fa[0-7]` |
> | LoongArch | `$f[8-23]` | `$ft[0-15]` |
> | LoongArch | `$f[24-31]` | `$fs[0-7]` |

```rust
# #[cfg(target_arch = "x86_64")] { unsafe{
let x: i64;
core::arch::asm!("mov eax, 5", out("rax") x);
# }}
```

r[asm.register.reserved]
Certain registers are reserved registers. Reserved Registers shall not be named by an explicit register operand.

> [!NOTE]
> Reserved Registers that belong to a register class may still be assigned to register operands regardless

> [!TARGET-SPECIFIC]
> | Architecture | Unsupported register | Reason |
> | ------------ | -------------------- | ------ |
> | All | `sp` | The stack pointer must be restored to its original value at the end of an asm code block. |
> | All | `bp` (x86), `x29` (AArch64), `x8` (RISC-V), `$fp` (LoongArch) | The frame pointer cannot be used as an input or output. |
> | ARM | `r7` or `r11` | On ARM the frame pointer can be either `r7` or `r11` depending on the target. The frame pointer cannot be used as an input or output. |
> | All | `si` (x86-32), `bx` (x86-64), `r6` (ARM), `x19` (AArch64), `x9` (RISC-V), `$s8` (LoongArch) | This is used internally by LLVM as a "base pointer" for functions with complex stack frames. |
> | x86 | `ip` | This is the program counter, not a real register. |
> | AArch64 | `xzr` | This is a constant zero register which can't be modified. |
> | AArch64 | `x18` | This is an OS-reserved register on some AArch64 targets. |
> | ARM | `pc` | This is the program counter, not a real register. |
> | ARM | `r9` | This is an OS-reserved register on some ARM targets. |
> | RISC-V | `x0` | This is a constant zero register which can't be modified. |
> | RISC-V | `gp`, `tp` | These registers are reserved and cannot be used as inputs or outputs. |
> | LoongArch | `$r0` or `$zero` | This is a constant zero register which can't be modified. |
> | LoongArch | `$r2` or `$tp` | This is reserved for TLS. |
> | LoongArch | `$r21` | This is reserved by the ABI. |

```rust,compile_fail
# #[cfg(target_arch = "x86_64")] { unsafe{
core::arch::asm!("mov rsp, 5", out("rsp") x);
# }}
# #[cfg(not(target_arch = "x86_64"))] compile_error!("Inline Assembly Tests are not supported off of x86_64");
```

## Template modifiers r

r[asm.template]

r[asm.template.modifier]
An operand spec that refers to a register operand may specify a modifier as part of the format specifier. 

r[asm.template.class]
A format specifier shall only use a modifier that is supported for the register class specified by the register opernd.

> [!TARGET-SPECIFIC]
> The list of supported modifiers for each register class is as follows
> | Architecture | Register class | Modifier | Example output | LLVM modifier |
> | ------------ | -------------- | -------- | -------------- | ------------- |
> | x86-32 | `reg` | None | `eax` | `k` |
> | x86-64 | `reg` | None | `rax` | `q` |
> | x86-32 | `reg_abcd` | `l` | `al` | `b` |
> | x86-64 | `reg` | `l` | `al` | `b` |
> | x86 | `reg_abcd` | `h` | `ah` | `h` |
> | x86 | `reg` | `x` | `ax` | `w` |
> | x86 | `reg` | `e` | `eax` | `k` |
> | x86-64 | `reg` | `r` | `rax` | `q` |
> | x86 | `reg_byte` | None | `al` / `ah` | None |
> | x86 | `xmm_reg` | None | `xmm0` | `x` |
> | x86 | `ymm_reg` | None | `ymm0` | `t` |
> | x86 | `zmm_reg` | None | `zmm0` | `g` |
> | x86 | `*mm_reg` | `x` | `xmm0` | `x` |
> | x86 | `*mm_reg` | `y` | `ymm0` | `t` |
> | x86 | `*mm_reg` | `z` | `zmm0` | `g` |
> | x86 | `kreg` | None | `k1` | None |
> | AArch64 | `reg` | None | `x0` | `x` |
> | AArch64 | `reg` | `w` | `w0` | `w` |
> | AArch64 | `reg` | `x` | `x0` | `x` |
> | AArch64 | `vreg` | None | `v0` | None |
> | AArch64 | `vreg` | `v` | `v0` | None |
> | AArch64 | `vreg` | `b` | `b0` | `b` |
> | AArch64 | `vreg` | `h` | `h0` | `h` |
> | AArch64 | `vreg` | `s` | `s0` | `s` |
> | AArch64 | `vreg` | `d` | `d0` | `d` |
> | AArch64 | `vreg` | `q` | `q0` | `q` |
> | ARM | `reg` | None | `r0` | None |
> | ARM | `sreg` | None | `s0` | None |
> | ARM | `dreg` | None | `d0` | `P` |
> | ARM | `qreg` | None | `q0` | `q` |
> | ARM | `qreg` | `e` / `f` | `d0` / `d1` | `e` / `f` |
> | RISC-V | `reg` | None | `x1` | None |
> | RISC-V | `freg` | None | `f0` | None |
> | LoongArch | `reg` | None | `$r1` | None |
> | LoongArch | `freg` | None | `$f0` | None |

```rust
# #[cfg(target_arch = "x86_64")] { unsafe{
let x: i32 = 5;
let y: i32;
core::arch::asm!("mov {:e}, {:e}", out(reg) y, in(reg) x);
# }}
```

> [!NOTE]
> The supported modifiers are a subset of LLVM's (and GCC's) [asm template argument modifiers][llvm-argmod], but do not use the same letter codes.

> [!NOTE]
> - on ARM `e` / `f`: this prints the low or high doubleword register name of a NEON quad (128-bit) register.
> - on x86: our behavior for `reg` with no modifiers differs from what GCC does.
>   GCC will infer the modifier based on the operand value type, while we default to the full register size.
> - on x86 `xmm_reg`: the `x`, `t` and `g` LLVM modifiers are not yet implemented in LLVM (they are supported by GCC only), but this should be a simple change.

r[asm.template.diagnostic]
A lint diagnostic should be emitted if a modifier is omitted, or a modifier is used such that the modified expanded register is of an inappropriate width for the type used to initialize the operand

[llvm-argmod]: http://llvm.org/docs/LangRef.html#asm-template-argument-modifiers

## Behaviour of an asm block 

r[asm.evaluation]

r[asm.evaluation.general]
Each evaluation of an asm block (invocation of [`core::arch::asm!`]) shall perform an operation that correpsonds to the result of a valid sequence of operations on the Minirust Abstract Machine. The behaviour is undefined if the operations performed by the asm block do not validly correspond to a valid sequence of Minirust operations.

> [!NOTE]
> The operation the asm block performs may differ between evaluations of the same asm block.

> [!TARGET-SPECIFIC]
> The correspondance between the operation performed by the asm block is target-dependant and implementation-dependant, subject to the rules set in [asm.operands]. Unless the program modifies the execution state, the basic operation performed by the asm block is the one performed by executing the sequence of instructions specified in the *expanded asm-string* starting with the first instruction.

r[asm.evaluation.reg-values]
The value of each register mentioned in an input operand is set according to [asm.operands] before evaluating any instructions in the asm block. The value of each other *operand-usable register* is unspecified. The value of all other registers is target-dependant.

> [!NOTE]
> The target may define that the register value (or some portion thereof) is undefined.

r[asm.evaluation.constraints]
Certain constraints may be placed on the asm block, and on the requirements of the correspondance, by default or by an option explicitly specified on the asm block. The behaviour is undefined if any such constraint is violated.

r[asm.evaluation.memory]
The behaviour is undefined if the asm block accesses any allocation, or disables, freezes, or activates any tags, except via:
* An access to a static item which is declared with the `#[no_mangle]` attribute, the `#[export_name]` attribute, or which is visible to an expression within the function in which the asm block is expanded,
* A pointer tag which has been exposed, 
* A pointer tag which was passed as an input operand, or
* A pointer tag which is accessible by reading any memory the asm block can read under this clause, recursively.

r[asm.evaluation.unwind]
The behaviour is undefined if an inline assembly block exits by unwinding from a panic or a foreign exception.

```rust,ignore
// The following snippet has undefined behaviour
extern "C-unwind" fn panics(){panic!("unwind through asm")}
# #[cfg(target_arch = "x86_64")] { unsafe{
core::arch::asm!("call {}", sym panics);
# }}
```


r[asm.evaluation.register-value]
The behaviour is undefined upon exiting an asm block unless the stack pointer register and each operand-usable register not mentioned by an `out` , `lateout`, `inout`, or `inlateout` operand has the value the register held upon entry to the asm block.

> [!TARGET-SPECIFIC]
> In addition to operand-usable registers, certain other registers on a target may require being preserved, or have specific rules regarding the value at exit.
> On x86 and x86-64 targets:
> * The Direction flag (`flags.DF`) is clear upon entry and must be clear upon exit
> * The x87 Stack (that is the `TOP` field of the floating-point status word, and each bit in the floating-point tag word) must be preserved and restored upon exit. If all x87 `st` registers are marked as clobbered, the stack is guaranteed to be empty on entry to the asm block (that is, `TOP` is set to `0x7` and the `ftw` is set to `0xFFFF`).

## Options 

r[asm.options]

r[asm.options.general]
An options-spec provided in the asm invocation places constraints on the assembly block. 

r[asm.options.att_syntax]
The `att_syntax` option may be specfied on the x86 and x86_64 target. The program shall not specify the `att_syntax` option on any other target.

> [!TARGET-SPECIFIC]
> The `att_syntax` option modifies the syntax used to parse the *expanded asm-string* as though the `.att_syntax prefix` directive was issued before parsing the *expanded asm-string*, and modifies the expansion of register operands to include a `%` prefix.

```rust
# #[cfg(target_arch = "x86_64")] { unsafe{
let x: i32;
core::arch::asm!("mov {:e}, %eax", in(reg) 5, out("eax") x, options(att_syntax));
# }}
```

r[asm.options.raw]
The `raw` option may be specified. If the `raw` option is specified, the asm block shall not have any operands, other than explicit register operands, and the `clobbers_abi` special operand. 

> [!NOTE]
> The `raw` option causes the *joined asm-string* to be handled verbatim without being interpreted as a format string and expanded. 


r[asm.options.nomem]
The `nomem` option may be specified. The behaviour is undefined if the assembly block modifies any allocation, disables, freezes, or activates any tag, *synchronizes-with* any other thread of execution or signal handler, and the implementation may assume that the behaviour or outputs of the assembly block does not depend on the contents of any allocation.


```rust,ignore
// The following snippet has undefined behaviour
static mut FOO: i32 = 5;
# #[cfg(target_arch = "x86_64")] { unsafe{
core::arch::asm!("mov dword ptr [{}+rip], 3", sym FOO, options(nomem));
# }}
```

```rust
// The following snippet may have unpredictable results
static mut FOO: i32 = 5;
# #[cfg(target_arch = "x86_64")] { unsafe{
let x: i32;
core::arch::asm!("mov {:e}, dword ptr [{}+rip]", out(reg) x, sym FOO, options(nomem));
# }}
```

r[asm.options.readonly]
The `readonly` option may be specified. The behaviour is undefined if the assembly block modifies any allocation or activates any tag. 

```rust,ignore
// The following snippet has undefined behaviour
static mut FOO: i32 = 5;
# #[cfg(target_arch = "x86_64")] { unsafe{
core::arch::asm!("mov dword ptr [{}+rip], 3", sym FOO, options(readonly));
# }}
```

r[asm.options.exclusive]
The program shall not specify both the `nomem` and `readonly` options.

```rust,compile_fail
# #[cfg(target_arch = "x86_64")] { unsafe{
core::arch::asm!("mov dword ptr [FOO+rip], 3", options(readonly, nomem));
# }}
# #[cfg(not(target_arch = "x86_64"))] compile_error!("Inline Assembly Tests are not supported off of x86_64");
```

r[asm.options.pure]
The `pure` option may be specfied. The evaluation of the assembly block shall not produce any observable behaviour, consume input, or terminate execution, and the implementation may assume that the outputs of the assembly block depends only on the inputs and the contents of any allocation. If the program specifies the `pure` option, it shall specify either the `nomem` or `readonly` option.

```rust,ignore
// The following snippet has undefined behaviour
static mut FOO: i32 = 5;
# #[cfg(target_arch = "x86_64")] { unsafe{
core::arch::asm!("xor edi, edi","call exit@plt", options(pure, readonly));
# }}
```


r[asm.options.nostack]
The `nostack` option may be specified. The implementation may assume that the assembly block does not modify or access the stack, except an allocation placed in that region by the implementation.

> [!TARGET-SPECIFIC]
> The stack is defined by an target-specific register and is a target-specific memory region. It may include a "red zone".
> If the `nostack` option is *not* specified, then the stack pointer is guaranteed to point to memory that can be allocated by the asm block, which is aligned at least as much as is required by the ABI for a function call.

```rust,ignore
// The following snippet has undefined behaviour
# #[cfg(target_arch = "x86_64")] { unsafe{
let x: i32;
core::arch::asm!("push 5", "pop rax", out("eax") x, options(nostack));
# }}
```

r[asm.options.preserve_flags]
The `preserves_flags` option may be specified. The implementation may assume that the value of the status flags are preserved by the assembly block.

> [!TARGET-SPECFIC]
> - These flags registers must be restored upon exiting the asm block if the `preserves_flags` option is set:
> - x86
>   - Status flags in `EFLAGS` (CF, PF, AF, ZF, SF, OF).
>   - Floating-point status word (all).
>   - Floating-point exception flags in `MXCSR` (PE, UE, OE, ZE, DE, IE).
> - ARM
>   - Condition flags in `CPSR` (N, Z, C, V)
>   - Saturation flag in `CPSR` (Q)
>   - Greater than or equal flags in `CPSR` (GE).
>   - Condition flags in `FPSCR` (N, Z, C, V)
>   - Saturation flag in `FPSCR` (QC)
>   - Floating-point exception flags in `FPSCR` (IDC, IXC, UFC, OFC, DZC, IOC).
> - AArch64
>   - Condition flags (`NZCV` register).
>   - Floating-point status (`FPSR` register).
> - RISC-V
>   - Floating-point exception flags in `fcsr` (`fflags`).
>   - Vector extension state (`vtype`, `vl`, `vcsr`).
> - LoongArch
>   - Floating-point condition flags in `$fcc[0-7]`.

```rust,ignore
# #[cfg(target_arch = "x86_64")] { unsafe{
core::arch::asm!("cmp eax, eax", in("eax") 5, options(preserve_flags));
# }}
```

r[asm.options.noreturn]
The `noreturn` option may be specifed. An invocation of the [`core::arch::asm!`] macro that specifies the `noreturn` option expands to an expression of type `!`. The behaviour is undefined if an evaluation of the assembly block exits. The program shall not specify the `clobber_abi` specification, or an operand that is an `out`, `lateout`, `inout`, or `inlateout` operand. 

```rust
# #[cfg(target_arch = "x86_64")]
pub fn main() -> ! {
  unsafe{
    core::arch::asm!("xor edi, edi", "call exit@plt", options(noreturn));
  }
}
```

```rust,ignore
// The following snippet has undefined behaviour
# #[cfg(target_arch = "x86_64")] { unsafe{
core::arch::asm!("", options(noreturn));
# }}
```

```rust,compile_fail
# #[cfg(target_arch = "x86_64")] { unsafe{
let x: i32;
core::arch::asm!("xor edi, edi", "call exit@plt", out("edi") x, options(noreturn));
# }}
# #[cfg(not(target_arch = "x86_64"))] compile_error!("Inline Assembly Tests are not supported off of x86_64");
```

r[asm.options.global]
A program shall not specify an option, other than the `att_syntax` or `raw` options, in an invocation of the [`core::arch::global_asm!`] macro.

```rust,compile_fail,ignore
# #[cfg(target_arch = "x86_64")]
core::arch::global_asm!("", options(noreturn));

# #[cfg(not(target_arch = "x86_64"))] compile_error!("Inline Assembly Tests are not supported off of x86_64");
```

## Directives Support 

r[asm.directives]

r[asm.directives.gen]
The common subset of the LLVM and GNU AS Assembly Syntax used for the *expanded asm-string* is guaranteed to support the following directives
- `.2byte`
- `.4byte`
- `.8byte`
- `.align`
- `.alt_entry`
- `.ascii`
- `.asciz`
- `.balign`
- `.balignl`
- `.balignw`
- `.bss`
- `.byte`
- `.comm`
- `.data`
- `.def`
- `.double`
- `.endef`
- `.equ`
- `.equiv`
- `.eqv`
- `.fill`
- `.float`
- `.global`
- `.globl`
- `.inst`
- `.lcomm`
- `.long`
- `.octa`
- `.option`
- `.p2align`
- `.popsection`
- `.private_extern`
- `.pushsection`
- `.quad`
- `.scl`
- `.section`
- `.set`
- `.short`
- `.size`
- `.skip`
- `.sleb128`
- `.space`
- `.string`
- `.text`
- `.type`
- `.uleb128`
- `.word`

> [!NOTE]
> These directives are generally ones that solely emit sequences of bytes, or that modify the property of symbols.

r[asm.directives.dwarf]

> [!TARGET-SPECIFIC]
> The following Directives are guaranteed to be supported on ELF Targets that use DWARF Debug Information and DWARF Unwind Tables
> - `.cfi_adjust_cfa_offset`
> - `.cfi_def_cfa`
> - `.cfi_def_cfa_offset`
> - `.cfi_def_cfa_register`
> - `.cfi_endproc`
> - `.cfi_escape`
> - `.cfi_lsda`
> - `.cfi_offset`
> - `.cfi_personality`
> - `.cfi_register`
> - `.cfi_rel_offset`
> - `.cfi_remember_state`
> - `.cfi_restore`
> - `.cfi_restore_state`
> - `.cfi_return_column`
> - `.cfi_same_value`
> - `.cfi_sections`
> - `.cfi_signal_frame`
> - `.cfi_startproc`
> - `.cfi_undefined`
> - `.cfi_window_save`

r[asm.directives.seh]

> [!TARGET-SPECIFIC]
> The following directives are guaranteed to be supported on platforms that use Structured Exception Handling
> - `.seh_endproc`
> - `.seh_endprologue`
> - `.seh_proc`
> - `.seh_pushreg`
> - `.seh_savereg`
> - `.seh_setframe`
> - `.seh_stackalloc`


r[asm.directives.x86]

> [!TARGET-SPECIFIC]
> The following directives are guaranteed to be supported on x86 and x86-64 platforms
> - `.nops`
> - `.code16`
> - `.code32`
> - `.code64`
> Use of `.code16`, `.code32`, and `.code64` directives are only supported if the state is reset to the default before exiting the assembly block.
> 32-bit x86 uses `.code32` by default, and x86_64 uses `.code64` by default.

r[asm.directives.arm]

> The following directives are guaranteed to be supported on 32-bit ARM platforms
> - `.even`
> - `.fnstart`
> - `.fnend`
> - `.save`
> - `.movsp`
> - `.code`
> - `.thumb`
> - `.thumb_func`
