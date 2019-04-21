创建c静态库
```
 rustc --crate-type=staticlib ./src/lib.rs
```

会发现产生一个liblib.a文件

之后在编译main.c
```
gcc -o main main.c -L.  -llib -Wl -lpthread -ldl
```