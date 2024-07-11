/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parsing.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: cpapot <cpapot@student.42lyon.fr >         +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/06/25 22:48:41 by cpapot            #+#    #+#             */
/*   Updated: 2024/07/11 19:02:42 by cpapot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub struct Objdata {
	pub vertex:Vec<(f64, f64, f64)>,
	normal:Vec<(f64, f64, f64)>,
	pub face:Vec<(f64, f64, f64)>
 }

macro_rules! get_value {
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
	for i in 0..data.face.len()
	{
		println!("face: x:{:?}, y:{:?}, z:{:?}", data.face[i].0, data.face[i].1, data.face[i].2);
	}
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
	let mut data = Objdata{vertex:Vec::new(), normal:Vec::new(), face:Vec::new()};
	let lines = content.lines();
	let mut lineindex = 0;
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
			let x = get_value!(lineindex, words);
			let y = get_value!(lineindex, words);
			let z = get_value!(lineindex, words);
			data.vertex.push((x, y, z));
		}
		if word == "f"
		{
			let size = words.clone().count();
			let mut index = 3;
			if size < 3 {return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "face is not a 3D point"));}

			let x = get_value!(lineindex, words);
			let y = get_value!(lineindex, words);
			let z = get_value!(lineindex, words);
			data.face.push((x, y, z));
			while index < size
			{
				let y = z;
				let z = words.next().unwrap().parse::<f64>().unwrap();
				data.face.push((x, y, z));
				index += 1;
			}
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
