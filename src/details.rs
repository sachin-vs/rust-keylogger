use std::process::Command;
#[derive(Debug)]
pub struct SystemDetails{
    pub os:String,
    pub username:String,
    pub event_path:String,
}

fn shell_command(cmd:&str)->String{
    let mut my_cmd = Command::new("sh");
    my_cmd.arg("-c")
              .arg(cmd);
    let my_cmd = my_cmd.output().expect("failed to execute process");

    let s = String::from_utf8(my_cmd.stdout).expect("Found invalid UTF-8");
    let mut output = String::new();
    for i in s.chars(){
        if i == '\n'{
            continue;
        }
        else{
            output.push(i);
        }
    }
    return output;
}


impl SystemDetails {
    pub fn details()->SystemDetails{
        SystemDetails{
            os:shell_command("uname"),
            username:shell_command("whoami"),
            event_path:format!("{}/{}","/dev/input",shell_command("grep -E  'Handlers|EV=' /proc/bus/input/devices | \
            grep -B1 'EV=120013' | \
            grep -Eo 'event[0-9]+'")),
        }

    }
}
