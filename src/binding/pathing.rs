use jni::JNIEnv;
use jni::objects::{JClass, JObject, JValueGen};
use jni::sys::jobject;
use eyre::Result;
use crate::binding::jni::JNICompatible;
use crate::config::Configuration;
use crate::pathing::action::default_moveset;
use crate::pathing::algorithm::PathCalculator;
use crate::pathing::data::PathNode;
use crate::pathing::math::Vector3i;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_genericbadname_s4mc_pathing_PathCalculator_calculate
    <'local>(
    mut env: JNIEnv<'local>, _class: JClass<'local>,
    start: JObject<'local>,
    end: JObject<'local>) -> jobject {
    let try_this: Result<JObject<'local>> = (move || {
        let config = Configuration::new();
        let moves = default_moveset();

        let mut calc = PathCalculator::new(moves, &config);
        let start_vec = Vector3i::from_jni(&mut env, start)?;
        let end_vec = Vector3i::from_jni(&mut env, end)?;

        let out = match calc.calculate(start_vec, end_vec) {
            None => Vec::<PathNode>::new().to_jni(&mut env)?,
            Some(path) => {
                path.to_jni(&mut env)?
            }
        };

        Ok(out)
    })();

    match try_this {
        Ok(obj) => *obj,
        Err(e) => {
            eprintln!("Error while unwrapping path: {:?}", e);
            *JObject::null()
        }
    }
}