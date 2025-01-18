use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;

fn get_exercism_workspace() -> Option<String> {
    let output = Command::new("exercism")
        .arg("workspace")
        .output()
        .ok()?;
    
    String::from_utf8(output.stdout)
        .ok()
        .map(|s| s.trim().to_string())
}

fn get_next_exercise() -> Option<String> {
    let workspace = get_exercism_workspace()?;
    let content = read_to_string(format!("{}/README.md", workspace)).ok()?;
    
    // Find lines containing "🔄" and extract exercise name
    for line in content.lines() {
        if line.contains("🔄") {
            // Extract exercise name from markdown table format
            if let Some(exercise_name) = line
                .split('|')
                .nth(2)  // Third column contains [Exercise Name](./rust/exercise-name)
                .and_then(|col| {
                    col.trim()
                        .split(']')
                        .next()
                        .map(|s| s.trim_start_matches('['))
                })
            {
                return Some(exercise_name.to_string());
            }
        }
    }
    None
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
            let exercise_path = format!("{}/rust/{}", workspace, exercise.to_lowercase());
            if Path::new(&exercise_path).exists() {
                println!("Exercise directory {} already exists, skipping download", exercise);
                return;
            }
            
            // Launch exercism download command
            let status = Command::new("exercism")
                .args(["download", "--track=rust", &format!("--exercise={}", exercise)])
                .status();
                
            match status {
                Ok(exit_status) => {
                    if exit_status.success() {
                        println!("Successfully downloaded {}", exercise);
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
