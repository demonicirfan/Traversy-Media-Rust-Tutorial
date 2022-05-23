# RUST TUTORIAL

Cargo - It is the rust package manager just like npm or yarn.
        We can also use cargo for installing rust binaries that can be used system wide.
Crates - Packages in rust are called crates 
Crates.io - It is the central registry for crates like we have npm registry

# Manual memory management in rust
Stack - it is a special region of the process that stores variables created by each functions.
        The size of every variable in stack has to be known at compile time.
        When a function exits its stack is deleted so we do not have to manage memory

Heap - It is a region of memory which is not automatically allocated
        We have to manage it by our own
        It has no memory limitations
        It is assisible by any function anywhere in the program
        
