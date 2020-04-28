use link_list::LinkList::LinkList;

fn main() {
    let r = LinkList::new(5);
    r.push_back(6);
    println!("{}", r.get_tail().unwrap());
}
