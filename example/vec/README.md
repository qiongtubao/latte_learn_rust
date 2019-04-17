#数组类型

1. 固定数组
   ```
     let a: [i32:4] = [1,2,3,4];
   ```
   并不会有删除添加等操作  只有更新

2. 动态数组

常用以下
* push: 尾部追加
* pop: 尾部删除
* swap: 交换特定的两个值
* reverse：逆序
* get_mut: 获得值
* sort: 排序
* sort_by: 自定义排序
* insert: 插入
* remove: 删除
* retain: 过滤
* len : 长度
* is_empty: 是否为空
* append: 合并
* drain: 拆分
* clear: 清空
* truncate: 截取
* extend: 扩展  (和合并的区别是参数类型不同)