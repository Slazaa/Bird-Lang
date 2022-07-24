pub fn types() -> String {
	String::from("\
#ifndef BIRD_TYPES_H
#define BIRD_TYPES_H

typedef char int8;
typedef short int16;
typedef long int32;
typedef long long int64;

typedef unsigned char uint8;
typedef unsigned short uint16;
typedef unsigned long uint32;
typedef unsigned long long uint64;

typedef float float32;
typedef double float64;

#endif\
	")
}