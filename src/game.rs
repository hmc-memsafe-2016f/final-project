//Just a place to store implementation details for the actual game part.

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
