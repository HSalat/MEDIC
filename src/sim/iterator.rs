use serde_json::{Result, Value};
use medic::{Agent,House}; // how to call content from lib?

/* SIMULATOR STATE:
 * agents
 * houses
 * everything else is static (street / transport / amenities)
 */

fn main(){
    let path_to_agents = ""; // TODO data structure
    let path_to_houses = ""; // TODO data structure

    let iterator = 10i32; // bring over from parameters

    let agents: Value = serde_json::from_str(path_to_agents)?; // How to read as list of <Agent> as defined in lib?
    let houses: Value = serde_json::from_str(path_to_houses)?; // How to read as list of <House> as defined in lib?

    seeding(agents,houses);

    for i in 0..iterator {
        step(agents,houses);
    }
}

fn seeding(agents: list, houses: list) {
    // 5. (below) fill houses from waiting list, then larger pool
}

fn step(agents: list, houses: list) {
    // 1. add new born agents + immigrant agents
    // 2. increase all agents age + kill agents > life expectancy 
    for agent in agents {
        up(agent);
        if agent.age > agent.social_class.death {
            todo!() // ???
        }
    }
    // 3. vacate dissatisfied and add to waiting list
    // 4. vacate random agents (emigration)
    // 5. fill houses from waiting list, then larger pool
    // 6. house price update
}


