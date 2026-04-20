#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data untuk task
#[contracttype]
#[derive(Clone, Debug)]
pub struct Task {
    id: u64,
    title: String,
    description: String,
    completed: bool,
}

// Storage key
const TASK_DATA: Symbol = symbol_short!("TASK");

#[contract]
pub struct TaskContract;

#[contractimpl]
impl TaskContract {

    // Ambil semua task
    pub fn get_tasks(env: Env) -> Vec<Task> {
        env.storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Tambah task baru
    pub fn create_task(env: Env, title: String, description: String) -> String {
        let mut tasks: Vec<Task> = env.storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env));

        let task = Task {
            id: env.prng().gen::<u64>(),
            title,
            description,
            completed: false, // default belum selesai
        };

        tasks.push_back(task);

        env.storage().instance().set(&TASK_DATA, &tasks);

        String::from_str(&env, "Task berhasil dibuat")
    }

    // Hapus task
    pub fn delete_task(env: Env, id: u64) -> String {
        let mut tasks: Vec<Task> = env.storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            if tasks.get(i).unwrap().id == id {
                tasks.remove(i);

                env.storage().instance().set(&TASK_DATA, &tasks);
                return String::from_str(&env, "Task berhasil dihapus");
            }
        }

        String::from_str(&env, "Task tidak ditemukan")
    }

    // Update status task (done / not done)
    pub fn update_status(env: Env, id: u64, status: bool) -> String {
        let mut tasks: Vec<Task> = env.storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            let mut task = tasks.get(i).unwrap();

            if task.id == id {
                task.completed = status;
                tasks.set(i, task);

                env.storage().instance().set(&TASK_DATA, &tasks);
                return String::from_str(&env, "Status task berhasil diupdate");
            }
        }

        String::from_str(&env, "Task tidak ditemukan")
    }
}

mod test;