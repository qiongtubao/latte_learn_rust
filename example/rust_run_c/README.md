先创建c的静态库
```
//编译math.o
gcc -c -Wall -Werror -fpic math.c
//math.o -> libmath.a
ar rcs libmath.a math.o
```

编译rust
```
rustc -L . ./src/main.rs
```