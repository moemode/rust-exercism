use std::fs::{self, read_to_string};
use std::path::Path;
use std::process::Command;
use std::error::Error;

fn get_exercism_workspace() -> Option<String> {
    let output = Command::new("exercism")
        .arg("workspace")
        .output()
        .ok()?;
    
    String::from_utf8(output.stdout)
        .ok()
        .map(|s| s.trim().to_string())
}

fn format_exercise_name(name: &str) -> String {
    name.to_lowercase().replace(' ', "-")
}

fn get_next_exercise() -> Option<String> {
    let workspace = get_exercism_workspace()?;
    let content = read_to_string(format!("{}/README.md", workspace)).ok()?;
    
    for line in content.lines() {
        if line.contains("🔄") {
            if let Some(exercise_name) = line
                .split('|')
                .nth(2)
                .and_then(|col| {
                    col.trim()
                        .split(']')
                        .next()
                        .map(|s| s.trim_start_matches('['))
                })
            {
                return Some(format_exercise_name(exercise_name));
            }
        }
    }
    None
}

fn update_vscode_settings(workspace: &str, exercise: &str) -> Result<(), Box<dyn Error>> {
    let settings_path = format!("{}/.vscode/settings.json", workspace);
    let content = read_to_string(&settings_path)?;
    
    // Parse the JSON content
    let mut lines: Vec<String> = content.lines().map(String::from).collect();
    
    // Find the last project entry
    let insert_pos = lines.iter()
        .position(|line| line.contains("rust-analyzer.linkedProjects"))
        .map(|pos| {
            lines.iter()
                .skip(pos)
                .position(|line| line.contains("]"))
                .map(|end| pos + end)
        })
        .flatten()
        .ok_or("Could not find insert position")?;

    // Insert new project before the closing bracket
    let new_project = format!("        \"rust/{}/Cargo.toml\",", exercise.to_lowercase());
    lines.insert(insert_pos, new_project);

    // Write back to file
    fs::write(settings_path, lines.join("\n"))?;
    Ok(())
}

fn main() {
    let workspace = match get_exercism_workspace() {
        Some(path) => path,
        None => {
            eprintln!("Could not determine exercism workspace path");
            return;
        }
    };

    match get_next_exercise() {
        Some(exercise) => {
            println!("Next exercise to work on: {}", exercise);
            
            // Check if exercise directory already exists
            let exercise_path = format!("{}/rust/{}", workspace, exercise);
            if Path::new(&exercise_path).exists() {
                println!("Exercise directory {} already exists, skipping download", exercise);
                return;
            }
            
            // Print and launch exercism download command
            let command = format!("exercism download --track=rust --exercise={}", exercise);
            println!("Running: {}", command);
            
            let status = Command::new("exercism")
                .args(["download", "--track=rust", &format!("--exercise={}", exercise)])
                .status();
                
            match status {
                Ok(exit_status) => {
                    if exit_status.success() {
                        println!("Successfully downloaded {}", exercise);
                        // Update VS Code settings after successful download
                        if let Err(e) = update_vscode_settings(&workspace, &exercise) {
                            eprintln!("Failed to update VS Code settings: {}", e);
                        }
                    } else {
                        eprintln!("Failed to download exercise");
                    }
                }
                Err(e) => eprintln!("Failed to execute exercism command: {}", e),
            }
        }
        None => println!("No exercises marked as undone found in README.md"),
    }
}
