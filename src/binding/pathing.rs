use jni::JNIEnv;
use jni::objects::{JClass, JObject};
use jni::sys::jobject;
use crate::binding::data::new_pathnode;
use crate::binding::math::{map_vec3i};
use crate::binding::util::new_list;
use crate::config::Configuration;
use crate::pathing::action::default_moveset;
use crate::pathing::algorithm::PathCalculator;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_genericbadname_s4mc_pathing_PathCalculator_calculate
    <'local>(
    mut env: JNIEnv<'local>, _class: JClass<'local>,
    start: JObject<'local>,
    end: JObject<'local>) -> jobject {
    let config = Configuration::new();
    let moves = default_moveset();

    let mut calc = PathCalculator::new(moves, &config);
    let start_vec = map_vec3i(&mut env, start);
    let end_vec = map_vec3i(&mut env, end);

    if start_vec.is_ok() && end_vec.is_ok() {
        match calc.calculate(start_vec.unwrap(), end_vec.unwrap()) {
            None => *new_list(&mut env, vec![]).expect("List construction failed on NONE"),
            Some(path) => {
                let pv = path.into_iter()
                    .map(|pn| new_pathnode(&mut env, pn)
                        .expect("PathNode construction failed")).collect();
                *new_list(&mut env, pv).expect("List construction failed on FULL LIST")
            }
        }
    } else {
        *new_list(&mut env, vec![]).expect("List construction failed on FAILED LIST")
    }
}