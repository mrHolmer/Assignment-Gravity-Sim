//use rand::prelude::*;
use macroquad::prelude::*;
// use macroquad::shapes::* as macroquadshapes;


//
fn RandomNumberBt0and1() -> f64 {0.5}


enum ResultOfFunctionCall {
	FunctionCallSuccess,
	FunctionCallFailure
}

// const UniversalGravitationalConstant: f64 = (6.6743015 / (10 ^ 11));
const UniversalGravitationalConstant: f64 = 1.0;
//

struct PlanetaryBody {
	mass: f64,
	radius: f64,
	location: (f64, f64),
	velocity: (f64, f64)
}

// The impl block defines properties of the type specified. Here, the type specified is PlanetaryBody. 
impl PlanetaryBody {  

	fn PairwiseAdjustVelocityForGravity(body_1: &mut PlanetaryBody, body_2: &mut PlanetaryBody, delta_time: f64) {
		
	let x_displacement: f64 = body_2.location[0] - body_1.location[0]
	let y_displacement: f64 = body_2.location[1] - body_1.location[1]
let distance: f64 = sqrt(((x_displacement) ^ 2) +((y_displacement) ^ 2))
	let force: f64 = (UniversalGravitationalConstant * body_1.mass * body_2.mass / (distance ^ 2))
	let vectors: ((f64, f64), (f64, f64)) = ((x_displacement / distance, y_displacement / distance), (0 - x_displacement / distance, 0 - y_displacement / distance))
	body_1.velocity[0] = body_1.velocity[0] + (delta_time * force * vectors[0][0] / body_1.mass)
	body_1.velocity[1] = body_1.velocity[1] + (delta_time * force * vectors[0][1] / body_1.mass)
	body_2.velocity[0] = body_2.velocity[0] + (delta_time * force * vectors[1][0] / body_2.mass)
	body_2.velocity[1] = body_2.velocity[1] + (delta_time * force * vectors[1][1] / body_2.mass)
}

fn SelfAdjustLocationForVelocity(self: &mut Self, delta_time: f64) {
	self.location[0] = self.location[0] + self.velocity[0];
	self.location[1] = self.location[1] + self.velocity[1];
}

fn PairwiseFindDistanceBetween(body_1: &PlanetaryBody, body_2: &PlanetaryBody) -> f64 {sqrt(((body_1.location[0] - body_2.location[0]) ^ 2) +((body_1.location[1] - body_2.location[1]) ^ 2))}
fn PairwiseCheckForCollision(body_1: &PlanetaryBody, body_2: &PlanetaryBody) -> Boolean {(body_1.radius + body_2.radius) > (PlanetaryBody::PairwiseFindDistanceBetween(body_1, body_2))}
}


fn Tick(planetary_bodies: &mut Vec<PlanetaryBody>) {
	{
let unprocessed_bodies = &planetary_bodies[0..planetary_bodies.length()];
'collision_checks:loop {break 'collision_checks;
	
} // check and handle collisions. break added temporarily
}
{
	
}
}

fn RenderBodies(PlanetaryBodies: &Vec<PlanetaryBody>, view_attributes: (f64, f64, f64)) {
	for item in PlanetaryBodies {
	draw_circle(item.location[0] * view_attributes[2] + view_attributes[0], item.location[1] * view_attributes[2] + view_attributes[1], item.radius * view_attributes[2], BLACK)
}


} // maybe add  -> ResultOfFunctionCall  as output?


#[macroquad::main("Leo Malkovitch's Gravity Simulation Test")]
async fn main() {  // This is the function that is normally set to immediately execute on starting the program. 
	
	let mut planetary_bodies: Vec<PlanetaryBody> = Vec<PlanetaryBody>::with_capacity(64);
	let mut view_attributes: (f64, f64, f64) = (screen_width() / 2, screen_height() / 2, 1.0);
	
	for i in 1..5 {
		planetary_bodies.push(PlanetaryBody {mass: 1.0 + 0.25 * i, radius: screen_height() / 10, velocity: ({a: f64 = RandomNumberBt0and1() * 40 - 20; a}, {a: f64 = RandomNumberBt0and1() * 40 - 20; a}), location: ({a: f64 = (RandomNumberBt0and1() - 0.5) * screen_width(); a}, {a: f64 = (RandomNumberBt0and1() - 0.5) * screen_height(); a})})
	}
	
	'main_cycle loop {
		clear_background(WHITE)
		RenderBodies(&planetary_bodies, view_attributes)
		
next_frame().await
}
	
}
