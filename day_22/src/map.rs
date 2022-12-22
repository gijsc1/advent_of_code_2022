use std::str::FromStr;
use crate::error::{Error, parse_error};
use crate::parsing::Parsable;
use crate::facing::{Facing, Rotation};
use crate::field::Field;
use crate::field::Field::{Open, Unmapped};
use crate::move_operation::MoveOperation;
use crate::types::{Zone, ZONE_SIZE, ZoneMap};


pub struct Map{
    layout: Vec<Vec<Field>>,
    length: usize,
    height: usize,
    xpos: usize,
    ypos: usize,
    facing: Facing,
    operations: Vec<MoveOperation>,
}

impl Map{
    fn zone_is_defined(&self,zone:Zone)->bool{
        *self.get(zone.0*ZONE_SIZE,zone.1*ZONE_SIZE) != Unmapped
    }

    pub fn get_zone_mod(&self,x:isize,y:isize) ->Zone{
        ((x.rem_euclid(self.length as isize)) as usize/ZONE_SIZE,(y.rem_euclid(self.height as isize)) as usize/ZONE_SIZE)
    }

    pub fn get(&self,x:usize,y:usize)->&Field{
        assert!(x<self.length);
        &self.layout[y].get(x).unwrap_or(&Unmapped)
    }

    pub fn get_with_wraparound(&self,x:isize,y:isize)->&Field{
        self.get(x.rem_euclid(self.length as isize).try_into().unwrap(),
                  y.rem_euclid(self.height as isize).try_into().unwrap())
    }

    fn find_first_defined_v1(&self,x:isize,y:isize)->(&Field,(usize,usize)){
        let (new_field,(newx,newy))= match self.facing {
                Facing::NORTH => {(self.get_with_wraparound(x,y-1),(x,y-1))}
                Facing::SOUTH => {(self.get_with_wraparound(x,y+1),(x,y+1))}
                Facing::EAST => {(self.get_with_wraparound(x+1,y),(x+1,y))}
                Facing::WEST => {(self.get_with_wraparound(x-1,y),(x-1,y))}
        };
        if *new_field != Unmapped {
            (new_field, (newx.rem_euclid(self.length as isize).try_into().unwrap(),
                         newy.rem_euclid(self.height as isize).try_into().unwrap()))
        } else {
            self.find_first_defined_v1(newx, newy)
        }
    }

    fn find_first_defined_v2(&self,x:isize,y:isize,facing:Facing,zonemap:&ZoneMap)->(&Field,(usize,usize),Facing){
        let zone = self.get_zone_mod(x,y);
        assert!(self.zone_is_defined(zone));
        let (newx,newy)= match facing {
            Facing::NORTH => {(x,y-1)}
            Facing::SOUTH => {(x,y+1)}
            Facing::EAST => {(x+1,y)}
            Facing::WEST => {(x-1,y)}
        };
        let posx = newx.rem_euclid(self.length as isize).try_into().unwrap();
        let posy = newy.rem_euclid(self.height as isize).try_into().unwrap();
        // println!("unwrapped target: x:{},y:{}",posx,posy);

        let adjacent_zone = self.get_zone_mod(newx,newy);
        if  adjacent_zone!= zone{
            if let Some((new_zone,rotation)) = zonemap.get(&(zone,facing)){
                // println!("\t\tnow attempting rotation from zone {},{} to zone {},{}",zone.0,zone.1,new_zone.0,new_zone.1);
                let (adjusted_x,adjusted_y) = rotation.move_coordinate(posx,posy,&adjacent_zone,new_zone);
                let new_field = self.get(adjusted_x,adjusted_y);
                assert_ne!(*new_field, Unmapped);
                let new_facing = facing.rotate(*rotation);
                return (new_field,(adjusted_x,adjusted_y),new_facing);
            }
        }

        let new_field = self.get(posx,posy);
        assert_ne!(*new_field, Unmapped);
        return (new_field, (posx as usize, posy as usize),facing);
    }

    fn execute_step(&mut self,rotation:Rotation,steps:usize,do_v2:bool,zonemap:&ZoneMap){
        // println!("attempting to take {steps} steps");
        for _i in 0..steps{
            if !self.take_step(do_v2,zonemap){
                // println!("\tstopped after {} steps",_i);
                break;
            }
        }
        self.facing = self.facing.rotate(rotation);
        // println!("new pos: {},{}; new facing: {}",self.xpos,self.ypos,self.facing);
    }

    ///Take a step, and return true if your face did not hit a wall.
    fn take_step(&mut self,do_v2:bool,zonemap:&ZoneMap)->bool{
        let target:Field;
        let x:usize;
        let y:usize;
        let mut facing = self.facing;
        if do_v2{
            let (temp_target,(temp_x,temp_y),newfacing) = self.find_first_defined_v2(self.xpos as isize,self.ypos as isize,self.facing,zonemap);
            target = *temp_target;
            x = temp_x;
            y = temp_y;
            // println!("\t\tAttempting to step into {},{} wich is {:?} ",x,y,target);
            facing = newfacing;
        } else {
            let (temp_target,(temp_x,temp_y))  = self.find_first_defined_v1(self.xpos as isize,self.ypos as isize);
            target = *temp_target;
            x = temp_x;
            y = temp_y;
        }
        if target==Open{
            // print!("\tTaking single step from {},{}",self.xpos,self.ypos);
            // println!(" to {x},{y}");
            self.xpos=x;
            self.ypos=y;
            self.facing = facing;
            true
        } else {
            false
        }

    }

    pub fn execute_plan(&mut self,do_v2:bool,zonemap:&ZoneMap){
        for i in 0..self.operations.len(){
            let step = &self.operations[i];
            self.execute_step(step.rotation, step.steps,do_v2,zonemap);
        }
    }

    pub fn get_password(&self) ->usize{
        (self.ypos+1)*1000 + (self.xpos+1)*4 + self.facing.get_value()
    }
}

impl FromStr for Map{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (map,instructions) = s.split_once("\n\n").ok_or(parse_error("No instruction part found"))?;
        let height = map.lines().count();

        let mut layout:Vec<Vec<Field>> = Vec::with_capacity(height);
        for line in map.lines(){
            let row:Result<Vec<Field>,_> = line.chars().map(|c|Field::try_from(c)).collect();
            layout.push(row?);
        }
        let length = layout.iter().map(|row|row.len()).max().unwrap();
        let mut ins_vec = Vec::new();
        let mut remainder = instructions;
        while let  Ok((rem,steps)) = remainder.parse_num(){
                let facing:Rotation = Rotation::try_from(rem.chars().nth(0).unwrap_or('n')).unwrap_or(Rotation::NONE);
                ins_vec.push(MoveOperation::new(facing,steps.try_into()?));
            if facing!=Rotation::NONE{
                remainder = &rem[1..];
            } else {
                remainder = rem;
            }
        }
        let mut result = Map{
            layout,
            length,
            height,
            xpos: 0,
            ypos: 0,
            facing: Facing::EAST,
            operations: ins_vec,
        };
        while *result.get(result.xpos,0)!=Open{
            result.xpos+=1;
        }
        println!("parsed map with height:{} and length:{}",result.height,result.length);
        println!("starting position: {},{} with facing {}",result.xpos,result.ypos,result.facing);
        Ok(result)

    }
}

