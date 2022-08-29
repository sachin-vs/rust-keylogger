use std::process::Command;
#[derive(Debug)]
//Struct showing details of the system
//OS - Linux/Windows
//Username - whoami
//Event path - /dev/input/event* path of the keyboard event file
pub struct SystemDetails{
    pub os:String,
    pub username:String,
    pub event_path:String,
}
//Function to get system details
//All system details are taen using shell scripts
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

//Parameters of sell commands are given here
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
