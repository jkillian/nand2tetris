use std::collections::{HashMap, HashSet};

pub struct SymbolTable {
  map: HashMap<String, u16>,
  unknown_symbols: HashSet<String>,
}

impl SymbolTable {
  pub fn new() -> Self {
    let mut map: HashMap<String, u16> = HashMap::new();
    map.insert(String::from("SP"), 0);
    map.insert(String::from("LCL"), 1);
    map.insert(String::from("ARG"), 2);
    map.insert(String::from("THIS"), 3);
    map.insert(String::from("THAT"), 4);
    map.insert(String::from("SCREEN"), 0x4000);
    map.insert(String::from("KBD"), 0x6000);
    for i in 0..=15 {
      map.insert(format!("R{}", i), i);
    }

    let table = SymbolTable {
      map,
      unknown_symbols: HashSet::new(),
    };

    table
  }

  pub fn insert_label(&mut self, label: &str, value: u16) {
    self.map.insert(String::from(label), value);
  }

  pub fn insert_unknown_symbol(&mut self, symbol: &str) {
    self.unknown_symbols.insert(String::from(symbol));
  }

  pub fn finalize(&mut self) {
    let mut pos: u16 = 16;
    for s in self.unknown_symbols.clone() {
      if !self.map.contains_key(&s) {
        self.map.insert(s, pos);
        pos += 1
      }
    }
  }

  pub fn get_value(&self, symbol: &str) -> Option<&u16> {
    self.map.get(symbol)
  }
}