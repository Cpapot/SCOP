/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parsing.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cpapot <cpapot@student.42lyon.fr >         +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/06/25 22:48:41 by cpapot            #+#    #+#             */
/*   Updated: 2024/07/22 14:50:20 by cpapot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub struct Objdata {
	pub vertex:Vec<(f64, f64, f64)>,
	pub normal:Vec<(f64, f64, f64)>,
	pub indexs:Vec<u16>,
 }

 fn calculate_normal(v1: (f64, f64, f64), v2: (f64, f64, f64), v3: (f64, f64, f64)) -> (f64, f64, f64) {
    let u = (
        v2.0 - v1.0,
        v2.1 - v1.1,
        v2.2 - v1.2,
    );
    let v = (
        v3.0 - v1.0,
        v3.1 - v1.1,
        v3.2 - v1.2,
    );

    let normal = (
        u.1 * v.2 - u.2 * v.1,
        u.2 * v.0 - u.0 * v.2,
        u.0 * v.1 - u.1 * v.0,
    );

    let length = (normal.0 * normal.0 + normal.1 * normal.1 + normal.2 * normal.2).sqrt();
    (
        normal.0 / length,
        normal.1 / length,
        normal.2 / length,
    )
}

 macro_rules! get_u16_value {
	($lineindex:expr, $words:expr) => {
		$words.next().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("invalid value at line: {}", $lineindex)))?
		.parse::<u32>().map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("invalid value at line: {}", $lineindex)))?
	};
}

macro_rules! get_f64_value {
	($lineindex:expr, $words:expr) => {
		$words.next().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("invalid value at line: {}", $lineindex)))?
		.parse::<f64>().map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("invalid value at line: {}", $lineindex)))?
	};
}

pub fn print_objdata(data : &Objdata)
{
	for i in 0..data.vertex.len()
	{
		println!("vertex: x:{:?}, y:{:?}, z:{:?}", data.vertex[i].0, data.vertex[i].1, data.vertex[i].2);
	}
	/*for i in 0..data.face.len()
	{
		println!("face: x:{:?}, y:{:?}, z:{:?}", data.face[i].0, data.face[i].1, data.face[i].2);
	}*/
}

fn	read_file(path: &str) -> Result<String, std::io::Error>
{
	use std::fs::File;
	use std::io::Read;

	let mut file = File::open(path)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	Ok(contents)
}

fn fill_struct(content: &String) -> Result<Objdata, std::io::Error>
{
	let mut data = Objdata{vertex:Vec::new(), normal:Vec::new(), indexs:Vec::new()};
	let lines = content.lines();
	let mut lineindex = 0;
	let mut isnormal: i32 = 0;
	for line in lines
	{
		lineindex += 1;
		let mut words = line.split_whitespace();
		let word = words.next().unwrap_or("None");
		if word == "None" || word == "#" {continue;}
		if word == "v"
		{
			let size: usize = words.clone().count();
			if size != 3 {return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "vertex is not a 3D point"));}
			let x = get_f64_value!(lineindex, words);
			let y = get_f64_value!(lineindex, words);
			let z = get_f64_value!(lineindex, words);
			data.vertex.push((x, y, z));
		}
		if word == "f"
		{
			let size = words.clone().count();
			let mut index = 0;
			if size < 3 {return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "face is not a 3D point"));}

			while index < size
			{
				let value: u32 = get_u16_value!(lineindex, words);
				data.indexs.push(value as u16 - 1);
				index += 1;
			}
		}
		if word == "vn"
		{
			isnormal = 1;
		}
	}
	if isnormal == 0
	{
		for face in data.indexs.chunks(3)
		{
			let v1 = data.vertex[face[0] as usize];
			let v2 = data.vertex[face[1] as usize];
			let v3 = data.vertex[face[2] as usize];
			let normal = calculate_normal(v1, v2, v3);
			data.normal.push(normal);
		}
	}
	return Ok(data);
}

pub fn parse_obj(path: &str) -> Result<Objdata, std::io::Error>
{
	if path.ends_with(".obj") == false {
		return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "file is not an obj file"));}
	match read_file(path)
	{
		Ok(contents) =>
		{
			let result = fill_struct(&contents);
			return result;
		}
		Err(e) => {Err(e)}
	}
}
