struct LinkedList<T> {
    next: Option(Box<LinkedList<T>>),
    value: T
}

impl LinkedList<T: SomeTrait> {
    fn search(node: LinkedList<T>, target: T) -> Option(LinkedList<T>) {
        if node.value == target { Option(node) } else { search(node.next) }
    }
}