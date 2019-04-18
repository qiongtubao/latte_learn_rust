# 宏

1. 类似于vec!这样的宏
    用法的 （以vec!为例）
    ```rust
        let x:Vec<u32> = vec![1,2,3];
    ```
    功能类似于下面的代码
    ```rust
        let x:Vec<i32> = {
            let mut temp_vec = Vec::new();
            temp_vec.push(1);
            temp_vec.push(2);
            temp_vec.push(3);
            temp_vec
        }
    ```
    宏的写法
    ```
        macro_rules! vec {
            ( $( $x:expr ),* ) => {
                {
                    let mut temp_vec = Vec::new();
                    $(
                    temp_vec.push($x);
                    )*
                    temp_vec
                }
            };
        }

    ```


参考:
    [rust宏中文教程](http://wiki.jikexueyuan.com/project/rust/macros.html)