use bevy::prelude::Component;

#[allow(dead_code)]
#[derive(Component, Debug, Clone)]
pub enum CellStructure {
    Producer,
    Collector,
}

#[allow(dead_code)]
impl CellStructure {
    pub fn new(cell_structure: CellStructure) -> Self {
        match cell_structure {
            CellStructure::Producer => CellStructure::Producer,
            CellStructure::Collector => CellStructure::Collector,
        }
    }

    pub fn get_structure(&self) -> CellStructure {
        match self {
            CellStructure::Producer => CellStructure::Producer,
            CellStructure::Collector => CellStructure::Collector,
        }
    }

    pub fn get_required_energy(&self) -> u16 {
        match self {
            CellStructure::Producer => 10,
            CellStructure::Collector => 15,
        }
    }

    pub fn get_required_time(&self) -> u16 {
        match self {
            CellStructure::Producer => 10,
            CellStructure::Collector => 15,
        }
    }
}

impl std::fmt::Display for CellStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}
