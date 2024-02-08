use link_rs::link_list::Node;

fn main() {
    let mut node = Node::new(1);
    println!("Node value: {:#?}", node);

    node.append(2);
    node.append(3);
    println!("Node value: {:#?}", node);
    println!("Node length: {:#?}", node.length());

    node.insert(4, 9);
    println!("Node value: {:#?}", node);
    println!("节点长度: {}", node.length());

    node.remove(2);
    println!("Node value: {:#?}", node);

    println!("节点长度: {}", node.length());

    println!("节点位于第{}个元素", node.search(0).unwrap_or(-1));

    node.insert(8, 99);
}
