/*
This file is part of MOSAIC.

MOSAIC is free software: you can redistribute it and/or modify it under 
the terms of the GNU General Public License as published by the Free 
Software Foundation, either version 3 of the License, or any later version.

MOSAIC is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; 
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR 
PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with 
MOSAIC. If not, see <https://www.gnu.org/licenses/>.
*/

// ANALYSIS ACTS AS THE CONDUCTOR FOR MOSAIC
// Once the relevent data has been extracted into the UMD - we then "run MOSAIC" which includes:

// 1. Anchor calculation
// 2. Centering landmarks
// 3. Pose correction
// 4. Landmark euclidean distance (from origin)
// 5. Landmark angle distance
// 6. Curvature calculation
// 7. Area calculation (bio and quadrant)

use crate::coreMeasurements::anchor::anchor::{AnchorCoordinate};

use std::path::Path

pub struct Anchor {
    x: Vec<f64>,
    y: Vec<f64>,
    z: Vec<f64>,
    //x_uncertainty: Vec<f64>, will add when we add calibration 
    //y_uncertainty: Vec<f64>,
    //z_uncertainty: Vec<f64>,
}

fn run_anchor(coordinate_data) -> AnchorCoordinate{
    // we pass a list of points and return that
}

pub struct CoreMeasurementsData {
    anchor: Anchor,
    // we will add the other modules to the struct as they are made
}

impl CoreMeasurementsData {
    fn Anchor(UMD_path) {
        prinln!("Hello")
        // we use a for loop to calculate the anchor points per frame and then return that to the Anchor struct

        // definine vector sizes
        
    }
}



// ANCHOR 

