## Setup

### Windows

Create a batch file somewhere in your PATH:

```batch
@echo off
cd %1
pls
```

### Linux

Create a function in your `.bashrc` or somewhere else that's sourced:

```bash
function cd() {
  builtin cd $1
  pls
}
```
