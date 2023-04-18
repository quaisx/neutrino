/*
---------------- PROJECT NEUTRINO --------------------
 _______                 __         .__               
 \      \   ____  __ ___/  |________|__| ____   ____  
 /   |   \_/ __ \|  |  \   __\_  __ \  |/    \ /  _ \ 
/    |    \  ___/|  |  /|  |  |  | \/  |   |  (  <_> )
\____|__  /\___  >____/ |__|  |__|  |__|___|  /\____/ 
        \/     \/                           \/        
------------------ NO COPYRIGHTS ---------------------
-------------------- NO LICENSE ----------------------
-------------------- NO PATENTS ----------------------
------------------- FREE FOR ALL ---------------------
------------------- F*CK LAWYERS ---------------------
----------------- F*CK CAPITALISTM -------------------
@authors: free thinkers of the world
    1. Qua Is X (Ukraine) qua.is.kyiv.ua@gmail.com
    /add your name here.../

 */

use std::time::{SystemTime, UNIX_EPOCH};

/// ---------------------------------------------------------------------------
/// gen_seed() is time based seed generator that returns a u64 seed
/// 
/// Returns:
///     u64 seed value
/// ---------------------------------------------------------------------------
pub fn gen_seed() -> u64 {
    let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let pre: u64  = dur.subsec_nanos() as u64;
    pre << 32 ^ dur.as_secs()
}