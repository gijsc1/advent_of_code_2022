use std::str::FromStr;
use crate::error::Error;
use crate::parsing::Parsable;
use crate::types::{Clay, Obsidan, Ore, State, Time};


pub struct Factory{
    pub name:usize,
    orecost:Ore,
    claycost: Ore,
    obsidiancost:(Ore,Clay),
    geodecost:(Ore,Obsidan)
}

impl Factory{
    pub fn can_build_obsrobot(&self, (_,ore,_,clay,_,_,_,_):&State) -> bool {
        *ore>=self.obsidiancost.0 && *clay>=self.obsidiancost.1
    }

    pub fn enough_obsrobot(&self, (_,_,_,_,obsbot,_,_,_):&State) -> bool {
        *obsbot>=self.geodecost.1
    }

    pub fn destruct_obsrobot(&self, (_,ore,_,clay,obsrobot,_,_,_):&mut State) {
        *ore+=self.obsidiancost.0;
        *clay+=self.obsidiancost.1;
        *obsrobot-=1;

    }

    pub fn build_obsrobot(&self, (_,ore,_,clay,obsrobot,_,_,_):&mut State) {
        *ore-=self.obsidiancost.0;
        *clay-=self.obsidiancost.1;
        *obsrobot+=1;

    }

    pub fn can_build_geoderobot(&self, (_,ore,_,_,_,obs,_,_):&State) -> bool {
        *ore>=self.geodecost.0 && *obs>=self.geodecost.1
    }

    pub fn build_geoderobot(&self, (_,ore,_,_,_,obs,geoderobot,_):&mut State) {
        *ore-=self.geodecost.0;
        *obs-=self.geodecost.1;
        *geoderobot+=1;
    }

    pub fn destruct_geoderobot(&self, (_,ore,_,_,_,obs,geoderobot,_):&mut State) {
        *ore+=self.geodecost.0;
        *obs+=self.geodecost.1;
        *geoderobot-=1;
    }

    pub fn can_build_clayrobot(&self, (_,ore,_,_,_,_,_,_):&State) -> bool {
        *ore>=self.claycost
    }

    pub fn enough_clayrobot(&self, (_,_,claybot,_,_,_,_,_):&State) -> bool {
        *claybot>= self.obsidiancost.1
    }

    pub fn build_clayrobot(&self, (_,ore,clayrobot,_,_,_,_,_):&mut State) {
        *ore-=self.claycost;
        *clayrobot+=1;
    }

    pub fn destruct_clayrobot(&self, (_,ore,clayrobot,_,_,_,_,_):&mut State) {
        *ore+=self.claycost;
        *clayrobot-=1;
    }

    pub fn can_build_orerobot(&self, (_,ore,_,_,_,_,_,_):&State) -> bool {
        *ore>=self.orecost
    }

    pub fn enough_orebot(&self, (orebot,_,_,_,_,_,_,_):&State) -> bool {
        *orebot>= (self.orecost.max(self.claycost.max(self.obsidiancost.0).max(self.geodecost.0)))
    }

    pub fn build_orerobot(&self, (oreRobot,ore,_,_,_,_,_,_):&mut State) {
        *ore-=self.orecost;
        *oreRobot+=1;
    }

    pub fn destruct_orerobot(&self, (oreRobot,ore,_,_,_,_,_,_):&mut State) {
        *ore+=self.orecost;
        *oreRobot-=1;
    }

    ///failed attempt at an optimisation
    pub fn obs_still_usefull(&self,(orerobot,ore,_,_,obsrobot,_,_,_):&State,time:Time)->bool{
        let ore_needed = self.geodecost.0 as isize-(*ore as isize) +self.obsidiancost.0 as isize;
        let ore_cycles_needed = ore_needed/(*orerobot as isize);
        let obs_needed = self.geodecost.1 as isize - (*obsrobot as isize * ore_cycles_needed);
        if obs_needed <=0
            || ore_cycles_needed>=time as isize
            // || obs_needed>= (time as isize)
        {
            return false;
        }
        true

    }


}

impl FromStr for Factory{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        let (_,(((((((name,ore_ore),clay_ore)),obs_ore),obs_clay),geo_ore),geo_obs)  ) = s.parse_prefix("Blueprint ")
            .parse_num()
            .parse_prefix(": Each ore robot costs ")
            .parse_num()
            .parse_prefix(" ore. Each clay robot costs ")
            .parse_num()
            .parse_prefix(" ore. Each obsidian robot costs ")
            .parse_num()
            .parse_prefix(" ore and ")
            .parse_num()
            .parse_prefix(" clay. Each geode robot costs ")
            .parse_num()
            .parse_prefix(" ore and ")
            .parse_num()?;
        Ok(Factory{
            name: name.try_into().unwrap(),
            orecost: ore_ore.try_into().unwrap(),
            claycost: clay_ore.try_into().unwrap(),
            obsidiancost: (obs_ore.try_into().unwrap(), obs_clay.try_into().unwrap()),
            geodecost: (geo_ore.try_into().unwrap(), geo_obs.try_into().unwrap()),
        })
    }
}

