// Importations nécessaires du module standard de Rust
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

// Ajout d'une tâche au fichier "tasks.txt"
fn add_task(description: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open("tasks.txt")?;
    writeln!(file, "{}", description)?;
    Ok(())
}

// Liste toutes les tâches du fichier "tasks.txt"
fn list_tasks() -> io::Result<()> {
    let file = File::open("tasks.txt");
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for (index, line) in reader.lines().enumerate() {
                println!("{}: {}", index + 1, line?);
            }
            Ok(())
        }
        Err(_) => {
            eprintln!("No tasks found.");
            Ok(())
        }
    }
}

// Met à jour une tâche à l'index spécifié dans le fichier "tasks.txt"
fn update_task(index: usize, description: &str) -> io::Result<()> {
    let mut tasks = load_tasks()?;
    if let Some(task) = tasks.get_mut(index) {
        *task = description.to_string();
        save_tasks(&tasks)?;
    } else {
        eprintln!("Index out of bounds");
    }
    Ok(())
}

// Supprime une tâche à l'index spécifié dans le fichier "tasks.txt"
fn delete_task(index: usize) -> io::Result<()> {
    let mut tasks = load_tasks()?;
    if index < tasks.len() {
        tasks.remove(index);
        save_tasks(&tasks)?;
    } else {
        eprintln!("Index out of bounds");
    }
    Ok(())
}

// Sauvegarde les tâches dans le fichier "tasks.txt"
fn save_tasks(tasks: &[String]) -> io::Result<()> {
    let mut file = File::create("tasks.txt")?;
    for task in tasks {
        writeln!(file, "{}", task)?;
    }
    Ok(())
}

// Charge les tâches à partir du fichier "tasks.txt"
fn load_tasks() -> io::Result<Vec<String>> {
    let file = File::open("tasks.txt");
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            let tasks: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
            Ok(tasks)
        }
        Err(_) => Ok(vec![]),
    }
}

// Fonction principale (main) pour un exemple d'utilisation
fn main() {
    // Ajout de tâches
    add_task("Faire les courses").unwrap();
    add_task("Apprendre Rust").unwrap();

    // Liste des tâches
    list_tasks().unwrap();

    // Mise à jour d'une tâche
    update_task(1, "Lire un livre Rust").unwrap();
    list_tasks().unwrap();

    // Suppression d'une tâche
    delete_task(0).unwrap();
    list_tasks().unwrap();
}
