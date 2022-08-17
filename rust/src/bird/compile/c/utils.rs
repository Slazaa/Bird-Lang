use std::path::Path;
use std::fs::{self, OpenOptions};
use std::io::Write;

use crate::bird::feedback::{Feedback, Error};

use super::OUTPUT_FOLDER;

pub fn utils_file() -> Result<(), Feedback> {
	if !Path::new(&format!("{}/bird", OUTPUT_FOLDER)).exists() && fs::create_dir(&format!("{}/bird", OUTPUT_FOLDER)).is_err() {
		return Err(Error::unspecified(&format!("Failed creating '{}/bird' directory", OUTPUT_FOLDER)));
	}

	let mut types_file = match OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(&format!("{}/bird/c_utils.h", OUTPUT_FOLDER))
	{
		Ok(x) => x,
		Err(_) => return Err(Error::unspecified("Failed creating 'bird/c_utils.h' file")) 
	};

	if write!(types_file, "\
#ifndef BIRD_C_UTILS_H
#define BIRD_C_UTILS_H

// Types
typedef enum {{ false, true }} bool;

typedef char i8;
typedef short i16;
typedef long i32;
typedef long long i64;

typedef unsigned int uint;
typedef unsigned char u8;
typedef unsigned short u16;
typedef unsigned long u32;
typedef unsigned long long u64;

typedef float f32;
typedef double f64;

#endif\
		").is_err() {
			return Err(Error::unspecified("Failed writing to 'bird/types.h' file"));
		}

	Ok(())
}