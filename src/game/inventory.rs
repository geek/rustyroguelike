use super::{Item, gui, gui::ItemMenuResult, State, TickType, BaseEntity, player};
use crate::rltk;
use rltk::Rltk;
extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Inventory {
    pub items : Vec<Item>,
    capacity: i32
}

impl Inventory {
    pub fn new(capacity : i32) -> Inventory {
        return Inventory{ items: Vec::new(), capacity: capacity };
    }

    pub fn add_item(&mut self, item : Item) -> Vec<String> {
        let mut result : Vec<String> = Vec::new();
        if self.items.len() as i32 > self.capacity {
            result.push("You cannot carry any more!".to_string());
        } else {
            result.push(format!("You pick up the {}", item.name));
            self.items.push(item);
        }
        return result;
    }

    pub fn remove_item_return_clone(&mut self, item_index: i32) -> Item {
        let item_copy = self.items[item_index as usize].clone();
        self.items.remove(item_index as usize);
        return item_copy;
    }
}

pub fn pickup(gs : &mut State) {
    let mut i = 0;
    let mut item_index = 0;
    let ppos = gs.player().position;
    for e in gs.entities.iter_mut() {
        if e.can_pickup() && e.get_position() == ppos {
            // We can do it!
            item_index = i;
        }
        i += 1;
    }

    if item_index > 0 {
        let cloned_item = gs.entities[item_index].as_item().unwrap().clone();
        let results = gs.player_mut().inventory.add_item(cloned_item); 
        gs.entities.remove(item_index);
        for s in results.iter() {
            gs.add_log_entry(s.clone());
        }
    }
}

pub fn use_item(gs : &mut State, ctx : &mut Rltk) {
    let (result, selection) = gui::handle_item_menu(gs, ctx, "Use which item? (or ESC)");
    match result {
        ItemMenuResult::NoResponse => {}
        ItemMenuResult::Selected => {
            let result = player::use_item(selection, gs);
            for s in result.iter() {
                gs.add_log_entry(s.to_string());
            }            
        }
        ItemMenuResult::Cancel => { gs.game_state = TickType::PlayersTurn }
    }
}

pub fn drop_item(gs : &mut State, ctx : &mut Rltk) {
    let (result, selection) = gui::handle_item_menu(gs, ctx, "Drop which item? (or ESC)");
    match result {
        ItemMenuResult::NoResponse => {}
        ItemMenuResult::Selected => {
            let mut item_copy = gs.player_mut().inventory.remove_item_return_clone(selection);
            item_copy.position = gs.player().get_position();
            gs.add_log_entry(format!("You drop the {}", item_copy.name));
            gs.entities.push(Box::new(item_copy));
            gs.game_state = TickType::EnemyTurn;
        }
        ItemMenuResult::Cancel => { gs.game_state = TickType::PlayersTurn }
    }
}

pub fn item_targeting(gs : &mut State, ctx : &mut Rltk) {
    let result = gui::handle_item_targeting(gs, ctx, "Select your target with cursor keys or mouse, Escape to cancel.");
    match result {
        ItemMenuResult::NoResponse => {}
        ItemMenuResult::Cancel => { gs.game_state = TickType::PlayersTurn }
        ItemMenuResult::Selected => { player::use_area_item(gs); }
    }
}