To build, run...

```cargo rustc --release -- -C link-arg=-nostartfiles --emit asm```

To execute, run...

```target/release/no-std-printf```

strace.txt contains the output from strace after running this program on my machine - strace allows you to see what syscalls a program is making.
