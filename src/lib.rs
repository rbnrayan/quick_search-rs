use curl::easy::Easy;

pub fn make_request(url: &str) -> Result<Vec<u8>, curl::Error> {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(url)?;
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;
        transfer.perform()?;
    }

    handle.perform()?;

    Ok(data)
}

pub fn format_input_to_url(mut input: String) -> String {
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }

    let input = input.chars().map(|c| {
        if c.is_whitespace() {
            "%20".to_string()
        } else { c.to_string() }
    }).collect::<String>();

    format!("https://api.duckduckgo.com/?q={}&format=json&pretty=1", input)
}
