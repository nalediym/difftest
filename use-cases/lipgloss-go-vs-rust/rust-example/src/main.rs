use lipgloss::{join_horizontal, rounded_border, Color, Style, CENTER, RIGHT, TOP};

fn main() {
    let name = std::env::args()
        .nth(1)
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "World".to_string());

    // Header style: bold, colored, with rounded border
    let header_style = Style::new()
        .bold(true)
        .foreground(Color("205".to_string()))
        .background(Color("235".to_string()))
        .padding(0, 2, 0, 2)
        .border(rounded_border())
        .border_foreground(Color("63".to_string()))
        .width(40)
        .align_horizontal(CENTER);

    // Row label style
    let label_style = Style::new()
        .bold(true)
        .foreground(Color("86".to_string()))
        .width(12)
        .align_horizontal(RIGHT);

    // Row value style
    let value_style = Style::new()
        .foreground(Color("252".to_string()));

    // Print header
    let greeting = format!("Hello, {}!", name);
    println!("{}", header_style.render(&greeting));
    println!();

    // Print info table rows
    let rows = [
        ("Name", name.as_str()),
        ("Language", "Rust"),
        ("Framework", "lipgloss-rs"),
    ];

    for (label, value) in &rows {
        let label_text = format!("{}:", label);
        let rendered_label = label_style.render(&label_text);
        let rendered_value = value_style.render(value);
        println!(
            "{}",
            join_horizontal(TOP, &[rendered_label.as_str(), " ", rendered_value.as_str()])
        );
    }

    // Footer separator
    println!("{}", "─".repeat(40));
}
