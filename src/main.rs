
//use rand::prelude::*;
extern crate macroquad;
use macroquad::prelude::*;
use std::borrow::BorrowMut;
// use macroquad::shapes::* as macroquadshapes;
extern crate num_traits;
use num_traits::pow::pow as Pow;

//
fn RandomNumberBt0and1() -> f64 {0.5}
fn LocalDrawCircle(a: f64, b: f64, c: f64, d: macroquad::prelude::Color) { // called this because it's the local version
	macroquad::prelude::draw_circle(a as f32, b as f32, c as f32, d)
}

enum ResultOfFunctionCall {
	FunctionCallSuccess,
	FunctionCallFailure
}

// const UniversalGravitationalConstant: f64 = (6.6743015 / (10 ^ 11));
const UniversalGravitationalConstant: f64 = 1.0;
//
#[derive(Debug)]
struct PlanetaryBody {
	mass: f64,
	radius: f64,
	location: [f64; 2],
	velocity: [f64; 2]
}

// The impl block defines properties of the type specified. Here, the type specified is PlanetaryBody. 
impl PlanetaryBody {  
	fn PairwiseAdjustVelocityForGravity(body_1: &mut PlanetaryBody, body_2: &mut PlanetaryBody, delta_time: f64) {
		let x_displacement: f64 = body_2.location[0] - body_1.location[0];
		let y_displacement: f64 = body_2.location[1] - body_1.location[1];
		let distance: f64 = f64::sqrt(Pow((x_displacement), 2) + Pow((y_displacement), 2));
		// if distance = (0 as f64) {break} // this should throw an error somehow
		let force: f64 = UniversalGravitationalConstant * body_1.mass * body_2.mass / Pow(distance, 2);
		let vectors: [[f64; 2]; 2] = [[x_displacement / distance, y_displacement / distance], [0.0 - x_displacement / distance, 0.0 - y_displacement / distance]];
		body_1.velocity[0] = body_1.velocity[0] + (delta_time * force * vectors[0][0] / body_1.mass);
		body_1.velocity[1] = body_1.velocity[1] + (delta_time * force * vectors[0][1] / body_1.mass);
		body_2.velocity[0] = body_2.velocity[0] + (delta_time * force * vectors[1][0] / body_2.mass);
		body_2.velocity[1] = body_2.velocity[1] + (delta_time * force * vectors[1][1] / body_2.mass);
	}

	fn SelfAdjustLocationForVelocity(self: &mut Self, delta_time: f64) {
		self.location[0] = self.location[0] + self.velocity[0] * delta_time;
		self.location[1] = self.location[1] + self.velocity[1] * delta_time;
	}

	fn PairwiseFindDistanceBetween(body_1: &PlanetaryBody, body_2: &PlanetaryBody) -> f64 {f64::sqrt(Pow((body_1.location[0] - body_2.location[0]), 2) + Pow((body_1.location[1] - body_2.location[1]), 2))}
	fn PairwiseCheckForCollision(body_1: &PlanetaryBody, body_2: &PlanetaryBody) -> bool {(body_1.radius + body_2.radius) > (PlanetaryBody::PairwiseFindDistanceBetween(body_1, body_2))}
} // this is the end of the impl block


fn PhysicsTick(planetary_bodies_r: &Vec<PlanetaryBody>, number_of_bodies: usize, delta_time: f64) {
	//'collision_checks: loop {break 'collision_checks;} // check and handle collisions. break added temporarily, commented out for skipping initially
	
	for first_index in 1..(number_of_bodies-1) {
		//let first_body_opt: Option::<&mut PlanetaryBody> = planetary_bodies_r.get_mut(lower_index - 1);
		if let Some(first_body) = planetary_bodies_r.get_mut(first_index - 1) {
			for second_index in first_index..number_of_bodies {
				if let Some(second_body) = planetary_bodies_r.get_mut(second_index) {
					PlanetaryBody::PairwiseAdjustVelocityForGravity(first_body, second_body, delta_time);
				};
			};
		};
		//if number_of_bodies - lower_index == 1 
	}
	for index in 0..(number_of_bodies - 1) {
		if let Some(body) = planetary_bodies_r.get_mut(index){
			body.SelfAdjustLocationForVelocity(delta_time);
		};
	}
}

fn RenderBodies(planetary_bodies_r: &Vec<PlanetaryBody>, view_attributes: [f64; 3]) {
	for item in planetary_bodies_r {
		LocalDrawCircle(item.location[0] * view_attributes[2] + view_attributes[0], item.location[1] * view_attributes[2] + view_attributes[1], item.radius * view_attributes[2], macroquad::prelude::BLACK)
	}
}

#[macroquad::main("Assignment-Gravity-Sim")]
async fn main() {  // This is the function that is normally set to immediately execute on starting the program. 
	
	let mut planetary_bodies: Vec<PlanetaryBody> = Vec::<PlanetaryBody>::with_capacity(64);
	let mut view_attributes: [f64; 3] = [(macroquad::prelude::screen_width() as f64) / 2.0, (macroquad::prelude::screen_height() as f64) / 2.0, 1.0];
	
/*	for i in 1..5 {
		planetary_bodies.push(PlanetaryBody {mass: 1.0 + 0.25 * (i as f64), radius: (macroquad::prelude::screen_height() as f64) / 10.0, velocity: [{let a: f64 = RandomNumberBt0and1() * 40.0 - 20.0; a}, {let a: f64 = RandomNumberBt0and1() * 40.0 - 20.0; a}], location: [{let a: f64 = (RandomNumberBt0and1() - 0.5) * (macroquad::prelude::screen_width() as f64); a}, {let a: f64 = (RandomNumberBt0and1() - 0.5) * (macroquad::prelude::screen_height() as f64); a}]})
	} /*Temporarily removed so I try non-random start*/ */
	planetary_bodies.push(PlanetaryBody {mass: 1.0, radius: (macroquad::prelude::screen_height() as f64) / 20.0, velocity: [0.0, 5.0], location: [{macroquad::prelude::screen_width() * 0.75} as f64, 0.0]});
	planetary_bodies.push(PlanetaryBody {mass: 1.0, radius: (macroquad::prelude::screen_height() as f64) / 20.0, velocity: [0.0, -5.0], location: [{macroquad::prelude::screen_width() * 0.25} as f64, 0.0]});
	'main_cycle: loop {
		clear_background(WHITE);
		RenderBodies(&planetary_bodies, view_attributes);
		let mut templen = 0;
		{
			templen += planetary_bodies.len()
		}
		PhysicsTick(&planetary_bodies, templen, 1.0 as f64);
		println!("{:#?}", &planetary_bodies);
		next_frame().await
	}
	
}
