use crate::ItemStoreType;
use std::collections::{BTreeMap, HashMap};
use std::ops::{Add, Sub};

pub struct Menu {
    item_list: Vec<MenuItem>,
    pub storage: BTreeMap<String, (MenuItem, u32)>, //Vec<(MenuItem, ItemStoreType, u8)>,
}
impl Menu {
    pub fn new(item_list: Vec<MenuItem>, storage: BTreeMap<String, (MenuItem, u32)>) -> Menu {
        Self { item_list, storage }
    }
    fn create_item(&mut self, item: MenuItem) {
        self.item_list.push(item);
    }
    pub fn add_item(&mut self, item_to_add: String, amount: u32) -> Result<String, ()> {
        match self.storage.get(&item_to_add).unwrap().0.store_type {
            ItemStoreType::Count => {
                let item = self
                    .storage
                    .entry(item_to_add)
                    .and_modify(|v| {v.1 += amount});
            }
            ItemStoreType::Liters => {
                let item = self
                    .storage
                    .entry(item_to_add)
                    .and_modify(|v| {v.1 += amount/1000});
            }
            ItemStoreType::Grams => {
                let item = self
                    .storage
                    .entry(item_to_add)
                    .and_modify(|v| {v.1 += amount/1000});
            }
        }

        Ok("PLEASE WORK".to_string())
        // let item = self.storage.get_mut(&item_to_add);
        // match item {
        //     Some(mut item) => match item.0.store_type {
        //         ItemStoreType::Count => {
        //             item.1 += amount;
        //             Ok(format!("New amount: {}", item.1))
        //         }
        //         ItemStoreType::Grams => {
        //             item.1 += amount / 1000;
        //             Ok(format!("New amount: {}", item.1))
        //         }
        //         ItemStoreType::Liters => {
        //             item.1 += amount / 1000;
        //             Ok(format!("New amount: {}", item.1))
        //         }
        //     },
        //     None => Ok("Item not found".to_string()),
        // }
    }
    pub fn remove_item(&mut self, item_to_remove: String, amount: u32) -> Result<String, ()> {
        match self.storage.get(&item_to_remove).unwrap().0.store_type {
            ItemStoreType::Count => {
                let item = self
                    .storage
                    .entry(item_to_remove)
                    .and_modify(|v| {v.1 -= amount});
            }
            ItemStoreType::Liters => {
                let item = self
                    .storage
                    .entry(item_to_remove)
                    .and_modify(|v| {v.1 -= amount/1000});
            }
            ItemStoreType::Grams => {
                let item = self
                    .storage
                    .entry(item_to_remove)
                    .and_modify(|v| {v.1 -= amount/1000});
            }
        }
        Ok("PLEASE WORK".to_string())
        // let item = self.storage.get_mut(&item_to_remove);
        // match item {
        //     Some(item) => match item.0.store_type {
        //         ItemStoreType::Count => {
        //             item.1 -= amount;
        //             Ok(format!("New amount: {}", item.1))
        //         }
        //         ItemStoreType::Grams => {
        //             item.1 -= amount / 1000;
        //             Ok(format!("New amount: {}", item.1))
        //         }
        //         ItemStoreType::Liters => {
        //             item.1 -= amount / 1000;
        //             Ok(format!("New amount: {}", item.1))
        //         }
        //     },
        //     None => Ok("Item not found".to_string()),
        // }
    }
}
pub struct MenuItem {
    name: String,
    description: String,
    price: u8,
    id: u8,
    pub store_type: ItemStoreType,
    amount: Option<u8>,
}
impl MenuItem {
    pub fn new(
        name: String,
        description: String,
        store_type: ItemStoreType,
        price: u8,
        id: u8,
    ) -> MenuItem {
        Self {
            name,
            store_type,
            description,
            price,
            id,
            amount: None,
        }
    }
}
