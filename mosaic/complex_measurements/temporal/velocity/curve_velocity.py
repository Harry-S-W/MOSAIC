"""
Takes the velocity of the change in curvature using the curve points and coeffs


WHAT WE CAN DO:

To see how much the curve just moves overall, we can calculate the current curve and the curve before it and
calculate the area in between the two curves

Then, to calculate how much the curve changes, we can compare the slop of each curve and get the difference
"""

from pathlib import Path
import pandas as pd
from mosaic.config import OUTER_BEZ_CURVE_LIST, INNER_BEZ_CURVE_LIST

class CurveVelocity:
    def __init__(self, src: str | Path | pd.DataFrame) -> str | None:
        if isinstance(src, (str, Path)):
            self.landmarks = pd.read_csv(src)
        elif isinstance(src, pd.DataFrame):
            self.landmarks = src
        else:
            raise TypeError("landmarks must be a CSV path or a pandas DataFrame")

    @staticmethod
    def _getting_data(landmarks, row):

        df = landmarks.set_index("frame")
        df_row = df.loc[row]

        curves = []
        for curve in OUTER_BEZ_CURVE_LIST:
            curve_vals = []
            time = df_row["timestamp"]
            curve_vals.append(time)
            for curve_val in curve:
                curve_val_flt = df_row[curve_val]
                curve_vals.append(curve_val_flt)

            curves.append(curve_vals)

        return curves

    def _get_cubic_curve_gradient(self, curve, t):
        """
        I think the best way of getting the "velocity" of a coeff is to compare the gradients of the 4 control points (P0-P3).
        that gives me the how the coeff really changes over time.

        this func is for cubic curves

        :return: Single curve gradient - must pass one curve at a time
        """
        coeffs = []
        for coeff in curve[1:]:
            coeffs.append(coeff)

            # derivative of cubic coeff function:

        X_t = (3 * coeffs[0] * (t ** 2)) + (2 * coeffs[2] * t) + (coeffs[4] * t)
        Y_t = (3 * coeffs[1] * (t ** 2)) + (2 * coeffs[3] * t) + (coeffs[5] * t)

        slope = Y_t / X_t

        # then we make return it like [time, slope} so it makes getting velo easier

        slope_velo = [curve[0], float(slope)]
        print(slope_velo)
        return slope


    def _get_quadratic_curve_gradient(self, curve, t):
        # coeff will be a series of 4 lists which contain the x, y coeffs
        slope = []
        for coeff in curve:

            # differential of quadratic coeff function - will code later:

            X_t = 1
            Y_t = 1

            slope.append(Y_t / X_t)
        return slope

    def curve_velocity(self, row: int):
        """
        We can call the function to calculate the curve slope at each point of t
        """

        cubic_curve_velocity = []
        quadratic_curve_velocity = []

        if row == 1:
            cubic_curve_velocity = 0
            quadratic_curve_velocity = 0

            return cubic_curve_velocity, quadratic_curve_velocity


        """
        CUBIC CURVE VELOCITY:
        """

        cubic_t = [0, 1/3, 2/3, 1]

        current_cubic_curve = self._getting_data(self.landmarks, row)
        previous_cubic_curve = self._getting_data(self.landmarks, row-1)

        current_cubic_curve_gradients = []

        previous_cubic_curve_gradients = []



        for curve, t in zip(current_cubic_curve, cubic_t):
            current_cubic_curve_gradients.append(self._get_cubic_curve_gradient(curve, t))

        for curve, t in zip(previous_cubic_curve, cubic_t):
            previous_cubic_curve_gradients.append(self._get_cubic_curve_gradient(curve, t))

        # Now we have a list of the gradients for all 4 control points along the current and previous curve

        cubic_gradients = []

        for curr_curve, prev_curve in zip(current_cubic_curve, previous_cubic_curve):
            cubic_gradient_difference = []
            for curr_gradient, prev_gradient in zip(curr_curve, prev_curve):
                cubic_gradient_difference.append(curr_gradient - prev_gradient)

            cubic_gradients.append(cubic_gradient_difference)

        # Now that we have a list of the gradient displacements, we can iterate through each list and divide it by time to get gradient/s


        return cubic_gradients

        # now we can calculate the difference in time and then divide the gradient displacement by time to get gradient/s



        # we can do the same for quadratic curves








    #def curve_area_calculation(self, curve_data):
        """
        This function essentially takes the curve in the current frame and the curve from the previous frame and
        calculates the area inbetween the two curves. This is then used to see how the curve has increased over
        time.

        Mapping overtime, we can see how active the curves are over time. A hilly graph means the curve moved a lot
        and a flatter graph means the curve didn't move much at all
        """
        #pass

    #def curve_slope_calculator(self, curve_data):
        """
        This calculates the slop of the curve

        Mapping over time we could see how curvy the curve is over time - a hillier graph means that curve was more curvey
        over time or saw increases/decreases in curviness and a flatter graph indicates the curve didn't change its curvature very much
        """
        #pass

    #def mid_curve(self, curve_data):
        """
        This calculates how the curve has moved from the previous frame to the current by
        subtracting the previous curve from the current.

        When mapping over time, we can see how much the curve changes and see if it changes a lot or a littl (still
        working on th logic for this)
        """
        #pass
