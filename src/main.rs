use std::{
    env::args,
    error::Error,
    ffi::{OsStr, OsString},
    fs,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    process::Command,
    str,
};

use serde::Serialize;
use symbolic_debuginfo::{elf::ElfObject, Function};
use symbolic_demangle::{Demangle, DemangleOptions};

fn get_test_list<S: AsRef<OsStr>>(executable: S) -> Vec<String> {
    let output = Command::new(executable).arg("--list").output().unwrap();

    String::from_utf8_lossy(&output.stdout)
        .to_string()
        .lines()
        .filter_map(|x| x.strip_suffix(": test"))
        .map(|x| format!("::{x}"))
        .collect::<Vec<_>>()
}

#[derive(Debug, Serialize)]
struct FunctionLocation {
    name: String,
    compile_dir: PathBuf,
    file: PathBuf,
    line: u64,
}

impl std::fmt::Display for FunctionLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}:{}", self.name, self.compile_dir.join(&self.file).display(), self.line)
    }
}


fn extract_name_and_location(fun: &Function<'_>) -> Result<FunctionLocation, Box<dyn Error>> {
    let name = fun
        .name
        .demangle(DemangleOptions::name_only())
        .ok_or("demangling failed")?;
    let compile_dir = OsStr::from_bytes(fun.compilation_dir).to_owned();
    let compile_dir = PathBuf::from(compile_dir);
    let location = fun.lines.first().unwrap();
    let fdir = location.file.dir_str();
    let fname = location.file.name_str();
    let file = PathBuf::from(fdir.as_ref()).join(fname.as_ref());
    let line = location.line;

    Ok(FunctionLocation {
        name,
        compile_dir,
        file,
        line,
    })
}

fn main() {
    let exe = args().nth(1).unwrap_or_else(|| "testdata/api".to_string());

    let mut test_list = get_test_list(&exe);

    let elf = fs::File::open(exe).unwrap();
    let map = unsafe { memmap2::Mmap::map(&elf).unwrap() };

    let object = ElfObject::parse(&map).unwrap();
    let session = object.debug_session().unwrap();

    let mut test_functions = Vec::new();

    for fun in session.functions() {
        let f = fun.unwrap();
        let fun = extract_name_and_location(&f).unwrap();

        if let Some(idx) = test_list.iter().position(|x| fun.name.ends_with(x)) {
            println!("{fun}");
            println!("{}", serde_json::to_string(&fun).unwrap());
            test_functions.push(fun);

            test_list.remove(idx);
        }

        if test_list.is_empty() {
            break;
        }
    }

    for missing_test in test_list {
        eprintln!("not found: {missing_test}");
    }
}
