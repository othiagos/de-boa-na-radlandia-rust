#[derive(Clone)]
pub struct Trick {
    m_baseline_score: i32,
    m_time_trick: u32,
    m_index: u8
}

impl Trick {
    pub fn new(baseline_score: i32, time_trick: u32, index: u8)  -> Trick {
        Trick {
            m_baseline_score: baseline_score,
            m_time_trick: time_trick,
            m_index: index
        }
    }

    pub fn get_baseline_score(&self) -> i32 {
        self.m_baseline_score
    }

    pub fn set_baseline_score(&mut self, baseline_score: i32) {
        self.m_baseline_score = baseline_score;
    }

    pub fn get_time_trick(&self) -> u32 {
        self.m_time_trick
    }

    pub fn set_time_trick(&mut self, time_trick: u32) {
        self.m_time_trick = time_trick;
    }

    pub fn get_index(&self) -> u8 {
        self.m_index
    }
    
    pub fn set_index(&mut self, index: u8) {
        self.m_index = index;
    }
}
