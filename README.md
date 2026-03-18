# Sorrel

## To Do

- Create compat module with traits/impls for converting to/from:
    - proc-macro
    - proc-macro2
- Add missing composite syntax:
    - Punctuated
    - etc...
- Add Extend impls to Stream.
- impl ToStream for any type that impls proc_macro::ToTokens as compat bridge.
- impl ToStream to common types like:
    - Box<T> (and any other points)
    - Option<T>
    - Result<T, E>
