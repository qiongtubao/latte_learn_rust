
# 基础类型

1. 布尔型 (bool)

```
   let a: bool = true;
```

2. 整型

| 类型 | 大小                                      | 字节        | 备注         |
| ---- | ----------------------------------------- | ----------- | ------------ |
| u8   | 0~255                                     | 1           | 8bit无符号   |
| u16  | 0~65535                                   | 2           | 16bit无符号  |
| u32  | 0~4294967295                              | 4           | 32bit无符号  |
| u64  | 0~18446744073709551615                    | 8           | 64bit无符号  |
| u128 | 0~340282366920938463463374607431768211455 | 16          | 128bit无符号 |
| i8   | -128~127                                  | 1           | 8bit有符号   |
| i16  | -2^15 ~2^15 -1                            | 16bit有符号 |
| i32  | -2^31 ~ 2^31 -1                           | 32bit有符号 |
| i64  | -2^63 ~ 2^63 -1                           | 64bit有符号 |

编译警告 溢出

```
 --> src/main.rs:2:17
  |
2 |     let a: i8 = 128;
  |                 ^^^
  |
  = note: #[warn(overflowing_literals)] on by default
```


3. 浮点数

| 类型 | 字节 | 备注       |
| ---- | ---- | ---------- |
| f32  | 4    | 32位浮点数 |
| f64  | 8    | 64位浮点数 |

4. 字符串

char
String

5. array -> vec
  ```
    let array = vec![1,2,3];
  ```
6. map -> std::collections::HashMap
 
7. struct
  ```
    struct studentA {
      name: String,
      age: i8
    }

    struct studentB(String, i8);
  ```
 8. tuple

  ```
  
  ```
9. enum 

```
  enum Option<T> {
    None,
    Some(T)
  }
```