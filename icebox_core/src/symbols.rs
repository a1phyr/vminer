use hashbrown::HashMap;

pub struct StructField {
    pub name: String,
    pub offset: u64,
}

pub struct OwnedStruct {
    pub name: String,
    pub fields: Vec<StructField>,
}

impl OwnedStruct {
    fn borrow(&self) -> Struct {
        Struct {
            name: &self.name,
            fields: &self.fields,
        }
    }
}

pub struct Struct<'a> {
    pub name: &'a str,
    pub fields: &'a [StructField],
}

pub struct SymbolsIndexer {
    structs: HashMap<String, OwnedStruct>,
}

impl SymbolsIndexer {
    pub fn new() -> Self {
        Self {
            structs: HashMap::new(),
        }
    }

    pub fn get_struct(&self, name: &str) -> Option<Struct> {
        self.structs.get(name).map(OwnedStruct::borrow)
    }

    pub fn insert(&mut self, structure: OwnedStruct) {
        self.structs.insert(structure.name.clone(), structure);
    }
}
