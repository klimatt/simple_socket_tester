#[derive(Debug, Clone, Copy)]
pub struct Average<T>{
    av : T,
    index: u32
}

impl Average<f64> {
    pub fn new(start_value: f64) -> Average<f64> {
        Average{
            av: start_value,
            index: 0
        }
    }
    pub fn add(&mut self, value: f64){
        if value.is_normal() {
            self.av += value;
            self.index += 1;
        }
    }

    pub fn get(&mut self) -> f64 {
        self.av / self.index as f64
    }

}