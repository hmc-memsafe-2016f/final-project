//Just a place to store implementation details for the actual game part.

extern crate sfml;

use sfml::graphics::{PrimitiveType, CircleShape, Color, Drawable, RectangleShape, RenderStates, RenderTarget,
                     RenderWindow, Shape, Transformable};
use sfml::window::{Key, VideoMode, Event, window_style};
use sfml::system::Vector2f;

pub struct Point<'s>
{
	pos: Vector2f,
	shape: CircleShape<'s>
}

impl<'s> Point<'s>
{
	pub fn new(x: f32, y: f32) -> Self
	{
		pos = Vector2f::new(x, y);
		let mut shape = CircleShape::new_init(10f32 , 50);
		shape.set_position2f(x, y);
		shape.set_fill_color(&Color::red());

		Point
		{
			pos:	pos,
			shape:	shape,
		}
	}
}

impl<'s> Drawable for Point<'s>
{
	fn draw(&self, render_target: &mut RenderTarget, _: &mut RenderStates) {
        render_target.draw(&self.shape);
    }
}

/*
pub struct Line
{
	p1: Vector2f,
	p2: Vector2f,
	//shape: line
}
*/



/*
//For two line segments defined by points,
//when do they intersect?
//TODO: CLEAN THIS UP
fn lines_intersect(line1_p1: (i32, i32), line1_p2: (i32, i32), line2_p1: (i32, i32), line2_p2: (i32, i32)) -> bool
{
	let line1_vector = line1_p2 - line1_p1;
	let line2_vector = line2_p2 - line2_p1;

	let point_vector = line2_p1 - line1_p1;

	let denominator = line1_vector /*CROSS*/ line2_vector;
	let numerator1 = point_vector /*CROSS*/ line1_vector;
	let numerator2 = -point_vector /*CROSS*/ line2_vector;

	if (denominator == 0)
	{
		//Colinear lines
		if (numerator == 0)
		{
			//Check if they intersect
		}
		//Parallel and non-intersecting
		else
		{
			return false;
		}
	}
	else
	{
		let t = numerator2/denominator
		let u = numerator1/denominator
		if ((0 <= t) && (t <= 1) && (0 <= u) && (u <= 1))
		{
			return true;
		}
		else
		{
			return false;
		}
	}
}
*/
