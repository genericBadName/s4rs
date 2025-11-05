use jni::JNIEnv;
use jni::objects::{JObject, JValueGen};
use eyre::Result;
use log::info;

const LIST_CLASS: &'static str = "java/util/ArrayList";
const LIST_CTOR: &'static str = "()V";

pub fn new_list<'local>(env: &mut JNIEnv<'local>, objs: Vec<JObject<'local>>) -> Result<JObject<'local>> {
    let list_class = env.find_class(LIST_CLASS)?;
    let list_obj = env.new_object(list_class, LIST_CTOR, &[])?;

    for object in objs.iter() {
        env.call_method(&list_obj, "add", "<T:Ljava/lang/Object;>", &[
            JValueGen::Object(object)
        ])?;
    }

    Ok(list_obj)
}