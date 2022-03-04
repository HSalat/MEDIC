// ***** Underlying City *****
struct Address {
    lat: f32,
    lng: f32,
}

enum Sort {
    Work,
    Leisure,
    School,
    Retail,
}

struct House {
    adress: Address,
    /// For now, each geography unit contains exactly one abstract house
    capacity: i32,
    /// For now, average price inside the house
    price: i32,
}

struct Venue {
    address: Address,
    sort: Sort,
    weight: i32, // Number of jobs for work, normalised size for leisure activities, normlised ranking for schools etc.
}

// Street network

struct SNetwork {

}

// Transport network

struct TNetwork {

}


// ***** Agent *****

struct SocialClass{ // For now, fixed values, later random values around these means
    name: String,
    init_income: f32,
    growth: f32,
    start: i32,
    retire: i32,
    death: i32,
}

enum BehaviourWalking {
    Statusquo,
    MaxSim,
    KotH,
    PoBA,
    Rooted,
}

enum BehaviourTransport {
    Statusquo,
    MaxOpp,
    Picky,
    None,
}

struct Agent {
    age: i32,
    house: House,
    social_class: SocialClass,
    behaviour_walk: BehaviourWalking,
    behaviour_trans: BehaviourTransport,
}

impl Agent {
    fn up(&mut self) {
        self.age += 1;
    }
}

// ***** Income evolution *****

impl Agent {
    // TODO add rate to global parameters
    pub fn income(&self, rate: f32) -> f32 {
        let age = self.age;
        let init_income = self.social_class.init_income;
        let start = self.social_class.start;
        let retire = self.social_class.retire + 1;
        if age < start {
            0.0
        } else if age < retire {
            init_income * self.social_class.growth.pow(age - start)
        } else {
            init_income * retire
        }
    }
}

// ***** Behaviour functions *****

fn income_in_wneigh(s_network: SNetwork, adress: Address, dist: f32) -> Vec<i32> { // add walking distance to define walking neigh to global parameters
    // returns income distrib within walking distance
    todo!()
}

fn activity_in_tneigh(t_network: TNetwork, adress: Address, time: f32) -> Vec<i32> { // add transport time to define transport neigh to global parameters
    // returns number of each activity within transport distance
    todo!()
}

fn mean(list: &[i32]) -> f64 {
    let sum: i32 = Iterator::sum(list.iter());
    f64::from(sum) / (list.len() as f64)
}

pub fn behaviour_walking(agent: &Agent, rate: f32, house: House, s_network: SNetwork, dist: f32) {
    let i = agent.income(rate);
    let j = income_in_wneigh(s_network, agent.house.adress, dist);
    let jj = income_in_wneigh(s_network, house.adress, dist);
    match agent.behaviour_walk {
        BehaviourWalking::Statusquo => 1/(mean(j) - mean(jj)).abs(),
        BehaviourWalking::MaxSim => 1/(i - mean(jj)).abs(),
        BehaviourWalking::KotH => i - mean(jj),
        BehaviourWalking::PoBA => mean(jj) - i,
        BehaviourWalking::Rooted => 1,
    }
}

pub fn behaviour_transport(agent: Agent, rate: f32) {
    let (j,l,s) = activity_in_tneigh(s_network, agent.house.adress, dist);
    let (jj,ll,ss) = activity_in_tneigh(s_network, house.adress, dist);
    match agent.behaviour_trans {
        BehaviourTransport::Statusquo => 1/((j - jj).abs() + (l - ll).abs() + (s - ss).abs()),
        BehaviourTransport::MaxOpp => jj + ll + ss,
        BehaviourTransport::Picky => ll + ss - jj,
        BehaviourTransport::None => 1,
    }
}

// ***** Affordability rating *****

pub fn affordability(agent: Agent, house: House, rate: f32, price_to_income: f32, fund: f32) -> f32 { // add price_to_income to global parameters
    let i = price_to_income * agent.income(rate) + fund;
    let p = house.price;
    if p > i {
        1.0 / (1.0 + ((p - 1.5 * i)/6.0).exp())
    } else {
        (- ((0.7 * i - p)/5.0).exp()).exp()
    }
}

// add freeze period







