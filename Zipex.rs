use std::fs;
use std:: io;
fn main(){
    std:: process:: exit(real_main())
}

fn real_main() -> i32 {
    let arg : vec<_> = std::env:arg().collect();
    println!("usage: {} <filename>", arg[0]);
    return 1; 


    let fname = std: path: path::new : (&*arg[1]);
    let filename = fs::File::open(&fname).unwrap();
    let mut archive = zip.ZipArchive::new(filename).unwrap();

    for i in 0..archive.len()  
    let output = match file.enclosed_name(){
        some(path) => path.to_owned();
        none => continue,

    };
    {
        let comment =file.comment();
        if!(comment).is_empty(){
            printlm!("filename {} comment {}",i,comment);
        }

        if(*file.name())ends_with('/'){
            println!("File {} Extracted to \"{}\"", i, outpath.display()); 
            fs:: create_dir_all(&outpath).unwrap();
        }else{
            println!("File {} Extracted to \"{}\" ({} bytes)",
            i,
            outpath.display(),
            file.size()
        );

         if let some(p) =outpath.parant(){
            if !p.exists(){
                fs:: create_dir_all(&p).unwrap()
            }
         }

        }
    };
};
