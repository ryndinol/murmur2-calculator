extern crate neon;
extern crate num_bigint;
extern crate num_traits;
extern crate murmurhash32;

use neon::prelude::*;
use neon::register_module;
use murmurhash32::murmurhash2;
use std::fs::File;
use std::io::Read;

struct MurmurTask {
    argument: String,
}

impl Task for MurmurTask {
    type Output = u32;
    type Error = String;
    type JsEvent = JsNumber;

    fn perform(&self) -> Result<u32, String> {
        let mut file = File::open(&self.argument).expect("Error opening file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Error opening file");
        buffer.retain(|&x| (x != 9 && x != 10 && x != 13 && x != 32));
        let d: &[u8] = buffer.as_ref();
        let murmur : u32 = murmurhash2(d);
        Ok(murmur)
    }

    fn complete(self, mut cx: TaskContext, result: Result<u32, String>) -> JsResult<JsNumber> {
        Ok(cx.number(result.unwrap()))
    }
}

fn murmur2_async(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let n = cx.argument::<JsString>(0)?.value() as String;
    let cb = cx.argument::<JsFunction>(1)?;
    let task = MurmurTask { argument: n };
    // println!("{}", task);
    task.schedule(cb);
    Ok(cx.undefined())
}

register_module!(mut m, {
    m.export_function("murmur2", murmur2_async)?;
    Ok(())
});
