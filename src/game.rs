use crate::rltk;
use rltk::Color;
use rltk::Console;

extern crate rand;

use rand::Rng;
use std::cmp::{max, min};

const ROOM_MAX_SIZE : i32 = 10;
const ROOM_MIN_SIZE : i32 = 6;
const MAX_ROOMS : i32 = 30;

pub enum TileType {
    Wall, Floor
}

pub struct Player {
    pub x : i32,
    pub y : i32
}

pub struct State {
    pub map_tiles : Vec<TileType>,
    pub player : Player
}

struct Rect {
    x1 : i32,
    x2 : i32,
    y1 : i32,
    y2 : i32
}

impl Rect {
    pub fn new(x1:i32, y1: i32, x2:i32, y2:i32) -> Rect {
        return Rect{x1: x1, y1: y1, x2: x2, y2: y2};
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other:&Rect) -> bool {
        return self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1;
    }

    pub fn center(&self) -> (i32, i32) {
        return ((self.x1 + self.x2)/2, (self.y1 + self.y2)/2);
    }
}

impl State {
    pub fn new() -> State {
        let mut blank_map = Vec::new();
        for _i in 0 .. (80*50) {
            blank_map.push(TileType::Wall);
        }

        let rooms = State::random_rooms_tut3(&mut blank_map);
        let (player_x, player_y) = rooms[0].center();

        return State{ map_tiles: blank_map, player: Player{ x: player_x, y:player_y } };
    }

    fn random_rooms_tut3(mut blank_map : &mut Vec<TileType>) -> Vec<Rect> {
        let mut rng = rand::thread_rng();

        let mut rooms : Vec<Rect> = Vec::new();
        for _i in 1..MAX_ROOMS {
            let w = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE);
            let h = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE);
            let x = rng.gen_range(0, 80 - w - 1);
            let y = rng.gen_range(0, 50 - h - 1);

            let room_candidate = Rect::new(x, y, x+w, y+h);

            let mut collides = false;
            for room in rooms.iter() {
                if room_candidate.intersect(room) {
                    collides = true;
                }
            }

            if !collides {
                State::apply_room(&room_candidate, &mut blank_map);

                if rooms.len() > 0 {
                    let (new_x, new_y) = room_candidate.center();
                    let (prev_x, prev_y) = rooms[rooms.len()-1].center();
                    if rng.gen_range(0,1)==1 {
                        State::apply_horizontal_tunnel(prev_x, new_x, prev_y, &mut blank_map);
                        State::apply_vertical_tunnel(prev_y, new_y, new_x, &mut blank_map);
                    } else {
                        State::apply_vertical_tunnel(prev_y, new_y, prev_x, &mut blank_map);
                        State::apply_horizontal_tunnel(prev_x, new_x, new_y, &mut blank_map);
                    }
                }

                rooms.push(room_candidate);
            }
        }
        return rooms;
    }

    // Applies a rectangle room to the map
    fn apply_room(rect : &Rect, blank_map : &mut Vec<TileType>) {
        for y in min(rect.y1, rect.y2) .. max(rect.y1, rect.y2) {
            for x in min(rect.x1, rect.x2) .. max(rect.x1, rect.x2) {
                let idx = (y * 80) + x;
                if idx > 0 && idx < 80*50 {
                    blank_map[idx as usize] = TileType::Floor;
                }
            }
        }
    }

    fn apply_horizontal_tunnel(x1:i32, x2:i32, y:i32, blank_map : &mut Vec<TileType>) {
        for x in min(x1,x2) .. max(x1,x2)+1 {
            let idx = (y * 80) + x;
            if idx > 0 && idx < 80*50 {
                blank_map[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(y1:i32, y2:i32, x:i32, blank_map : &mut Vec<TileType>) {
        for y in min(y1,y2) .. max(y1,y2)+1 {
            let idx = (y * 80) + x;
            if idx > 0 && idx < 80*50 {
                blank_map[idx as usize] = TileType::Floor;
            }
        }
    }

    // Puts the map onto the console
    fn draw_map(&mut self, console : &mut Console) {
        console.cls();

        let mut idx = 0;
        for y in 0 .. 50 {
            for x in 0 .. 80 {
                match self.map_tiles[idx] {
                    TileType::Floor => { console.print_color(x, y, Color::dark_green(), Color::black(), ".".to_string()) }
                    TileType::Wall => { console.print_color(x, y, Color::white(), Color::black(), "#".to_string()) }
                }

                idx += 1;
            }
        }
    }

    // Draw the @
    fn draw_player(&mut self, console : &mut Console) {
        console.print_color(self.player.x as u32, self.player.y as u32, Color::yellow(), Color::black(), "@".to_string());
    }

    // Utility function: find the index of a tile at x/y
    fn tile_idx(&self, x:i32, y:i32) -> Option<usize> {
        if self.valid_tile(x, y) {
            return Some(((y*80)+x) as usize);
        } else {
            return None;
        }
    }

    // Utility function: bounds checking
    fn valid_tile(&self, x:i32, y:i32) -> bool {
        return x > 0 && x < 79 && y > 0 && y < 49;
    }

    // Utility function: is a tile walkable
    fn is_walkable(&mut self, x:i32, y:i32) -> bool {
        let idx = self.tile_idx(x, y);
        match idx {
            Some(idx) => {
                match self.map_tiles[idx] {
                    TileType::Floor => { return true }
                    TileType::Wall => { return false }
                }
            }

            None => {
                return false;
            }
        }
    }

    fn move_player(&mut self, delta_x : i32, delta_y: i32) {
        let new_x = self.player.x + delta_x;
        let new_y = self.player.y + delta_y;
        if new_x > 0 && new_x < 79 && new_y > 0 && new_y < 49 && self.is_walkable(new_x, new_y) {
            self.player.x = new_x;
            self.player.y = new_y;
        }
    }

    pub fn tick(&mut self, console : &mut Console) {
        self.draw_map(console);
        self.draw_player(console);

        match console.key {
            Some(key) => {
                match key {
                1 => { console.quit() }

                328 => { self.move_player(0, -1) }
                336 => { self.move_player(0, 1) }
                331 => { self.move_player(-1, 0) }
                333 => { self.move_player(1, 0) }

                _ =>  { console.print(0,6, format!("You pressed: {}", key)) }                
                }
            }
            None => {}
        }
    }
}