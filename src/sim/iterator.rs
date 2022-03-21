use serde_json::{Result, Value};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use medic::{Agent,House,BehaviourWalking,BehaviourTransport}; // how to call all content from lib?
use medic::inputs::Params;

/* SIMULATOR STATE:
 * agents
 * houses
 * everything else is static (street / transport / amenities)
 */

fn main(){
    let params: Params::new();

    let path_to_agents = ""; // TODO data structure
    let path_to_houses = ""; // TODO data structure
    let path_to_street_network = ""; // TODO data structure
    let path_to_trnsport_network = ""; // TODO data structure

    /////// !!!!!!!! equivalent of self in this context? !!!!!!!!

    let iter = params.iterations;

    let agents: Value = serde_json::from_str(path_to_agents)?; // How to read as list/vec of <Agent> as defined in lib? + mutable
    let houses: Value = serde_json::from_str(path_to_houses)?; // How to read as list/vec of <House> as defined in lib? + mutable

    let s_network: SNetwork = serde_json::from_str(path_to_street_network)?; // ...
    let t_network: TNetwork = serde_json::from_str(path_to_transport_network)?; // ...

    let sq = params.status_quo;
    let ms = params.max_sim + sq;
    let koth = params.k_o_t_h + ms;
    let poba = params.p_o_b_a + koth;
    let trsq = params.t_r_status_quo;
    let mo = params.max_opp + trsq;
    let pi = params.picky + mo;

    // Parameters needed throughout the process
    let rate = params.rate;
    let dist = params.dist;
    let time = params.time;

    impl Distribution<BehaviourWalking> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BehaviourWalking {
            match rng.gen::<f64>() {
                0.0..sq => BehaviourWalking::Statusquo,
                sq..ms => BehaviourWalking::MaxSim,
                ms..koth => BehaviourWalking::KotH,
                koth..poba => BehaviourWalking::PoBA,
                _ => BehaviourWalking::Rooted,
            }
        }
    }

    impl Distribution<BehaviourTransport> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BehaviourTransport {
            match rng.gen::<f64>() {
                0.0..trsq => BehaviourTransport::TRStatusquo,
                trsq..mo => BehaviourTransport::MaxOpp,
                mo..pi => BehaviourTransport::Picky,
                _ => BehaviourTransport::None,
            }
        }
    }

    seeding(agents,houses,params);

    for i in 0..iter {
        step(agents,houses,params);
    }
}

fn seeding(agents: Vec<Agent>, houses: Vec<Houses>, params: Params) {
    // 5. (below) fill houses from waiting list, then larger pool
}

fn step(agents: Vec<Agent>, houses: Vec<Houses>, params: Params) {

    // 1. add new born agents + immigrant agents
    let tot_nb = agents.len() * params.birth;
    let tot_im = agents.len() * params.immigration;

    let h_id = vec![1, 2, 3];// agents.household_id; how to extract all household_id ???
    let sc = vec![1, 2, 3];// list all available social classes ??? -> actually change to enum and implementation

    for i in 0..tot_nb {
        let mut rng = rand::thread_rng();
        let draw_h = rng.gen_range(0..h_id.len()); // TODO draw within compatible household
        let draw_sc = rng.gen_range(0..sc.len());

        let behaviour_walking: BehaviourWalking = rand::random();
        let behaviour_transport: BehaviourTransport = ramd::random();

        let mut new_agent: Agent = Agent {
                                        household_id: h_id.get(draw_h),
                                        age: 0,
                                        house: houses.get(0),
                                        social_class: sc.get(draw_sc),
                                        behaviour_walk: behaviour_walking,
                                        behaviour_trans: behaviour_transport,
                                        initial_walk: 0.0, // maybe NA / None would be more appropriate?
                                        initial_trans: 0, // maybe NA / None would be more appropriate?
                                    };

        agents.push(new_agent)
    }

    for i in 0..tot_im {
        let mut rng = rand::thread_rng();
        let new_h_id = h_id.len(); // For now, immigrant without families
        h_id.push(new_h_id);
        let draw_sc = rng.gen_range(0..len(sc));
        let social_class = sc.get(draw_sc);

        let max_age = social_class.death;
        let age = rng.gen_range(18..max_age);

        let behaviour_walking: BehaviourWalking = rand::random();
        let behaviour_transport: BehaviourTransport = ramd::random();

        let mut new_agent: Agent = Agent {
                                        household_id: new_h_id,
                                        age: age,
                                        house: houses.get(0),
                                        social_class: social_class,
                                        behaviour_walk: behaviour_walking,
                                        behaviour_trans: behaviour_transport,
                                        initial_walk: 0.0, // maybe NA / None would be more appropriate?
                                        initial_trans: 0, // maybe NA / None would be more appropriate?
                                    };

        agents.push(new_agent)
    }

    // 2. increase all agents age + kill agents > life expectancy 
    for agent in agents {
        up(agent);
    }

    agents.retain(|&x| x.age <= x.social_class.death); // what with potential children left alone?

    // 3. vacate random agents (emigration) // accidental death before life expectancy counted as emigration
    let tot_em = h_id.len() * params.emigration; // for now remove entire households at once. TODO: elect new head of household if needed and > 18 

    let draw_em = rng.gen_range(0..h_id.len()); // remove agents no matter where they are

    agents.retain(|&x| !draw_em.contains(x.household_id));
    
    // 4. vacate dissatisfied and add to waiting list
    let dissatisfaction = params.dissatisfaction;

    let mut waiting_list: Vec<Agents> = Vec::new();;

    /*let mut waiting_list = agents.retain(|&x| // maybe very slow? how to speed this up?
                                        x.house != houses.get(0) |
                                        ( 
                                            /* is the code stopping at first condition true or still evaluating 
                                            the second part of or? otherwise pb with houses[0] returning 0 for behaviours */
                                            behaviour_walking(x, rate, x.house, s_network, dist) > dissatisfaction * x.initial_walk & // or |
                                            behaviour_transport(x, rate, x.house, s_network, time) > dissatisfaction * x.initial_trans
                                        )
                                    );
    */

    for agent in tied_agents { // how to simplify this???
        if agent.house != houses.get(0) &
            (
                behaviour_walking(agent, rate, agent.house, s_network, dist) < agent.initial_walk * dissatisfaction | // or &
                behaviour_transport(agent, rate, agent.house, s_network, time) < agent.initial_transport * dissatisfaction
            )
            {
                agent.house = houses.get(0);
                waiting_list.push(agent);
        }
    }

    // 5. fill houses from waiting list, then larger pool

    
    // 6. house price update
}


