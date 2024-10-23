use std::collections::HashMap;
use std::result::Result;
use thiserror::Error;

use ecow::EcoString;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: EcoString,
    pub sym_type: SymbolType,
    pub scope: usize, //
    pub memory_location: usize,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub enum SymbolType {
    Variable,
    Function,
    Constant,
    Parameter,
    Array,
    Struct,
    Enum,
    Method,
    Type,
    Import,
}

#[derive(Error, Debug)]
pub enum SymbolError {
    #[error("Symbol '{0}' already defined")]
    SymbolAlreadyDefined(String),
}

pub struct SymbolTable {
    symbols: HashMap<EcoString, Symbol>,
    current_scope: usize,
    next_mem_location: usize,
}

impl SymbolTable {
    // create a new empty symbol table
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            current_scope: 0,
            next_mem_location: 0,
        }
    }

    // enter new scope
    pub fn enter_scope(&mut self) {
        self.current_scope += 1
    }

    pub fn exit_scope(&mut self) {
        self.current_scope -= 1
    }

    pub fn add_symbol(
        &mut self,
        name: EcoString,
        symbol_type: SymbolType,
        size: usize,
    ) -> Result<(), SymbolError> {
        if self.symbols.contains_key(&name) {
            return Err(SymbolError::SymbolAlreadyDefined(name.to_string()));
        } else {
            let sym = Symbol {
                name: name.clone(),
                sym_type: symbol_type,
                scope: self.current_scope,
                memory_location: self.next_mem_location,
                size,
            };
            self.symbols.insert(name.clone(), sym);
            self.next_mem_location += size;
            return Ok(());
        }
    }

    pub fn get_symbol(&self, name: &EcoString) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}
