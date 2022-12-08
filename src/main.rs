/**
 * Main Project for unerstanding the chapter as a whole
 * Dec 7, 2022
 * Edward Naidoo
 */

/**
 * GOALS:
 * I want to make this project as simple
 * as it can be while accurately giving
 * desired results.
 * 
 * I want to complete this before my
 * exam and after I have completed
 * my studying
*/ 

use std::io;
use round::round_up;


struct Capacity_Design {
    time_avail: f64,
    desired_u: f64,
    cycle_t: f64,
}

impl Capacity_Design {
    
    /**
     * Takes a String value and returns 
     * either a float or string... 
     * Haven't decided yet
     * 
     * Outputs answer
     */
    fn cycle_time(time_availible:String, units:String) {
        // Production Time
        let mut temp = String::new();
        println!("\nWhat is the Production Time for this process?");
        
        io::stdin()
            .read_line( &mut temp)
            .expect("Failed to Readline...");
        
        // Time given to production in minutes - Variable
        let time_available = temp
            .trim()
            .parse::<f64>()
            .expect( "error in time aval var...");
        
        // Desired Unit of Output
        let mut temp = String::new(); // Cleared data in stream 
        println!("\nWhat is the Desried Unit of Output for this process?");
        
        io::stdin()
            .read_line( &mut temp)
            .expect("Failed to Readline...");
        
        // Desired unit of Output - Variable
        let d_units = temp
            .trim()
            .parse::<f64>()
            .expect("Error in d_unit variable..");
        
        // Calc
        let desired_ctime = round_up(time_available / d_units, 2);
        
        // Print w/ formatting
        println!("\nThe Production Time Availible is: {} mins", time_available);
        println!("The Desired Unit of Output is: {} units", d_units);
        println!("The Desired Cycle Time is: {} mins", desired_ctime);
        
    }
    
    /**
     * Finds the number of workstations needed
     * 
     * Returns i64
     */
    fn num_of_workstations(&self) {
        // 2 Variables
        // Input
    }

    /**
     * Finds ths efficiency of the 
     * reorganized workstation
     */
    fn eff(&self) {

    }
}
fn main() {
    /*
     * I want this function to begin the UX of a given prompt asking
     * which choice the user would like to start with. 
     * 
     */
    // Cycle Time
    // Number of Work stations Function
    // Effeciency Function
}