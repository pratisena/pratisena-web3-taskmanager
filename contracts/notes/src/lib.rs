#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data untuk inventory
#[contracttype]
#[derive(Clone, Debug)]
pub struct Item {
    id: u64,
    name: String,
    quantity: u32,
    price: u32,
}

// Storage key
const INVENTORY_DATA: Symbol = symbol_short!("INVENTORY");

#[contract]
pub struct InventoryContract;

#[contractimpl]
impl InventoryContract {

    // Ambil semua item
    pub fn get_items(env: Env) -> Vec<Item> {
        env.storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Tambah item baru
    pub fn add_item(env: Env, name: String, quantity: u32, price: u32) -> String {
        let mut items: Vec<Item> = env.storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env));

        let item = Item {
            id: env.prng().gen::<u64>(),
            name,
            quantity,
            price,
        };

        items.push_back(item);

        env.storage().instance().set(&INVENTORY_DATA, &items);

        String::from_str(&env, "Item berhasil ditambahkan")
    }

    // Hapus item berdasarkan ID
    pub fn delete_item(env: Env, id: u64) -> String {
        let mut items: Vec<Item> = env.storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            if items.get(i).unwrap().id == id {
                items.remove(i);

                env.storage().instance().set(&INVENTORY_DATA, &items);
                return String::from_str(&env, "Item berhasil dihapus");
            }
        }

        String::from_str(&env, "Item tidak ditemukan")
    }

    // Update quantity (misal stok bertambah/berkurang)
    pub fn update_stock(env: Env, id: u64, new_quantity: u32) -> String {
        let mut items: Vec<Item> = env.storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            let mut item = items.get(i).unwrap();

            if item.id == id {
                item.quantity = new_quantity;
                items.set(i, item);

                env.storage().instance().set(&INVENTORY_DATA, &items);
                return String::from_str(&env, "Stock berhasil diupdate");
            }
        }

        String::from_str(&env, "Item tidak ditemukan")
    }
}

mod test;