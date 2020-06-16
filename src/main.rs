mod consts;

fn encode(input: &[u8]) -> String {
    let should_pad = input.len() % 2 != 0;
    let length = input.len() + should_pad as usize;
    let dashes = if input.len() > 2 { length / 2 - 1 } else { 0 };
    let capacity = length * 3 + dashes;
    let mut output = String::with_capacity(capacity);
    if should_pad {
        output.push_str(consts::PREFIXES[0]);
    }
    let mut dashes_placed = 0;
    for pair in input.rchunks(2).rev() {
        match pair.len() {
            1 => {
                output.push_str(consts::SUFFIXES[pair[0] as usize]);
            }
            2 => {
                output.push_str(consts::PREFIXES[pair[0] as usize]);
                output.push_str(consts::SUFFIXES[pair[1] as usize]);
            }
            _ => unreachable!(),
        }
        if dashes_placed != dashes {
            output.push('-');
            dashes_placed += 1;
        }
    }
    output
}

fn decode(input: &str) -> Option<Vec<u8>> {
    let stripped_input = input.replace(&['-', ' '][..], "");
    if stripped_input.len() % 3 != 0 {
        return None;
    }
    let capacity = stripped_input.len() / 3;
    let mut output: Vec<u8> = Vec::with_capacity(capacity);
    for i in (0..stripped_input.len()).step_by(6) {
        output.push(*consts::PREFIXES_MAP.get(&stripped_input[i..i + 3])?);
        output.push(*consts::SUFFIXES_MAP.get(&stripped_input[i + 3..i + 6])?);
    }
    Some(output)
}

fn main() {
    let bytes: [u8; 6] = [1, 2, 3, 4, 5, 6];
    println!("{:?}", decode(&encode(&bytes)));
}
