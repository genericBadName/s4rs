use jni::JNIEnv;
use jni::objects::{JObject, JValueGen};
use eyre::Result;
use crate::pathing::math::Vector3i;

const VEC3I_CLASS: &'static str = "com/genericbadname/s4mc/math/Vector3i";
const VEC3I_SIG: &'static str = "III";

pub fn map_vec3i<'local>(env: &mut JNIEnv<'local>, obj: JObject<'local>) -> Result<Vector3i> {
    let xv = env.call_method(&obj, "x", VEC3I_CLASS, &[])?.i()?;
    let yv = env.call_method(&obj, "y", VEC3I_CLASS, &[])?.i()?;
    let zv = env.call_method(&obj, "z", VEC3I_CLASS, &[])?.i()?;

    Ok(Vector3i {
        x: xv,
        y: yv,
        z: zv
    })
}

pub fn new_vec3i<'local>(env: &mut JNIEnv<'local>, vector3i: Vector3i) -> Result<JObject<'local>> {
    let vec3i_class = env.find_class(VEC3I_CLASS)?;
    Ok(env.new_object(vec3i_class, VEC3I_SIG, &[
        JValueGen::Int(vector3i.x),
        JValueGen::Int(vector3i.y),
        JValueGen::Int(vector3i.z)
    ])?)
}