# pls

A pretty LS-like screen for when you cd.

Running `pls` clears the screen and lists files and directories in the current directory, with icons and color information. 

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
