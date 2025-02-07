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