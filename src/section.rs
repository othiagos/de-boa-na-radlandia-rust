#[derive(Clone)]
pub struct Section {
    m_bonus_factor: u8,
    m_crossing_time: u32
}

impl Section {
    pub fn new(bonus_factor: u8, crossing_time: u32) -> Section {
        Section {
            m_bonus_factor: bonus_factor,
            m_crossing_time: crossing_time
        }
    }

    pub fn get_bonus_factor(&self) -> u8 {
        self.m_bonus_factor
    }

    pub fn set_bonus_facto(&mut self, bonus_factor: u8) {
        self.m_bonus_factor = bonus_factor;
    }

    pub fn get_crossing_time(&self) -> u32 {
        self.m_crossing_time
    }

    pub fn set_crossing_time(&mut self, crossing_time: u32) {
        self.m_crossing_time = crossing_time;
    }

}
