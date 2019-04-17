use std::vec::Vec;
// use std::iter::Cloned;

/**
 * vec![1,2,3,4]
 * vec!(1,2,3,4);
 * vec![0;5];
 */
/**
 * 动态扩充数组
 */
// fn vec() {
    
//     let mut array:Vec<i32> = Vec::new();
//     array.push(2); //末尾追加
//     array.push(1);
//     assert_eq!(array.len(), 2);  //len 获取长度
//     assert_eq!(array.pop(), Some(1)); //pop移出最后一个对象并返回  Some<T>
//     assert_eq!(array.len(), 1);  //len 获取长度
//     array.extend(vec![3,4,5]); //数组追加
//     // let mut array2: Cloned<i32> = array.iter().cloned();
//     //Cloned { it: Iter([2, 3, 4, 5]) }
//     for x in &array { //遍历数组对象
//         println!("{}", x);
//     }
//     array[3] = 10;
    
//     // let vec = vec![0; 5];
//     // assert_eq!(vec, [0, 0, 0, 0, 0]);
//     //容量大小。1<<5
//     //如果向量的长度超过其容量，则其容量将自动增加，但其元素必须重新分配。
//     let mut vec1:Vec<i32> = Vec::with_capacity(1<<5);
//     assert_eq!(vec1.len(), 0);
//     vec1.resize(3, 99);
//     //append 和 extend的区别是 &mut 类型
//     vec1.append(&mut vec![1,5]);
//     //更安全的获取对象 
//     if let Some(elem) = vec1.get_mut(1) {
//         *elem = 996;
//     }
//     println!("Hello, world! {:?}", vec1);
//     let mut v = ["a", "b", "c", "d"];
//     v.swap(1, 3);
//     assert!(v == ["a", "d", "c", "b"]);
//     v.reverse();
//     assert!(v == ["b", "c", "d", "a"]); 
//     let x = &mut [1, 2, 4];
//     for elem in x.iter_mut() {
//         *elem += 2;
//     }
//     let slice = [10, 40, 33, 20, 36, 41];
//     let mut iter = slice.split(|num| {
//         println!("number: {:?}", num);
//         num % 3 == 0
//     });
//     loop {
//         let n = iter.next();
//         if(n.is_none()) {
//             break;
//         }
//         println!("next {:?}", n.unwrap());
//     }
//     let v = [10, 40, 30];
//     assert!(v.contains(&30));
//     assert!(!v.contains(&50));
//     let v = [10, 40, 30];
//     assert!(v.starts_with(&[10, 40])); 
//     assert!(v.starts_with(&[]));
//     assert!(v.ends_with(&[30]));

//     let mut v = [-5, 4, 1, -3, 2];
//     //从小到大
//     v.sort();
//     //从大到小
//     let mut v = [-5i32, 4, 1, -3, 2];

//     v.sort_by_key(|k| k.abs());
//     assert!(v == [1, 2, -3, 4, -5]);
    
//     let s = [10, 40, 30];
//     let x = s.to_vec();
//     println!("Hello, world! {:?}", x==s);
//     let s: Box<[i32]> = Box::new([10, 40, 30]);
//     let x = s.into_vec();
//     // `s` cannot be used anymore because it has been converted into `x`.

//     assert_eq!(x, vec![10, 40, 30]);
//     let mut v = vec!["a", "b", "c"];
//     v.insert(1, "d");//在第1个序列号上insert 元素"d"
//     assert_eq!(v, vec!["a", "d", "b", "c"]);
//     // 值相等，再次举例
//     assert_eq!(v, ["a", "d", "b", "c"]);
//     let mut v = vec!["a", "b", "c"];
//     assert_eq!(v.remove(1), "b");

//     //过滤
//     let mut vec = vec![1, 2, 3, 4];
//     vec.retain(|&x| x%2 == 0);
//     assert_eq!(vec, [2, 4]);

//     //抽离数据
//     let mut v = vec![1, 2, 3];
//     let u = v.drain(1..).collect();
//     assert_eq!(v, &[1]);
//     assert_eq!(u, &[2, 3]);
//     //截取前n个之前
//     let mut n =3;
//     let mut vec = vec!["a", "b", "c", "d", "e"];
//     vec.truncate(n);//取0，1，2序列值
//     assert_eq!(vec, ["a", "b", "c"]);
// }

fn vec() {
    let mut vec:Vec<i32> = Vec::new();
    assert!(vec.is_empty());
    //push 追加
    vec.push(1<<4);
    vec.push(1<<3);
    assert_eq!(vec, [16,8]);
    assert_eq!(vec.len(), 2);
    vec.sort();
    assert_eq!(vec, [8,16]);
    vec[0] = 19;
    assert_eq!(vec, [19,16]);
    //删除并返回最后一个值
    let a = vec.pop();
    assert_eq!(a, Some(16));
    assert_eq!(vec, [19]);
    vec.extend(vec![1,2,3,4,5]);
    assert_eq!(vec, [19,1,2,3,4,5]);
    //获取
    assert_eq!(vec[1], 1);
    //建议使用这种方式  健壮性
    if let Some(x) = vec.get_mut(1) {
        assert_eq!(*x, 1);
    }
    vec.swap(1,3);
    assert_eq!(vec, [19,3,2,1,4,5]);
    vec.reverse();
    assert_eq!(vec, [5,4,1,2,3,19]);
    //遍历
    for elem in vec.iter_mut() {
        *elem += 2;
    }
    assert_eq!(vec, [7, 6, 3, 4, 5, 21]);
    //是否存在
    assert!(vec.contains(&3));
    //插入
    vec.insert(2,99);
    assert_eq!(vec, [7, 6, 99, 3, 4, 5, 21]);
    //删除
    vec.remove(3);
    assert_eq!(vec, [7, 6, 99, 4, 5, 21]);
    //过滤
    vec.retain(|&x| x%2 == 0);
    assert_eq!(vec, [6,4]);
    vec.sort();
    assert_eq!(vec, [4,6]);
    let mut a = vec![9,1,8,7,17];
    vec.append(&mut a);
    //合并  通过&mut 数组
    assert_eq!(vec, [4, 6, 9, 1, 8, 7, 17]);
    assert_eq!(a, []);

    let b :Vec<_> = vec.drain(4..).collect();
    assert_eq!(b, [8,7,17]);
    assert_eq!(vec, [4,6,9,1]);

    vec.truncate(3);

    assert_eq!(vec, [4,6,9]);
    //倒排序
    vec.sort_by(|x,y| y.cmp(x));
    assert_eq!(vec, [9,6,4]);

    vec.clear();
    assert_eq!(vec, []);
}
fn array() {
    let mut array: [i32;4] = [1,2,3,4];
    // array.push(5);  Error 定长数组不能push
    array[2] = 5;
    assert_eq!(array, [1,2,5,4]);
}
fn main() {
    vec();
    array();
} 
