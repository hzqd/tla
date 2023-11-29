use std::fs;
use aoko::no_std::pipelines::{tap::Tap, pipe::Pipe};
use tar::{Archive, Builder};

fn mark_file_or_dir(r#in: &str, vec: &mut Vec<u8>) {
    let meta = fs::metadata(r#in).unwrap();
    if meta.is_dir() {
        vec.push(0)
    } else if meta.is_file() {
        vec.push(1)
    } else {
        panic!("unsupported file type")
    }
}

fn judge_file_or_dir(last: u8, byt: Vec<u8>, f1: impl FnOnce(&mut Archive<&[u8]>), f2: impl FnOnce(&mut Archive<&[u8]>)) {
    let tar = &mut Archive::new(byt.as_slice());
    match last {
        0 => f1(tar),
        1 => f2(tar),
        _ => panic!("file type error")
    }
}

pub fn archive(r#in: &str, f: impl FnOnce(Vec<u8>) -> Vec<u8>) {
    Builder::new(vec![])
        .tap_mut(|b| b.append_dir_all("", r#in).unwrap_or_else(|_| b.append_path(r#in).unwrap()))
        .pipe(|b| b.into_inner().unwrap())
        .pipe(f)
        .tap_mut(|vec| mark_file_or_dir(r#in, vec)) // 0: dir, 1: file
        .pipe(|byt| fs::write(format!("{in}.tla2"), byt).unwrap())
}

pub fn restore(r#in: &str, f: impl FnOnce(Vec<u8>) -> Vec<u8>) {
    fs::read(r#in).unwrap()
        .pipe(|mut v| (v.pop().unwrap(), v))
        .pipe(|(last, data)| (f(data), last))
        .pipe(|(byt, last)| r#in.trim_end_matches(".tla2").pipe(|name|
            judge_file_or_dir(last, byt, |tar| tar.unpack(name).unwrap(),
                |tar| tar.entries().unwrap().for_each(|file| {
                    file.unwrap()
                        .unpack(name).unwrap();
                })
            )
        ))
}