**The Carbon Programming Language**

# carbon-lang_compiler

## About current branch

### ⛏🚧 This branch is set up to renew the Relocation process.

### Issue content

#### Present situation

We are using absolute address when relocating jumps.

#### What problems it could lead to

We need to relocate every single jumps when combining multiple compiled packages. Also, it's hard to locate the problem when debugging the compiler itself.

#### Solution

Use relative address when relocating the target jumps.

## Changelog

There's no changelog so far.
