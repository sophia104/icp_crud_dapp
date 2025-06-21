// src/icp_crud_dapp/main.rs
use candid::{candid_method, Deserialize, Serialize};
use ic_cdk::api::stable::{stable_restore, stable_save};
use ic_cdk_macros::*;
use std::cell::RefCell;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct Item {
    id: u64,
    content: String,
}

thread_local! {
    static ITEMS: RefCell<Vec<Item>> = RefCell::new(Vec::new());
}

#[update(name = "createItem")]
#[candid_method(update)]
fn create_item(content: String) -> Item {
    ITEMS.with(|items| {
        let mut items = items.borrow_mut();
        let id = items.len() as u64 + 1;
        let item = Item { id, content };
        items.push(item.clone());
        item
    })
}

#[query(name = "getItems")]
#[candid_method(query)]
fn get_items() -> Vec<Item> {
    ITEMS.with(|items| items.borrow().clone())
}

#[update(name = "updateItem")]
#[candid_method(update)]
fn update_item(id: u64, new_content: String) -> Option<Item> {
    ITEMS.with(|items| {
        let mut items = items.borrow_mut();
        if let Some(item) = items.iter_mut().find(|i| i.id == id) {
            item.content = new_content.clone();
            return Some(item.clone());
        }
        None
    })
}

#[update(name = "deleteItem")]
#[candid_method(update)]
fn delete_item(id: u64) -> bool {
    ITEMS.with(|items| {
        let mut items = items.borrow_mut();
        let len_before = items.len();
        items.retain(|i| i.id != id);
        len_before != items.len()
    })
}

#[pre_upgrade]
fn pre_upgrade() {
    let data = ITEMS.with(|items| items.borrow().clone());
    stable_save((data,)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (data,): (Vec<Item>,) = stable_restore().unwrap();
    ITEMS.with(|items| {
        *items.borrow_mut() = data;
    });
}
