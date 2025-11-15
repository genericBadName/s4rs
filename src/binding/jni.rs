use eyre::Result;
use jni::objects::{JObject, JValueGen};
use jni::JNIEnv;

/// A type which is easily transferable between a Rust representation and a Java `Object`.
/// For `struct`s, these are usually going to be `Record`s as they are the closest
/// analogue to a pure-data class.
pub trait JNICompatible<'local> {
    /// Java class of this type.
    const CLASS: &'static str;

    /// Constructs the corresponding `JObject` of this type.
    fn to_jni(&self, env: &mut JNIEnv<'local>) -> Result<JObject<'local>>;
    /// Constructs this type from a `JObject` that may or may not contain valid values.
    fn from_jni(env: &mut JNIEnv<'local>, object: JObject<'local>) -> Result<Self> where Self: Sized;
}

// Represent a Vec as a list, close enough (I know java has vectors, but I don't care)
impl <'local, T: JNICompatible<'local>> JNICompatible<'local> for Vec<T> {
    const CLASS: &'static str = "java/util/ArrayList";

    fn to_jni(&self, env: &mut JNIEnv<'local>) -> Result<JObject<'local>> {
        let list_class = env.find_class(Self::CLASS)?;
        let list_obj = env.new_object(list_class, "()V", &[])?;

        for object in self.iter() {
            let val = JValueGen::Object(&object.to_jni(env)?);
            env.call_method(&list_obj, "add", "<T:Ljava/lang/Object;>", &[
                val
            ])?;
        }

        Ok(list_obj)
    }

    fn from_jni(env: &mut JNIEnv<'local>, object: JObject<'local>) -> Result<Self>
    where
        Self: Sized
    {
        todo!()
    }
}