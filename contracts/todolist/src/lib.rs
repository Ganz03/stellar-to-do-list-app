#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data yang akan menyimpan todo items
#[contracttype]
#[derive(Clone, Debug)]
pub struct TodoItem {
    id: u64,
    title: String,
    completed: bool,
}

// Storage key untuk data todos
const TODO_DATA: Symbol = symbol_short!("TODO_DATA");

#[contract]
pub struct NotesContract;

#[contractimpl]
impl NotesContract {
    // Fungsi untuk mendapatkan semua todos
    pub fn get_todos(env: Env) -> Vec<TodoItem> {
    // 1. ambil data todos dari storage
    return env.storage().instance().get(&TODO_DATA).unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk membuat todo baru
    pub fn create_todo(env: Env, title: String) -> String {
    // 1. ambil data todos dari storage
        let mut todos: Vec<TodoItem> = env.storage().instance().get(&TODO_DATA).unwrap_or(Vec::new(&env));
    
    // 2. Buat object todo baru
        let todo = TodoItem {
            id: env.prng().gen::<u64>(),
            title: title,
            completed: false,
        };
    
    // 3. tambahkan todo baru ke todos lama
        todos.push_back(todo);
        
    // 4. simpan todos ke storage
        env.storage().instance().set(&TODO_DATA, &todos);
    
    return String::from_str(&env, "Todo berhasil ditambahkan");
    }

    // Fungsi untuk menghapus todo berdasarkan id
    pub fn delete_todo(env: Env, id: u64) -> String {
    // 1. ambil data todos dari storage 
        let mut todos: Vec<TodoItem> = env.storage().instance().get(&TODO_DATA).unwrap_or(Vec::new(&env));

    // 2. cari index todo yang akan dihapus menggunakan perulangan
        for i in 0..todos.len() {
            if todos.get(i).unwrap().id == id {
                todos.remove(i);

                env.storage().instance().set(&TODO_DATA, &todos);
                return String::from_str(&env, "Berhasil hapus todo");
            }
        }

        return String::from_str(&env, "To-do-list tidak ditemukan")
    }

    // Fungsi untuk menandai todo sebagai selesai
    pub fn toggle_todo(env: Env, id: u64) -> String {
    // 1. ambil data todos dari storage
        let mut todos: Vec<TodoItem> = env.storage().instance().get(&TODO_DATA).unwrap_or(Vec::new(&env));

    // 2. cari todo dan ubah status completed
        for i in 0..todos.len() {
            if todos.get(i).unwrap().id == id {
                let mut todo = todos.get(i).unwrap();
                todo.completed = !todo.completed;
                todos.set(i, todo);

                env.storage().instance().set(&TODO_DATA, &todos);
                return String::from_str(&env, "To-do-list berhasil diupdate");
            }
        }

        return String::from_str(&env, "To-do-list tidak ditemukan")
    }
}

mod test;













/* --- CONTOH SCRIPT ---

pub fn get_notes(env: Env) -> Vec<Note> {
    // 1. ambil data notes dari storage
    return env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
}

// Fungsi untuk membuat note baru
pub fn create_note(env: Env, title: String, content: String) -> String {
    // 1. ambil data notes dari storage
    let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
    
    // 2. Buat object note baru
    let note = Note {
        id: env.prng().gen::<u64>(),
        title: title,
        content: content,
    };
    
    // 3. tambahkan note baru ke notes lama
    notes.push_back(note);
    
    // 4. simpan notes ke storage
    env.storage().instance().set(&NOTE_DATA, &notes);
    
    return String::from_str(&env, "Notes berhasil ditambahkan");
}

// Fungsi untuk menghapus notes berdasarkan id
pub fn delete_note(env: Env, id: u64) -> String {
    // 1. ambil data notes dari storage 
    let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));

    // 2. cari index note yang akan dihapus menggunakan perulangan
    for i in 0..notes.len() {
        if notes.get(i).unwrap().id == id {
            notes.remove(i);

            env.storage().instance().set(&NOTE_DATA, &notes);
            return String::from_str(&env, "Berhasil hapus notes");
        }
    }

    return String::from_str(&env, "Notes tidak ditemukan")
}


*/