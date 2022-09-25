**The Carbon Programming Language**

# carbon-lang_compiler

## About current branch

### ⛏🚧 This branch is set up to resolve issue `HS-YT-TCPL-14`

### Issue content

#### Present situation

We use an absolute address to navigate to the function we want to call.

#### What problems it could lead to

It is OKAY when we only build a single code file because it is just simple, but things get a bit complex when we want to combine multiple packages, like package linking. When using absolute address navigation, we have to change every jump command to fit the linked package.

#### Solution

Create a function table to save the address of every function and allocate every function an id. When we want to call a function, we just need to use its id but not the absolute address.

Through this method, not only can we place function entry address to function table, but also put more properties such as parameter count and variable count into it.



## Changelog

There's no changelog so far.
