fn flush(process: &mut String, processes: &mut Vec<String>) {
    let command = process.trim().to_string();
    if !command.is_empty() {
        processes.push(command);
    }
    process.clear();
}

pub fn parse_processes(content: String) -> Vec<String> {
    let mut processes: Vec<String> = vec![];
    let mut group = String::new();
    let mut process = String::new();
    let chars: Vec<char> = content.chars().collect();
    let mut pos = 0;

    while pos < chars.len() {
        if chars[pos] == '{' {
            let mut name = String::new();
            pos += 1;
            while pos < chars.len() && chars[pos] != '}' {
                name.push(chars[pos]);
                pos += 1;
            }

            if pos < chars.len() {
                pos += 1;
            }

            let name = name.trim();
            if name.is_empty() {
                flush(&mut process, &mut processes);
                group.clear();
            } else {
                if group.is_empty() && name != "start" {
                    panic!("jnit: 'start' process group not founded, it should be first in file");
                }
                group = name.to_string();
            }
            continue;
        }

        if !group.is_empty() {
            match chars[pos] {
                ';' => flush(&mut process, &mut processes),
                '\r' => {}
                _ => process.push(chars[pos]),
            }
        }
        pos += 1;
    }

    flush(&mut process, &mut processes);
    processes
}
