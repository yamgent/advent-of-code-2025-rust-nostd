# advent-of-code-2025-rust-nostd

Based on https://github.com/yamgent/advent-of-code-2025-rust, but with the added
challenge of `no_std`.

Constraints for the `no_std` challenge:

* `no_std` enforced in "userland" and "nostdlib".
* When in `no_std`, cannot use global allocator to reactivate the `alloc` crate.
* "nostdlib" cannot panic when encountering OOM, it must just return error.
  However, "userland" is allowed to panic / unwrap / expect for OOM whenever it
  pleases
* "nostdlib" should not rely on `drop()` being called.
