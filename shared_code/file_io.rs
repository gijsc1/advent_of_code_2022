
let file = fs::File::open("input.txt").expect("Error while reading file");
    let commands = BufReader::new(file).lines()
        .map(|s|s.expect("Failure to read file"))