# Language rules

Clauses within the Reference are labeled with a named *rule*. This provides the ability to link and refer to individual clauses and to [link to the `rustc` test suite](test-annotations.md).

## Rule labels

Most clauses should be preceded by a rule label. A rule label should be on a line by itself and should look like this:

```markdown
r[foo.bar]
```

The rule name should be lowercase, with periods separating components from most general to most specific (e.g., `r[array.repeat.zero]`).

Rules can be linked to by their ID using Markdown such as `[foo.bar]`. There are [automatic link references] so that any rule can be referred to from any page in the book.

In the HTML, the rules are clickable, just like headers.

## Rule guidelines

When assigning rules to new paragraphs or modifying rule names, use the following guidelines:

1. A rule applies to one core idea, which should be easily determined when reading the paragraph it is applied to.
2. Other than the "intro" paragraph, purely explanatory, expository, or exemplary content does not need a rule. If the expository paragraph isn't directly related to the previous one, separate it with a hard (rendered) line break.
   - This content will be moved to `[!NOTE]` or more specific admonitions in the future.
3. Rust code examples and tests do not need their own rules.
4. Use the following guidelines for admonitions:
   - Notes: Do not include a rule.
   - Warning: Omit the rule if the warning follows from the previous paragraph or if the warning is explanatory and doesn't introduce any new rules.
   - Target-specific behavior: Always include the rule.
   - Edition differences: Always include the rule.
5. The following keywords should be used to identify paragraphs when unambiguous:
   - `intro`: The beginning paragraph of each section. It should explain the construct being defined overall.
   - `syntax`: Syntax definitions or explanations when BNF syntax definitions are not used.
   - `namespace`: For items only, specifies the namespace(s) the item introduces a name in. It may also be used elsewhere when defining a namespace (e.g., `r[attribute.diagnostic.namespace]`).
6. When a rule doesn't fall under the above keywords, or for section rule IDs, name the subrule as follows:
   - If the rule names a specific Rust language construct (e.g., an attribute, standard library type/function, or keyword-introduced concept), use the construct as named in the language, appropriately case-adjusted (but do not replace `_`s with `-`s).
   - Other than Rust language concepts with `_`s in the name, use `-` characters to separate words within a "subrule".
   - Whenever possible, do not repeat previous components of the rule.
   - Edition differences admonitions should typically be named by the edition where the behavior changed. You should be able to correspond the dates to the chapters in <https://doc.rust-lang.org/edition-guide/>.
   - Target-specific admonitions should typically be named by the least specific target property to which they apply (e.g., if a rule affects all x86 CPUs, the rule name should include `x86` rather than separately listing `i586`, `i686`, and `x86_64`. If a rule applies to all ELF platforms, it should be named `elf` rather than listing every ELF OS).
   - Use an appropriately descriptive, but short, name if the language does not provide one.

[automatic link references]: ../links.md#rule-links
