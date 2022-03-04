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
    capacity: i32, // For now, each geography unit contains exactly one abstract house
    price: i32, // For now, average price inside the house
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

pub fn income(agent: Agent, rate: f32) { // add rate to global parameters
    let a = agent.age;
    let i = agent.social_class.init_income;
    let g = agent.social_class.growth;
    let s = agent.social_class.start;
    let r = agent.social_class.retire + 1;
    if a < s {
        0
    } else if a < r {
        i * g.pow(a - s) 
    } else {
        i * r
    }
}

// ***** Behaviour functions *****

fn income_in_wneigh(s_network: SNetwork, adress: Address, dist: f32) { // add walking distance to define walking neigh to global parameters
    // returns income distrib within walking distance
}

fn activity_in_tneigh(t_network: TNetwork, adress: Address, time: f32) { // add transport time to define transport neigh to global parameters
    // returns number of each activity within transport distance
}

fn mean(list: &[i32]) -> f64 {
    let sum: i32 = Iterator::sum(list.iter());
    f64::from(sum) / (list.len() as f64)
}

pub fn behaviour_walking(agent: Agent, rate: f32, house: House, s_network: SNetwork, dist: f32) {
    let i = income(agent, rate);
    let j = income_in_wneigh(s_network, agent.house.adress, dist);
    let jj = income_in_wneigh(s_network, house.adress, dist);
    match agent.behaviour_walk {
        "Statusquo" => 1/(mean(j) - mean(jj)).abs(),
        "MaxSim" => 1/(i - mean(jj)).abs(),
        "KotH" => i - mean(jj),
        "PoBA" => mean(jj) - i,
        "Rooted" => 1,
    }
}

pub fn behaviour_transport(agent: Agent, rate: f32) {
    let (j,l,s) = activity_in_tneigh(s_network, agent.house.adress, dist);
    let (jj,ll,ss) = activity_in_tneigh(s_network, house.adress, dist);
    match agent.behaviour_trans {
        "Statusquo" => 1/((j - jj).abs() + (l - ll).abs() + (s - ss).abs()),
        "MaxOpp" => jj + ll + ss,
        "Picky" => ll + ss - jj,
        "None" => 1,
    }
}

// ***** Affordability rating *****

pub fn affordability(agent: Agent, house: House, rate: f32, price_to_income: f32, fund: f32) { // add price_to_income to global parameters
    let i = price_to_income * income(agent, rate) + fund;
    let p = house.price;
    if p > i {
        1 / (1 + ((p - 1.5 * i)/6).exp())
    } else {
        (- ((0.7 * i - p)/5).exp()).exp()
    }
}

// add freeze period







