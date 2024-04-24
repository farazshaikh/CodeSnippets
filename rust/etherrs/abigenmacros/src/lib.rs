use proc_macro::TokenStream;

#[proc_macro]
pub fn my_fn_like_proc_macro(input: TokenStream) -> TokenStream {
    // 1. Use syn to parse the input tokens into a syntax tree.
    // 2. Use quote to generate new tokens based on what we parsed.
    // 3. Return the generated tokens.
    println!("{input:?}");
    input
}

#[proc_macro_derive(MyDerive)]
pub fn my_derive_proc_macro(input: TokenStream) -> TokenStream {
    // 1. Use syn to parse the input tokens into a syntax tree.
    // 2. Generate new tokens based on the syntax tree. This is additive to the `enum` or
    //    `struct` that is annotated (it doesn't replace them).
    // 3. Return the generated tokens.
    input
}

#[proc_macro_attribute]
pub fn log_entry_and_exit(args: TokenStream, input: TokenStream) -> TokenStream {
    // 1. Use syn to parse the args & input tokens into a syntax tree.
    // 2. Generate new tokens based on the syntax tree. This will replace whatever `item` is
    //    annotated w/ this attribute proc macro.
    // 3. Return the generated tokens.
    input
}
