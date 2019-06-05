### specialization

#### nightly



所有范型

```rust
impl<T> Example for T {
    default fn call(&self) {
        println!("most generic");
    }
}
```

特殊范型

```
impl Example for str {
    fn call(&self) {
        println!("specialized for str, {}", self);
    }
}
```



需要添加

```rust
#![feature(specialization)]
```

