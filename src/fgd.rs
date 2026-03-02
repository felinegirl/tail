use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use eframe::egui::TextBuffer;
use eframe::egui::ahash::HashMap;

pub enum PropertyType {

    //basic (they don't know they're still alive)
    string,
    integer,
    float,

    //stupid
    boolean(String,String),
    flags, //touch later
    choices(Vec<(String, String)>),

    //magic crap
    angle,
    angle_negative_pitch,
    axis,
    color255,
    color1,
    decal,
    effect,
    filterclass,
    instance,
    instance_file,
    instance_parm,
    instance_variable,
    material,
    node_dest,
    node_id,
    npcclasses,
    orgin,
    particle,
    particlesystem,
    pointentityclass,
    scale,
    scene,
    script, // <=l4d
    scriptlist, // <=l4d
    shader,
    sidelist, // !!!
    sky,
    sound,
    soundscape,
    sprite,
    studio,
    target_destination, // !!!
    target_name_or_class,
    target_source,
    vecline,
    vector,
    
    unknown(String) // name
}

pub struct BaseClass {
    Name: String,

}

pub struct SolidClass {
    Name: String,
    properties: Vec<(PropertyType, String, String)> // property Type, name, desc
}






fn parseproperty(leek: String) -> Result<(String, Vec<String>), Box<dyn std::error::Error>> {

    //split from (
    let mut frombit: Vec<&str> = leek.split('(').collect();

    //remove ) from end, probably a stupid way of doing it
    let mut ve = frombit.last_mut().ok_or("err")?;
    let binding = ve.replace(')', "");
    let mut binding = binding.as_str();
    ve = &mut binding;

    let mut args: Vec<String> = Vec::new();
    for bit in ve.split(",") {
        args.push(bit.trim().to_string());
    }

    Ok((frombit[0].to_string(), args))
}

// problems with this system
// spaces or " in dir will maybe break this; if you do have "s, you're stupid

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
        contentstriped += &format!("{}\n",stline.trim());

    }
    
    //split by "Class Types and Properties"
    let splitedcontent: Vec<&str> = contentstriped.split(['@']).collect();

    //split by : then trims strings
    let mut contentz: Vec<Vec<String>> = Vec::new();
    for line in splitedcontent {
        let mut a: Vec<String> = Vec::new();

        let avsd: Vec<char> = line.chars().collect();
        let mut newstring: String = String::new();

        //if we are eaching through qoutes
        let mut instring = false;
        let mut lastinsect: char = '"';
        let mut squarebraketslevel:i8 = 0;

        //
        //  handeling inside square brackets
        //
        let mut insidesquarebraketqoute = false;
        for la in avsd {

            if instring && lastinsect =='[' {   

                if !insidesquarebraketqoute {

                    if 
                        la==']' && squarebraketslevel==1
                    { //close
                        instring = false;

                        let splitedted: Vec<&str> = newstring.split('\n').collect();
                        for sed in splitedted {
                            if(sed != ""){
                                a.push(String::from(sed.trim()));
                            }
                        }
                        squarebraketslevel=0;
                        
                        a.push(String::from("]"));
                        newstring = String::new();
                        continue;
                    }

                
                    if la==']' {
                        squarebraketslevel-=1;
                    }
                    if la=='[' {
                        squarebraketslevel+=1;
                        newstring.push('\n');
                    }

                    if la=='=' {
                        newstring.push('\n');
                    }

                }

                if la=='"' {insidesquarebraketqoute=!insidesquarebraketqoute;}

                newstring.push(la);
                continue;
            }
            if la=='[' { //open
                if newstring != "" {
                    a.push(newstring);
                }
                newstring = String::new();

                a.push(String::from("["));
                squarebraketslevel+=1;

                lastinsect=la;
                instring = true;
                continue;
            } 

            //
            //  handeling qoutes and ()
            //

            if instring {

                if 
                    la=='"' && lastinsect=='"'
                { //close ""
                    instring = false;
                    if newstring != "" {
                        a.push(newstring);
                    }
                    newstring = String::new();
                    continue;
                }

                if 
                    la==')' && lastinsect=='('
                { //close ()
                    instring = false;
                    if newstring != "" {
                        a.push(newstring+")");
                    }
                    newstring = String::new();
                    continue;
                }

                newstring.push(la);
                continue;
            }
            if la=='"' { //open ""
                if newstring != "" {
                    a.push(newstring);
                }
                newstring = String::new();

                lastinsect=la;
                instring = true;
                continue;
            }
            if la=='(' { //open ()
                newstring.push(la);
                lastinsect=la;
                instring = true;
                continue;
            }

            if la==':' || la==' ' || la=='\n' {
                if newstring != "" {
                    a.push(newstring);
                }
                newstring = String::new();
                continue;
            }
            newstring.push(la);
        }

        if(a.is_empty()){continue};
        contentz.push(a);
    }

    dbg!(&contentz);

    
    for lines in contentz {
        dbg!(&lines);
        match lines.first().unwrap().as_str() {
            "include" => includes(path, lines)?,
            "SolidClass" => {
                
            }
            _=>()
        }
    }



    Ok(())
}

fn includes(path: &String, lines: Vec<String>) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let newpath = &lines[1];
    let path = Path::new(path);
    let thedir = path
    .parent()
    .ok_or("fucked up parent path in fgd")?
    .join(newpath);
    let location = thedir
    .to_str()
    .ok_or("fucked up conversion path | fgd")?;
    Ok(match open(&format!("{}", location)) {
        Ok(_) => (),
        Err(err) => {
            let err = format!("while runnining included fgd:\n'{}' \n\n{}", location, err);
            return Err(err.into());
        },
    })
}