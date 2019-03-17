# Limits

The following [attributes] affect compile-time limits.

## The `recursion_limit` attribute

The *`recursion_limit` attribute* may be applied at the crate level to set the
maximum depth for potentially infinitely-recursive compile-time operations
like auto-dereference or macro expansion. It uses the [_MetaNameValueStr_]
syntax to specify the recursion depth. The default is
`#![recursion_limit="64"]`.

[attributes]: attributes.html
[_MetaNameValueStr_]: attributes.html#meta-item-attribute-syntax
