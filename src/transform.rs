use Screen;

use {Matrix4, Vector3, Unit};

impl<'a> Screen<'a> {
	/// Pre-multiply the current MVP transformation matrix with a matrix formed
	/// from the given values.
    pub fn apply_matrix(
        &mut self,
        n00: f32,
        n01: f32,
        n02: f32,
        n03: f32,
        n10: f32,
        n11: f32,
        n12: f32,
        n13: f32,
        n20: f32,
        n21: f32,
        n22: f32,
        n23: f32,
        n30: f32,
        n31: f32,
        n32: f32,
        n33: f32,
    ) {
        let m = Matrix4::new(
            n00,
            n01,
            n02,
            n03,
            n10,
            n11,
            n12,
            n13,
            n20,
            n21,
            n22,
            n23,
            n30,
            n31,
            n32,
            n33,
        );

        self.matrices.curr_matrix = m * self.matrices.curr_matrix;
    }

	/// Remove the current MVP transformation matrix from the stack and use the most
	/// recently used one instead.
    pub fn pop_matrix(&mut self) {
        match self.matrices.matrix_stack.pop() {
            Some(m) => self.matrices.curr_matrix = m,
            None => {
                self.matrices.curr_matrix = Matrix4::identity();
            }
        };
    }

	/// Push the current MVP transformation matrix onto the stack, so that it can be 
	/// saved for later. Useful for when you want to temporarily apply some rotation
	/// or translation to a single object and don't want to disturb the rest of the
	/// scene.
    pub fn push_matrix(&mut self) {
        self.matrices.matrix_stack.push(self.matrices.curr_matrix);
    }

	/// Remove the current MVP transfomation matrix and set it to the standard 4x4
	/// identity matrix.
    pub fn reset_matrix(&mut self) {
        self.matrices.curr_matrix = Matrix4::identity();
    }

	/// Pre-multiply the current MVP transformation matrix by a rotation matrix which
	/// is derived from a rotation angle about a vector in the direction (x, y, z).
    pub fn rotate(&mut self, angle: f32, x: f32, y: f32, z: f32) {
        // let m = Matrix4::new(
        //     angle.cos(),
        //     -angle.sin(),
        //     0.,
        //     0.,
        //     angle.sin(),
        //     angle.cos(),
        //     0.,
        //     0.,
        //     0.,
        //     0.,
        //     1.,
        //     0.,
        //     0.,
        //     0.,
        //     0.,
        //     1.,
        // );
        let m = Matrix4::from_axis_angle(&Unit::new_unchecked(Vector3::new(x, y, z)), angle);

        self.matrices.curr_matrix = m * self.matrices.curr_matrix;
    }

	/// Apply a rotation matrix for a given angle around the x-axis to the current MVP
	/// transformation matrix.
    pub fn rotate_x(&mut self, angle: f32) {
        let m = Matrix4::from_axis_angle(&Unit::new_unchecked(Vector3::new(1., 0., 0.)), angle);
        self.matrices.curr_matrix = m * self.matrices.curr_matrix;
    }

	/// Apply a rotation matrix for a given angle around the y-axis to the current MVP
	/// transformation matrix.
    pub fn rotate_y(&mut self, angle: f32) {
        let m = Matrix4::from_axis_angle(&Unit::new_unchecked(Vector3::new(0., 1., 0.)), angle);
        self.matrices.curr_matrix = m * self.matrices.curr_matrix;
    }

	/// Apply a rotation matrix for a given angle around the z-axis to the current MVP
	/// transformation matrix.
    pub fn rotate_z(&mut self, angle: f32) {
        let m = Matrix4::from_axis_angle(&Unit::new_unchecked(Vector3::new(0., 0., 1.)), angle);
        self.matrices.curr_matrix = m * self.matrices.curr_matrix;
    }

	/// Scale the scene along the x-, y-, and z-axes by applying a matrix derived from
	/// these values to the current MVP transformation matrix.
    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        // let m = Matrix4::new(x, 0., 0., 0., 0., y, 0., 0., 0., 0., z, 0., 0., 0., 0., 1.);

        let _ = self.matrices.curr_matrix.append_nonuniform_scaling(
            &Vector3::new(x, y, z),
        ); //* self.matrices.curr_matrix;
    }

	/// Derive a matrix that applies shear for a given angle the scene about the x-axis
	/// and apply it to the current MVP transformation matrix.
    pub fn shear_x(&mut self, angle: f32) {
        let m = Matrix4::new(
            1.,
            angle.tan(),
            0.,
            0.,
            0.,
            1.,
            0.,
            0.,
            0.,
            0.,
            1.,
            0.,
            0.,
            0.,
            0.,
            1.,
        );

        self.matrices.curr_matrix = m * self.matrices.curr_matrix;
    }

	/// Derive a matrix that applies shear for a given angle the scene about the y-axis
	/// and apply it to the current MVP transformation matrix.
    pub fn shear_y(&mut self, angle: f32) {
        let m = Matrix4::new(
            1.,
            0.,
            0.,
            0.,
            angle.tan(),
            1.,
            0.,
            0.,
            0.,
            0.,
            1.,
            0.,
            0.,
            0.,
            0.,
            1.,
        );

        self.matrices.curr_matrix = m * self.matrices.curr_matrix;
    }

	/// Derive a translation matrix from the given (x, y, z) vector and apply it to the
	/// current MVP transformation matrix.
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        let m = Matrix4::new(1., 0., 0., x, 0., 1., 0., y, 0., 0., 1., z, 0., 0., 0., 1.);

        self.matrices.curr_matrix = m * self.matrices.curr_matrix;
    }

	/// Print out the current MVP transformation matrix.
    pub fn print_matrix(&self) {
        println!("{:?}", self.matrices.curr_matrix);
    }
}
