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


/*
ANCHOR Calculation

This file calculates the anchor as a reference point in the mouth by taking the average x and y coordinates
of points across the mouth and defining a centralised anchor point
*/

#[derive(Debug)]
pub struct AnchorCoordinate {
    timestamp: f32,
    x_anchor: f64,
    y_anchor: f64,
    z_anchor: f64,
    //Uncertainty to be added later:
    //x_anchor_uncertainty: Vec<f64>, 
    //y_anchor_uncertainty: Vec<f64>,
    //z_anchor_uncertainty: Vec<f64>,
}

impl AnchorCoordinate{
    /*pub fn construction(frame_count: u32) ->Self{
        // reserves space required for this data in the memory
        Self {
            timestamp: Vec::with_capacity(frame_count.try_into().unwrap()),
            x_anchor: Vec::with_capacity(frame_count.try_into().unwrap()),
            y_anchor: Vec::with_capacity(frame_count.try_into().unwrap()),
            z_anchor: Vec::with_capacity(frame_count.try_into().unwrap()),
            //x_anchor_uncertainty: Vec::with_capacity(frame_count.try_into().unwrap()),
            //y_anchor_uncertainty: Vec::with_capacity(frame_count.try_into().unwrap()),
            //z_anchor_uncertainty: Vec::with_capacity(frame_count.try_into().unwrap()),

        }
    }*/

    pub fn anchor(time: f32, coordinate_list: &[[f64; 3]]) -> Self {
        let mut x_sum: f64 = 0.0; //place holder for adding x vals
        let mut y_sum: f64 = 0.0; // place holder for adding y vals
        let mut z_sum: f64 = 0.0; // place holder for adding z vals


        for i in coordinate_list { // getting points
            println!("{:?}", i);
            x_sum += i[0];
            y_sum += i[1];
            z_sum += i[2];

        }

        let x = x_sum / coordinate_list.len() as f64;
        let y = y_sum / coordinate_list.len() as f64;
        let z = z_sum / coordinate_list.len() as f64;

        Self {
            timestamp: time,
            x_anchor: x,
            y_anchor: y,
            z_anchor: z,
        }

        //println!("X sum: {}\nY sum: {}\nZ sum: {}", x_sum, y_sum, z_sum);
        //println!("Anchor Coordinate: ({}, {}, {})", x_anchor, y_anchor, z_anchor);
    }
}