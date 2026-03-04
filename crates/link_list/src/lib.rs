#[derive(Debug, PartialEq)]
pub struct Node<T> {
    /// データ
    data: T,
    /// 後続ノードへのポインタ
    next: Option<Box<Node<T>>>,
}

pub struct LinkList<T> {
    /// 先頭ノードへのポインタ
    head: Option<Box<Node<T>>>,
    /// 着目ノードへのポインタ
    current: Option<Box<Node<T>>>,
}

impl<T> LinkList<T> {
    /// 初期化します。
    pub fn new() -> LinkList<T> {
        LinkList {
            head: None,
            current: None,
        }
    }

    /// 先頭にノードを挿入します。
    pub fn insert_front(&mut self, item: T) {
        // take()は所有権ごと取り出すことができる。
        if let Some(front) = self.head.take() {
            let node = Node {
                data: item,
                next: Some(front)
            };
            // take()でNoneになったポインタを新しいノードに書き換える。
            self.head = Some(Box::new(node));
        } else {
            // 追加するノード
            let node = Node {
                data: item,
                next: None,
            };
            self.head = Some(Box::new(node));
        }
    }

    /// 末尾にノードを挿入します。
    pub fn insert_back(&mut self, item: T) {
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }

        let node = Node {
            data: item,
            next: None,
        };
        *current = Some(Box::new(node));
    }

    /// 先頭のノードの要素を取得します。
    pub fn get_front(&self) -> Option<&T> {
        if let Some(node) = &self.head {
            Some(&node.data)
        }else{
            None
        }
    }

    // 末尾のノードの要素への参照を取得します。
    pub fn get_back(&self) -> Option<&T> {
        let mut current = self.head.as_deref();
        while let Some(node) = current {
            match node.next.as_deref() {
                Some(next) => current = Some(next),
                None => break,
            }
        }

        current.map(|node| &node.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize() {
        let list: LinkList<i32> = LinkList::new();
        assert_eq!(list.current, None);
        assert_eq!(list.head, None);
    }

    #[test]
    fn insert_front(){
        let mut list = LinkList::<i32>::new();
        list.insert_front(1);
        assert_eq!(list.get_front(), Some(&1));
        list.insert_front(2);
        assert_eq!(list.get_front(), Some(&2));
    }

    #[test]
    fn insert_back(){
        let mut list = LinkList::<i32>::new();
        list.insert_back(1);
        assert_eq!(list.get_front(), Some(&1));
        list.insert_back(2);
        assert_eq!(list.get_front(), Some(&1));
        assert_eq!(list.get_back(), Some(&2));
    }
}
