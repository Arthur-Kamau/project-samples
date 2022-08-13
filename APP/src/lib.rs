use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Driver {
    Name : String,
    VehicleId :String,
    License : String,
    driverId: usize,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Parkingslot {
    locations:Vec<location>,
}
#[near_bindgen]
impl Parkingslot {
    pub fn empty_slots() -> self{
        let locations: Vec<location> = vec::empty();
        Parkingslot{
            locations
        }
        
    }

    pub fn Parkingslot_count(&mut Self { locations }: Self) -> usize {
        self.locations.len()
    }

    pub fn add_driver(&mut self, name:String,
     VehicleId:String, License:String,driverId:usize){
        let slot1 = Driver{
            Name:name.to_string(),
            VehicleId:VehicleId.to_string(),
            License:License.to_string(),
            driverId:driverId.to_string()
        };
        self.locations.push(slot1);
        env::log_str("slot available");
    }

    pub fn Available_slots(&mut self) -> &Vec<location>{
        &self.locations
    }
    
    pub fn booked_slot(&mut self) {
        self.locations.pop();
        env::log_str("slot is booked");
        
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn free_slots(){

        let user: AccountId =AccountId::new_unchecked(id:"masinde.test.net".to_string());
        let _context: VMContextBuilder = get_context(account:user.clone());

        let mut slots: Parkingslot = Parkingslot::empty_slots();
        slots.add_driver(name:"Naivas".to_string(),VehicleId:"khetias".to_string(),License:"khetias".to_string(), driverId:"khetias");

        let counting:usize - slots.Parkingslot_count();
        assert_eq!(counting,1)
    }


    #[test]
    fn add_slot() {
        let user: AccountId = AccountId::new_unchecked(id:"masinde.testet".to_string());
        let _context: VMContextBuilder = get_context(account:user.clone());

        let mut slots:Parkingslot = Parkingslot::empty_slots();

        slots.add_slot(name:"sharrif".to_string(),VehicleId:"KDC 146D".to_string(),License:"3137C23K".to_string(), driverId:"38547189");
        slots.add_slot(name:"sharrif".to_string(),VehicleId:"KDD 578D".to_string(),License:"3142K23L".to_string(), driverId:"35627828");
        slots.add_slot(name:"sharrif".to_string(),VehicleId:"KCB 192C".to_string(),License:"1568J76H".to_string(), driverId:"67358521");
    
        let counts:usize - slots.count();
        assert_eq!(counts, 3)
    
    


    }


    // TESTS HERE
    //add here my comments
    //this is an example of a single thread comment
    /*this is an example of  a multi line comment*/
}
