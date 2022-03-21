#![allow(dead_code)]

// ***** Underlying City *****
pub struct Address {
    lat: f32,
    lng: f32,
}

pub enum Sort {
    Work,
    Leisure,
    School,
    Retail,
}

pub struct House {
    // For now, free agents from the pool are stored within an infinite virtual house
    pool: bool,
    address: Address,
    // For now, each geography unit contains exactly one abstract house
    capacity: i32,
    // For now, average price inside the house
    price: i32,
}

pub struct Venue {
    address: Address,
    sort: Sort,
    // Number of jobs for work, normalised size for leisure activities, normlised ranking for schools etc:
    weight: i32,
}

// Street network
#[derive(Copy, Clone)]
pub struct SNetwork {

}

// Transport network
#[derive(Copy, Clone)]
pub struct TNetwork {

}


// ***** Agent *****

pub struct SocialClass{
    // For now, fixed values, later random values around these means
    name: String,
    init_income: f32,
    growth: f32,
    start: i32,
    retire: i32,
    death: i32,
}

pub enum BehaviourWalking {
    Statusquo,
    MaxSim,
    KotH,
    PoBA,
    Rooted,
}


pub enum BehaviourTransport {
    TrStatusquo,
    MaxOpp,
    Picky,
    None,
}


pub struct Agent {
    household_id: i32,
    age: i32,
    house: House,
    social_class: SocialClass,
    behaviour_walk: BehaviourWalking,
    behaviour_trans: BehaviourTransport,
    // To record initial satisfaction and compare if neighbourhood degrades
    initial_walk: f32,
    initial_trans: i32,
}

impl Agent {
    pub fn up(&mut self) {
        self.age += 1;
    }

    // ***** Income evolution *****

    // TODO add rate to global parameters
    pub fn income(&self, rate: f32) -> f32 {
        let age = self.age;
        let init_income = self.social_class.init_income;
        let start = self.social_class.start;
        let retire = self.social_class.retire + 1;
        if age < start {
            0.0
        } else if age < retire {
            init_income * self.social_class.growth.powf((age - start) as f32)
        } else {
            init_income * rate
        }
    }
}


// ***** Behaviour functions *****

fn income_in_wneigh(_s_network: SNetwork, _adress: Address, _dist: f32) -> Vec<i32> { 
    // TODO add walking distance to define walking neigh to global parameters
    // returns income distrib within walking distance
    todo!()
}

fn activity_in_tneigh(_t_network: TNetwork, _adress: Address, _time: f32) -> (i32,i32,i32) { 
    // TODO add transport time to define transport neigh to global parameters
    // returns number of each activity within transport distance
    todo!()
}

fn mean(list: &[i32]) -> f32 {
    let sum: i32 = Iterator::sum(list.iter());
    (f64::from(sum) / (list.len() as f64)) as f32
}

pub fn behaviour_walking(agent: Agent, rate: f32, house: House, s_network: SNetwork, dist: f32) -> f32 {
    let i = agent.income(rate);
    let j = income_in_wneigh(s_network, agent.house.address, dist);
    let jj = income_in_wneigh(s_network, house.address, dist);
    match agent.behaviour_walk {
        BehaviourWalking::Statusquo => 1.0/(mean(&j) - mean(&jj)).abs(),
        BehaviourWalking::MaxSim => 1.0/(i - mean(&jj)).abs(),
        BehaviourWalking::KotH => i - mean(&jj),
        BehaviourWalking::PoBA => mean(&jj) - i,
        BehaviourWalking::Rooted => 1.0,
    }
}

pub fn behaviour_transport(agent: Agent, house:House, t_network: TNetwork, time: f32) -> i32 {
    let (j,l,s) = activity_in_tneigh(t_network, agent.house.address, time);
    let (jj,ll,ss) = activity_in_tneigh(t_network, house.address, time);
    match agent.behaviour_trans {
        BehaviourTransport::TrStatusquo => 1/((j - jj).abs() + (l - ll).abs() + (s - ss).abs()),
        BehaviourTransport::MaxOpp => jj + ll + ss,
        BehaviourTransport::Picky => ll + ss - jj,
        BehaviourTransport::None => 1,
    }
}

// ***** Affordability rating *****

pub fn affordability(agent: Agent, house: House, rate: f32, price_to_income: f32, fund: f32) -> f32 {
    // TODO add price_to_income to global parameters
    let i = price_to_income * agent.income(rate) + fund;
    let p = house.price as f32;
    if p > i {
        1.0 / (1.0 + ((p - 1.5 * i)/6.0).exp())
    } else {
        (- ((0.7 * i - p)/5.0).exp()).exp()
    }
}

// TODO add freeze period







