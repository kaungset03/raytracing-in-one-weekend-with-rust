// #ifndef RTWEEKEND
// H
// _
// #define RTWEEKEND
// _
// H
// #include <cmath>
// #include <iostream>
// #include <limits>
// #include <memory>
// // C++ Std Usings
// using std::make
// _
// shared;
// using std::shared
// _ptr;
// // Constants
// const double infinity = std::numeric
// _
// limits<double>::infinity();
// const double pi = 3.1415926535897932385;
// // Utility Functions
// inline double degrees
// to
// _
// _
// radians(double degrees) {
// return degrees * pi / 180.0;
// }
// // Common Headers
// #include "color.h"
// #include "ray.h"
// #include "vec3.h"
// #endif

// Constants
pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

// Utility Functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
