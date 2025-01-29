# Rust Undefined Behavior Example

This repository demonstrates a common source of undefined behavior in Rust: modifying a vector's contents through a raw pointer without ensuring the capacity is sufficient.  The example showcases how this can lead to unexpected crashes or data corruption.

## Bug Description

The `bug.rs` file contains Rust code that attempts to modify a vector using a raw pointer obtained from `as_mut_ptr()`.  This is dangerous because it bypasses Rust's memory safety mechanisms.  If the value being written is larger than the existing capacity, it will cause memory corruption.

## Solution

The `bugSolution.rs` file provides a corrected version.  It demonstrates safe ways to modify vectors in Rust, emphasizing the importance of using safe methods provided by the standard library instead of directly manipulating memory with raw pointers.