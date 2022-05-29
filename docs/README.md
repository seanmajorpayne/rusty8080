# RUSTY 8080

An intel 8080 CPU emulator built in Rust. Mainly built as a personal exploration into the Rust language. Please do
not use this for any illegal purposes.

## Notes

Since Rust is particular about the size of integer variables, I've allocated 16 bits for each 8 bit register.
This allows for easier overflow and shifting operations that C would normally be able to handle.

It would have been simple to use a long match to handle all the opcodes, but I've opted to use
function pointers instead. There is nothing inherently wrong with using match since the LLVM would optimize for easy routing,
but making structs for the Instructions seemed like a better solution that would apply to more complex emulators.

## Contributions

Pull requests and reporting issues are welcome, although I don't anticipate many people will find this. :smile:
