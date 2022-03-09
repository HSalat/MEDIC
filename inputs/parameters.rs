pub struct Params {
  //*** scenario definition: ***
  pub name: str, // name for the study area
  pub iterations: f32, // number of model iterations (years)
  //*** sandbox calibration: ***
  pub rate: f32, // ratio between initial salary and retirement salary 
  pub dist: f32, // allowed distance in km through the street network to difine walking neighbourhood
  pub time: f32, // allowed time in min through transport to define transport neighbourhood
  pub price_to_income: f32, // average 'affordable' house price = price_to_income * yearly income
  pub birth: f32, // number of new born each year as a ratio of the size of the population
  pub immigration: f32, // number of new immigrants each year as a ratio of the size of the population
  pub emigration: f32, // prob of a random living individual emigrating
  pub dissatisfaction: f32, // threshold of measured dissatisfaction prompting relocation
  // pub l_to_v: f32, // mortgage has to be repaid before a property can be vacated again
  // pub inflation: f32, // for now we assume salary inflation and house price inflation are aligned
  //*** behaviours: ***
  pub status_quo: f32,
  pub max_sim: f32,
  pub k_o_t_h: f32,
  pub p_o_b_a: f32,
  pub rooted: f32,
  pub t_r_status_quo: f32,
  pub max_opp: f32,
  pub picky: f32,
  pub none: f32,
}

impl Params {
  pub fn new() -> Params {
    Params {
      name: "test",
      iterations: 50,
      rate: 0.6821,
      dist: 5.0,
      time: 25.0,
      price_to_income: 6.0,
      birth: 0.3,
      immigration: 0.05,
      emigration: 0.05,
      dissatisfaction: 0.5,
      status_quo: 0.4,
      max_sim: 0.2,
      k_o_t_h: 0.15,
      p_o_b_a: 0.15,
      rooted: 0.1,
      t_r_statusquo: 0.2,
      max_opp: 0.7,
      picky: 0.05,
      none: 0.05,
    }
  }
}
