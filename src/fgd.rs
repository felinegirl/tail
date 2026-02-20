use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


// problems with this system
// spaces or " in dir will probably break this; if you do have spaces or "s, you're stupid

// if toolpp gets ported to rust or c, probably replace it with that

pub fn open(path: &String) -> Result<(), Box<dyn std::error::Error>> {

    
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut contentstriped = format!("");

    //strip comments
    for (mut line, i) in contents.lines().zip(1..) {
        let mut stline = format!("{}",line);
        let offset = line.find("//").unwrap_or(line.len());
        
        stline.replace_range(offset.., "");
        contentstriped += &format!("{}\n",stline);

    }
    
    //split by "Class Types and Properties"
    let splitedcontent: Vec<&str> = contentstriped.split('@').collect();

    //split by : then trims strings
    let mut contentz: Vec<Vec<&str>> = Vec::new();
    for line in splitedcontent {
        let line = line.split(':');
        let mut a = Vec::new();
        for line in line {
            match line
                .trim()
                .split_once([' ', '\n']) {
                    Some((name, end)) => {
                        a.push(name);
                        if(end !=""){
                            a.push(end);
                        }
                    },
                    None => (),
                }
            
        }
        contentz.push(a);
    }

    
    for lines in contentz {
        match lines.as_slice() {
            ["include", newpath] => {
                let newpath = newpath.replace('"', "");

                let path = Path::new(path);
                
                let thedir = path
                .parent()
                .ok_or("fucked up parent path in fgd")?
                .join(newpath);

                let location = thedir
                .to_str()
                .ok_or("fucked up conversion path | fgd")?;


                //this shouldn't be fucked up right?
                match open(&format!("{}", location)) {
                    Ok(_) => (),
                    Err(err) => {
                        let err = format!("while runnining included fgd:\n'{}' \n\n{}", location, err);
                        return Err(err.into());
                    },
                }
            }
            _=>()
        }

    }



    Ok(())
}