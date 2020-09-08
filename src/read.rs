pub fn read_file_line_column(
    file: String,
    line: usize,
    position: usize,
) -> Result<f64, crate::error::MyError> {
    if let Ok(line) = read_file_line(file, line) {
        let num = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .get(position)
            .unwrap()
            .parse::<f64>()?;
        return Ok(num);
    };
    Ok(-0.0)
}

pub fn read_file_line(file: String, line: usize) -> Result<String, crate::error::MyError> {
    let content = std::fs::read_to_string(file)?;
    if let Some(line) = content
        .lines()
        .collect::<Vec<&str>>()
        .get(line)
        .and_then(|line| Some(line.to_string()))
    {
        return Ok(line);
    };
    // .into();

    Err(crate::error::MyError::OutOfRange)
}
