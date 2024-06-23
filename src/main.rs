struct Structure {
	field1:i32,
	field2:i32,
	field3:i32
 }

fn main()
{
	let float:i32 = 1.00005 as i32;
	println!("value {}", float);
	let float:i32 = 4.00005 as i32;
	println!("value {}", float);


	let mut test:f32 = 1.00005 as f32;
	println!("value {}", test);
	test = 4.00005 as f32;
	println!("value {}", test);


	let tuple:(f64, f64, f64) = (1.00, 2.0, 3.0);
	println!("value {:?}", tuple);
	println!("value {:?}", tuple.0);

	let arr:[i32; 5] = [1, 2, 3, 4, 5];
	println!("value {:?}", arr);
	println!("value {}", arr.len());
	println!("value {:?}", arr[1]);

	let stru:Structure = Structure{field1:15, field2:25, field3:35};
	println!("value {:?}", stru.field1);

	panic!("This is a panic message");
}
