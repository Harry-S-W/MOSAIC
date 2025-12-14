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
I want MOSAIC to be very modular, moreso than the Python version and with that in mind, there
is a dedicated analysis folder which is sort of the conductor of the entire program.

I plan to have it follow this sort of process:

    1. Rea the raw data
    2. Take the specific driver type and convert the raw data to the UMD (Universal Measurment Data) which is
       the file structure used by MOSAIC to make it as adaptable as possible.
    3. Then, it runs the core measurements
    4. It then analyzes speech sounds using pareselmouth (the only real python portion of MOSAIC left)
    5. Then calculates all the complex measurements requested by the user
    6. Comparitive file displaying speech sounds to oromotor movement exported as a csv file for readability
*/