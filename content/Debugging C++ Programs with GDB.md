
If you don't use GDB but program in C/C++, you are missing out on a powerful debugging tool. Here's a quick 5-minute tutorial to get you started.

## Step 1: Compile with Debug Symbols

Compile your program using the `-ggdb` flag to include debug symbols for GDB.

```bash
# For C++
g++ main.cpp -o main -ggdb

# For C
gcc main.c -o main -ggdb
```

## Step 2: Run GDB

Now, run GDB from the shell, pointing it to your compiled executable.

```bash
gdb ./main
```

(Note: If you aren't using Linux, at least consider using WSL (Windows Subsystem for Linux) for a better debugging experience.)

You will see the `(gdb)` prompt.

## Step 3: Basic GDB Commands

- Type `run` to execute your program.
- Use `break` followed by a line number or function name to set breakpoints.
- Type `n` to step through the code line by line.
- Type `s` to step into a function.

## Step 4: Visualize Code with `layout split`

One helpful feature is the split layout, which displays both the source code and GDB commands simultaneously.

```bash
layout split
```

or to skip the assembly view

```bash
layout src
```

## Step 5: Inspecting Variables

Use the `print` and `display` commands in GDB to inspect variables and expressions during debugging.

For example:

```bash
print variable_name
display expression
```

These commands help you check if anything is unexpected in your program.

Now go practice!!!

