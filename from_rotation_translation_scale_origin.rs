fn from_rotation_translation_scale_origin(
out: &mut[f32], q: &[f32], v: &[f32], s: &[f32], o: &[f32]){
 // Quaternion math
  let x:f32 = q[0];
    let y:f32 = q[1];
    let z:f32 = q[2];
    let w:f32 = q[3];
  let x2:f32 = x + x;
  let y2:f32 = y + y;
  let z2:f32 = z + z;
  let xx:f32 = x * x2;
  let xy:f32 = x * y2;
  let xz:f32 = x * z2;
  let yy:f32 = y * y2;
  let yz:f32 = y * z2;
  let zz:f32 = z * z2;
  let wx:f32 = w * x2;
  let wy:f32 = w * y2;
  let wz:f32 = w * z2;
  let sx:f32 = s[0];
  let sy:f32 = s[1];
  let sz:f32 = s[2];
  let ox:f32 = o[0];
  let oy:f32 = o[1];
  let oz:f32 = o[2];
  let out0:f32 = (1. - (yy + zz)) * sx;
  let out1:f32 = (xy + wz) * sx;
  let out2:f32 = (xz - wy) * sx;
  let out4:f32 = (xy - wz) * sy;
  let out5:f32 = (1. - (xx + zz)) * sy;
  let out6:f32 = (yz + wx) * sy;
  let out8:f32 = (xz + wy) * sz;
  let out9:f32 = (yz - wx) * sz;
  let out10:f32 = (1. - (xx + yy)) * sz;
  
  out[0] = out0;
  out[1] = out1;
  out[2] = out2;
  out[3] = 0.;
  out[4] = out4;
  out[5] = out5;
  out[6] = out6;
  out[7] = 0.;
  out[8] = out8;
  out[9] = out9;
  out[10] = out10;
  out[11] = 0.;
  out[12] = v[0] + ox - (out0 * ox + out4 * oy + out8 * oz);
  out[13] = v[1] + oy - (out1 * ox + out5 * oy + out9 * oz);
  out[14] = v[2] + oz - (out2 * ox + out6 * oy + out10 * oz);
  out[15] = 1.;
}

fn main() {
    let mut out: [f32; 16] = [0.; 16];
    let translation: [f32; 3] = [0.,0.,0.];
    let rotation: [f32; 4] = [1.57, 0., 0.32, 0.];
    let scale: [f32; 3] = [1., 1., 1.];
 
    from_rotation_translation_scale_origin(&mut out, &rotation, &translation, &scale, &translation);
   println!("{:?}", out);
}
