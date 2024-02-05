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

        self.m_possible_tricks.push(Vec::new());
        self.m_possible_tricks_time_trick.push(0);

        for i in 1..max_combination as usize {
            let mut sub_vec: Vec<Trick> = Vec::new();
            let mut sum: u64 = 0;

            for j in 0..self.m_tricks.len() {
                if i >> j & 1 == 1 {
                    sub_vec.push(self.m_tricks.get(j).unwrap().clone());
                    sum += self.m_tricks.get(j).unwrap().get_time_trick() as u64;
                }
            }

            self.m_possible_tricks.push(sub_vec);
            self.m_possible_tricks_time_trick.push(sum);
        }

        self.m_radical.reserve(max_combination as usize);
        for _ in 0..max_combination as usize {
            self.m_radical.push(vec![i64::MIN; self.m_sections.len()]);
        }

        self.m_radical_used.reserve(max_combination as usize);
        for _ in 0..max_combination as usize {
            self.m_radical_used.push(vec![0; self.m_sections.len()])
        }

        self.m_tricks_sum.reserve(max_combination as usize);
        for _ in 0..max_combination as usize {
            self.m_tricks_sum.push(vec![i64::MIN; max_combination as usize]);
        }
    }

    fn sum_penalized_tricks(&self, used_m: u16, m: u16) -> i64 {
        let mut sum: i64 = 0;

        let trick_size = self.m_tricks.len();
        for i in 0..trick_size {
            if (used_m >> i & 1) & (m >> i & 1) == 1 {
                sum += self.m_tricks[i].get_baseline_score() as i64 / 2;
            } else if m >> i & 1 == 1 {
                sum += self.m_tricks[i].get_baseline_score() as i64;
            }
        }
        sum
    }

    pub fn more_radical_crossing(&mut self) -> (i64, Vec<Vec<Trick>>) {
        if self.m_sections.is_empty() || self.m_tricks.is_empty() {
            panic!();
        }

        self.start_values();

        let mut max_section: i64;
        let max_combination = self.m_possible_tricks.len();
        
        for used_m in 0..max_combination {
            max_section = 0;
            let mut max_m: u16 = 0;

            for m in 0..max_combination {
                if self.m_possible_tricks_time_trick[m] > self.m_sections.last().unwrap().get_crossing_time() as u64 {
                    continue;
                }

                let mut sum: i64 = self.sum_penalized_tricks(used_m as u16, m as u16);
                self.m_tricks_sum[used_m][m] = sum;
                
                sum *= self.m_possible_tricks[m].len() as i64 * self.m_sections.last().unwrap().get_bonus_factor() as i64;

                if max_section < sum {
                    max_section = sum;
                    max_m = m as u16;
                }
            }
            self.m_radical[used_m][self.m_sections.len() - 1] = max_section;
            self.m_radical_used[used_m][self.m_sections.len() - 1] = max_m;
        }

        for i in (0..=self.m_sections.len() - 2).rev() {
            for used_m in 0..max_combination {
                max_section = 0;
                let mut max_m: u16 = 0;
                
                for m in 0..max_combination {
                    if self.m_possible_tricks_time_trick[m] > self.m_sections[i].get_crossing_time() as u64 {
                        continue;
                    }

                    let mut sum: i64;
                    if self.m_tricks_sum[used_m][m] == i64::MIN {
                        sum = self.sum_penalized_tricks(used_m as u16, m as u16);
                        self.m_tricks_sum[used_m][m] = sum;
                    } else {
                        sum = self.m_tricks_sum[used_m][m];
                    }
                    
                    sum *= self.m_possible_tricks[m].len() as i64 * self.m_sections[i].get_bonus_factor() as i64;
                    sum += self.m_radical[m][i + 1];
                    
                    if max_section < sum {
                        max_section = sum;
                        max_m = m as u16;
                    }
                }
                self.m_radical[used_m][i] = max_section;
                self.m_radical_used[used_m][i] = max_m;
            }
        } 

        let mut vec: Vec<Vec<Trick>> = Vec::new();
        let mut j: u16 = 0;
        for i in 0..self.m_sections.len() {
            j = self.m_radical_used[j as usize][i];
            vec.push(self.m_possible_tricks[j as usize].clone());
        }

        (self.m_radical[0][0], vec)
    }
}