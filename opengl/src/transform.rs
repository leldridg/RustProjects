// a 3x3 matrix
#[repr(C)]
pub struct Mat3([f32 ; 9]);

impl Mat3 {
    pub fn new() -> Self {
        Mat3([
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ])
    }

    pub fn ptr(&self) -> *const f32 {
        self.0.as_ptr()
    }

    pub fn mult(&mut self, mat: Mat3) {
        *self = Mat3([
            mat.0[0] * self.0[0] + mat.0[1] * self.0[3] + mat.0[2] * self.0[6] , mat.0[0] * self.0[1] + mat.0[1] * self.0[4] + mat.0[2] * self.0[7] , mat.0[0] * self.0[2] + mat.0[1] * self.0[5] + mat.0[2] * self.0[8] ,
            mat.0[3] * self.0[0] + mat.0[4] * self.0[3] + mat.0[5] * self.0[6] , mat.0[3] * self.0[1] + mat.0[4] * self.0[4] + mat.0[5] * self.0[7] , mat.0[3] * self.0[2] + mat.0[4] * self.0[5] + mat.0[5] * self.0[8] ,
            mat.0[6] * self.0[0] + mat.0[7] * self.0[3] + mat.0[8] * self.0[6] , mat.0[6] * self.0[1] + mat.0[7] * self.0[4] + mat.0[8] * self.0[7] , mat.0[6] * self.0[2] + mat.0[7] * self.0[5] + mat.0[8] * self.0[8] ,
        ])
    }

    pub fn scale(&mut self, x_scale: f32, y_scale: f32) {
        self.mult(Mat3([
            x_scale , 0.0     , 0.0 ,
            0.0     , y_scale , 0.0 ,
            0.0     , 0.0     , 1.0 ,
        ]))
    }

    // clockwise rotation
    pub fn rotate(&mut self, angle: f32) {
        self.mult(Mat3([
            angle.cos()  , angle.sin() , 0.0 ,
            -angle.sin() , angle.cos() , 0.0 ,
            0.0          , 0.0         , 1.0 ,
        ]))
    }

    pub fn translate(&mut self, x_move: f32, y_move: f32) {
        self.mult(Mat3([
            1.0 , 0.0 , x_move ,
            0.0 , 1.0 , y_move ,
            0.0 , 0.0 , 1.0    ,
        ]))
    }
}

// a 4x4 matrix
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Mat4([f32 ; 16]);

impl Mat4 {
    pub fn new() -> Self {
        Mat4([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn ptr(&self) -> *const f32 {
        self.0.as_ptr()
    }

    pub fn vec_mult(&self, vec: [f32 ; 4]) -> [f32 ; 4] {
        [
            self.0[0] * vec[0] + self.0[1] * vec[1] + self.0[2] * vec[2] + self.0[3] * vec[3],
            self.0[4] * vec[0] + self.0[5] * vec[1] + self.0[6] * vec[2] + self.0[7] * vec[3],
            self.0[8] * vec[0] + self.0[9] * vec[1] + self.0[10] * vec[2] + self.0[11] * vec[3],
            self.0[12] * vec[0] + self.0[13] * vec[1] + self.0[14] * vec[2] + self.0[15] * vec[3]
        ]
    }

    pub fn mult(&mut self, mat: Mat4) {
        *self = Mat4([
            mat.0[0]  * self.0[0] + mat.0[1]  * self.0[4] + mat.0[2]  * self.0[8] + mat.0[3]  * self.0[12] , mat.0[0]  * self.0[1] + mat.0[1]  * self.0[5] + mat.0[2]  * self.0[9] + mat.0[3]  * self.0[13] , mat.0[0]  * self.0[2] + mat.0[1]  * self.0[6] + mat.0[2]  * self.0[10] + mat.0[3]  * self.0[14] , mat.0[0]  * self.0[3] + mat.0[1]  * self.0[7] + mat.0[2]  * self.0[11] + mat.0[3]  * self.0[15] ,
            mat.0[4]  * self.0[0] + mat.0[5]  * self.0[4] + mat.0[6]  * self.0[8] + mat.0[7]  * self.0[12] , mat.0[4]  * self.0[1] + mat.0[5]  * self.0[5] + mat.0[6]  * self.0[9] + mat.0[7]  * self.0[13] , mat.0[4]  * self.0[2] + mat.0[5]  * self.0[6] + mat.0[6]  * self.0[10] + mat.0[7]  * self.0[14] , mat.0[4]  * self.0[3] + mat.0[5]  * self.0[7] + mat.0[6]  * self.0[11] + mat.0[7]  * self.0[15] ,
            mat.0[8]  * self.0[0] + mat.0[9]  * self.0[4] + mat.0[10] * self.0[8] + mat.0[11] * self.0[12] , mat.0[8]  * self.0[1] + mat.0[9]  * self.0[5] + mat.0[10] * self.0[9] + mat.0[11] * self.0[13] , mat.0[8]  * self.0[2] + mat.0[9]  * self.0[6] + mat.0[10] * self.0[10] + mat.0[11] * self.0[14] , mat.0[8]  * self.0[3] + mat.0[9]  * self.0[7] + mat.0[10] * self.0[11] + mat.0[11] * self.0[15] ,
            mat.0[12] * self.0[0] + mat.0[13] * self.0[4] + mat.0[14] * self.0[8] + mat.0[15] * self.0[12] , mat.0[12] * self.0[1] + mat.0[13] * self.0[5] + mat.0[14] * self.0[9] + mat.0[15] * self.0[13] , mat.0[12] * self.0[2] + mat.0[13] * self.0[6] + mat.0[14] * self.0[10] + mat.0[15] * self.0[14] , mat.0[12] * self.0[3] + mat.0[13] * self.0[7] + mat.0[14] * self.0[11] + mat.0[15] * self.0[15] ,
        ])
    }

    pub fn scale(&mut self, x_scale: f32, y_scale: f32, z_scale: f32) {
        self.mult(Mat4([
            x_scale , 0.0     , 0.0     , 0.0,
            0.0     , y_scale , 0.0     , 0.0,
            0.0     , 0.0     , z_scale , 0.0,
            0.0     , 0.0     , 0.0     , 1.0, 
        ]))
    }

    pub fn rotate_x(&mut self, angle: f32) {
        self.mult(Mat4([
            1.0 ,  0.0         , 0.0         , 0.0 ,
            0.0 ,  angle.cos() , angle.sin() , 0.0 , 
            0.0 , -angle.sin() , angle.cos() , 0.0 ,
            0.0 , 0.0          , 0.0         , 1.0 ,
        ]))
    }

    pub fn rotate_y(&mut self, angle: f32) {
        self.mult(Mat4([
            angle.cos()  , 0.0  , angle.sin() , 0.0 ,
            0.0          , 1.0  , 0.0         , 0.0 ,
            -angle.sin() , 0.0  , angle.cos() , 0.0 ,
            0.0          , 0.0  , 0.0         , 1.0 ,
        ]))
    }

    pub fn rotate_z(&mut self, angle: f32) {
        self.mult(Mat4([
            angle.cos()  , angle.sin() , 0.0 , 0.0 ,
            -angle.sin() , angle.cos() , 0.0 , 0.0 ,
            0.0          , 0.0         , 1.0 , 0.0 ,
            0.0          , 0.0         , 0.0 , 1.0 ,
        ]))
    }

    pub fn translate(&mut self, x_move: f32, y_move: f32, z_move: f32) {
        self.mult(Mat4([
            1.0 , 0.0 , 0.0 , x_move ,
            0.0 , 1.0 , 0.0 , y_move ,
            0.0 , 0.0 , 1.0 , z_move ,
            0.0 , 0.0 , 0.0 , 1.0    ,
        ]))
    }

    pub fn lookat(&mut self, eye_x: f32, eye_y: f32, eye_z: f32, target_x: f32, target_y: f32, target_z: f32, mut up_x: f32, mut up_y: f32, mut up_z: f32) {
        // forward vector
        let (mut f_x, mut f_y, mut f_z) = (eye_x - target_x, eye_y - target_y, eye_z - target_z);
        let invlen = 1.0 / (f_x * f_x + f_y * f_y + f_z * f_z).sqrt();
        (f_x, f_y, f_z) = (f_x * invlen, f_y *invlen, f_z * invlen);

        // left vector
        let (mut l_x, mut l_y, mut l_z) = (up_y * f_z - up_z * f_y, up_z * f_x - up_x * f_z, up_x * f_y - up_y * f_x);
        let invlen = 1.0 / (l_x * l_x + l_y * l_y + l_z * l_z).sqrt();
        (l_x, l_y, l_z) = (l_x * invlen, l_y * invlen, l_z * invlen);

        // up vector correction
        (up_x, up_y, up_z) = (f_y * l_z - f_z * l_y, f_z * l_x - f_x * l_z, f_x * l_y - f_y * l_x);

        *self = Self::new(); // resize matrix
        self.translate(-eye_x, -eye_y, -eye_z);
        self.mult(Mat4([
            l_x  , l_y  , l_z  , 0.0,
            up_x , up_y , up_z , 0.0,
            f_x  , f_y  , f_z  , 0.0,
            0.0  , 0.0  , 0.0  , 1.0, 
        ]));
    }

    pub fn project_orthographic(&mut self, l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) {
        *self = Mat4([
            2.0 / (r - l) , 0.0           , 0.0           , -(r + l) / (r - l) ,
            0.0           , 2.0 / (t - b) , 0.0           , -(t + b) / (t - b) ,
            0.0           , 0.0           , -2.0 / (f - n), -(f + n) / (f - n) ,
            0.0           , 0.0           , 0.0           , 1.0              ,
        ]);
    }

    pub fn project_perspective(&mut self, l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) {
        *self = Mat4([
            2.0 * n / (r - l) , 0.0               , (r + l) / (r - l) , 0.0,
            0.0               , 2.0 * n / (t - b) , (t + b) / (t - b) , 0.0,
            0.0               , 0.0               , -(f + n) / (f - n), -(2.0 * f * n) / (f - n),
            0.0               , 0.0               , -1.0              , 0.0,
        ]);
    }

    // inverse of a view matrix contructed using lookat
    pub fn inverse_view(&self) -> Mat4 {
        let m = &self.0;
        let mut inv = Mat4::new();

        // transpose rotation
        inv.0[0] = m[0];
        inv.0[1] = m[4];
        inv.0[2] = m[8];
        inv.0[4] = m[1];
        inv.0[5] = m[5];
        inv.0[6] = m[9];
        inv.0[8] = m[2];
        inv.0[9] = m[6];
        inv.0[10] = m[10];

        // negate the translation and apply the transposed rotation
        inv.0[12] = -(m[12] * m[0] + m[13] * m[1] + m[14] * m[2]);
        inv.0[13] = -(m[12] * m[4] + m[13] * m[5] + m[14] * m[6]);
        inv.0[14] = -(m[12] * m[8] + m[13] * m[9] + m[14] * m[10]);

        inv
    }

    // inverse of orthographic projection matrix
    pub fn inverse_orthographic(&self) -> Mat4 {
        let m = &self.0;
        Mat4([
            1.0 / m[0] , 0.0        , 0.0         , -m[12] / m[0]  ,
            0.0        , 1.0 / m[5] , 0.0         , -m[13] / m[5]  ,
            0.0        , 0.0        , 1.0 / m[10] , -m[14] / m[10] ,
            0.0        , 0.0        , 0.0         , 1.0            ,
        ])
    }

    // inverse of a perspective projection matrix
    pub fn inverse_perspective(&self) -> Mat4 {
        let m = &self.0;
        let mut inv = Mat4::new();

        inv.0[0] = 1.0 / m[0];
        inv.0[5] = 1.0 / m[5];
        inv.0[11] = 1.0 / m[14];
        inv.0[14] = 1.0 / m[11];
        inv.0[15] = -m[10] / (m[14] * m[11]);

        inv
    }
}