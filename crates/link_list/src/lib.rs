#[derive(Debug, PartialEq)]
pub struct Node<T> {
    /// データ
    data: T,
    /// 後続ノードへのポインタ
    next: Option<Box<Node<T>>>,
}

pub struct LinkList<T> {
    /// 先頭ノードへのポインタ
    head: Option<Node<T>>,
    /// 着目ノードへのポインタ
    current: Option<Node<T>>,
}

impl<T> LinkList<T> {
    pub fn new() -> LinkList<T> {
        LinkList {
            head: None,
            current: None,
        }
    }

    pub fn insert_front(&mut self, item: T) {
        // take()は所有権ごと取り出すことができる。
        if let Some(front) = self.head.take() {
            let node = Node {
                data: item,
                next: Some(Box::new(front))
            };
            // take()でNoneになったポインタを新しいノードに書き換える。
            self.head = Some(node);
        } else {
            // 追加するノード
            let node = Node {
                data: item,
                next: None,
            };
            self.head = Some(node);
        }
    }

    /// 先頭のノードの要素を取得します。
    pub fn get_first(&self) -> Option<&T> {
        if let Some(node) = &self.head {
            Some(&node.data)
        }else{
            None
        }
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
        assert_eq!(list.get_first(), Some(&1));
        list.insert_front(2);
        assert_eq!(list.get_first(), Some(&2));
    }
}
