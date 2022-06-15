# gnu-libjit

Safe and exceedingly performant wrapper around [gnu-libjit](https://www.gnu.org/software/libjit/). A just in time compiler with it's
own IR.

It's possible to jit a small program and execute it in <5ms compared to ~80ms with llvm.

**Warning**: This wrapper contains only exactly the functionality from libjit I needed for my own project. You may find it to be missing a piece of functionality. It shouldn't be too hard to add it yourself (:

# License
See LICENSE