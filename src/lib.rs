pub mod link_list {
    #[derive(Debug)]
    pub struct Node {
        pub value: i32,
        pub next: Option<Box<Node>>,
    }

    impl Node {
        pub fn new(value: i32) -> Node {
            Node { value, next: None }
        }
        pub fn length(&self) -> usize {
            let mut current_node = self;
            let mut count = 1;
            while let Some(ref next_node) = current_node.next {
                current_node = next_node;
                count += 1;
            }
            count
        }
        // 判断索引是否合法
        pub fn is_valid_index(&self, i: usize,msg :&str) {
            if i < 1 || i > self.length() {
                panic!("{}位置不合法",msg);
            }
        }
        // 搜索元素是否存在，返回索引，不存在返回None
        pub fn search(&self, value: i32) -> Option<i32> {
            let mut current_node = self;
            let mut index = 1;
            // 首节点比较麻烦，单独处理，用头结点更好
            if current_node.value == value {
                return Some(index);
            }

            while let Some(ref next_node) = current_node.next {
                index += 1;
                if next_node.value == value {
                    return Some(index);
                }
                current_node = next_node;
            }
            None
        }
        // 添加一个节点
        pub fn append(&mut self, value: i32) {
            // 创建一个新的节点
            let new_node = Node::new(value);
            // 移动到最后一个节点
            let mut current_node = self;
            // ref用于模式匹配中绑定引用
            while let Some(ref mut next_node) = current_node.next {
                current_node = next_node;
            }
            // 添加新的节点
            current_node.next = Some(Box::new(new_node));
        }

        // 删除指定位置的节点
        pub fn remove(&mut self, i: usize) {
          // 判断删除位置是否合法
          self.is_valid_index(i,"删除");
            let length = self.length();
            let mut pre_node = self;
            let mut index = if i > 1 { 1 } else { 0 };
            // 首节点比较麻烦，单独处理，用头结点更好
            for _ in 0..length {
                if index == i - 1 {
                    // 删除当前节点
                    pre_node.next = pre_node.next.as_mut().unwrap().next.take();
                    break;
                } else {
                    pre_node = pre_node.next.as_mut().unwrap();
                    index += 1;
                }
            }
        }

        // 指定位置插入节点
        pub fn insert(&mut self, i: usize, value: i32) {
            // 判断插入位置是否合法
            self.is_valid_index(i,"插入");

            let length = self.length();
            let mut pre_node = self;
            let mut new_node = Node::new(value);

            // 判断是否是第一个节点，调整index
            let mut index = if i > 1 { 1 } else { 0 };

            for _ in 0..length + 1 {
                if index == i - 1 {
                    // 插入新节点
                    // 新节点的next指向指定位置节点的next
                    new_node.next = pre_node.next.take();
                    // 将前一个节点next指向新节点
                    pre_node.next = Some(Box::new(new_node));
                    break;
                } else {
                    pre_node = pre_node.next.as_mut().unwrap();
                    index += 1;
                }
            }
        }
        
    }
}
