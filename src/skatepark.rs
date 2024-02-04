use crate::{section::Section, trick::Trick};

pub struct Skatepark {
    m_sections: Vec<Section>,
    m_tricks: Vec<Trick>,
    m_tricks_sum: Vec<Vec<i64>>,
    m_possible_tricks: Vec<Vec<Trick>>,
    m_possible_tricks_time_trick: Vec<u64>,
    m_radical: Vec<Vec<i64>>,
    m_radical_used: Vec<Vec<u16>>
}

impl Skatepark {
    pub fn new() -> Skatepark {
        Skatepark {
            m_sections: Vec::new(),
            m_tricks: Vec::new(),
            m_tricks_sum: Vec::new(),
            m_possible_tricks: Vec::new(),
            m_possible_tricks_time_trick: Vec::new(),
            m_radical: Vec::new(),
            m_radical_used: Vec::new()
        }
    }

    pub fn add_section(&mut self, sections: Vec<Section>) {
        self.m_sections = sections;
    }

    pub fn add_tricks(&mut self, tricks: Vec<Trick>) {
        self.m_tricks = tricks;
    }

    fn start_values(&mut self) {
        let max_combination: u16 = 2_u16.pow(self.m_tricks.len() as u32);

        self.m_possible_tricks.reserve(max_combination as usize);
        self.m_possible_tricks_time_trick.reserve(max_combination as usize);

        
    }

    pub fn more_radical_crossing(&mut self) -> (i64, Vec<Vec<Trick>>) {
        if self.m_sections.is_empty() || self.m_tricks.is_empty() {
            panic!();
        }

        self.start_values();

        
        return (0, Vec::new());
    }
}