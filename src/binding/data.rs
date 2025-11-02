use jni::JNIEnv;
use jni::objects::{JObject, JValueGen};
use eyre::Result;
use crate::binding::math::new_vec3i;
use crate::pathing::data::PathNode;

const PATHNODE_CLASS: &'static str = "com/genericbadname/s4mc/pathing/PathNode";
const PATHNODE_SIG: &'static str = "com/genericbadname/s4mc/pathing/PathNode/<init>";

pub fn new_pathnode<'local>(env: &mut JNIEnv<'local>, path_node: PathNode) -> Result<JObject<'local>> {
    let pathnode_class = env.find_class(PATHNODE_CLASS)?;
    let pos = new_vec3i(env, path_node.pos)?;
    Ok(env.new_object(pathnode_class, PATHNODE_SIG, &[
        JValueGen::Object(&pos),
    ])?)
}