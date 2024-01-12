pub fn parse_markdown(markdown_input: &str) -> String {
    let mut html_output = String::new();
    let mut in_list = false;
    for line in markdown_input.lines() {
        if line.starts_with("# ") {
            html_output.push_str(&format!("<h1>{}</h1>", &line[2..]));
        } 
        else if line.starts_with("## ") {
            html_output.push_str(&format!("<h2>{}</h2>", &line[2..]));
        }
        else if line.starts_with("* ") {
            if !in_list {
                html_output.push_str("<ul>");
                in_list = true;
            }
            html_output.push_str(&format!("<li>{}</li>", &line[2..]));
        } else {
            if in_list {
                html_output.push_str("</ul>");
                in_list = false;
            }
            html_output.push_str(&format!("<p>{}</p>", line));
        }
    }

    if in_list {
        html_output.push_str("</ul>");
    }

    return html_output;
}
