#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Vec};

// ==========================================
// 1. STRUKTUR PENYIMPANAN DINAMIS (KUNCI PRO)
// ==========================================
// Kita pakai Enum agar bisa menyimpan data komentar berdasarkan ID Post secara dinamis.
// Ini menunjukkan pemahaman struktur data tingkat lanjut!
#[contracttype]
pub enum DataKey {
    Posts,
    Comments(u64), // Kunci storage yang dinamis berdasarkan ID Post
}

// ==========================================
// 2. STRUKTUR DATA (ENTITAS)
// ==========================================
#[contracttype]
#[derive(Clone, Debug)]
pub struct Post {
    pub id: u64,
    pub author: String,
    pub title: String,
    pub category: String,
    pub content: String,
    pub upvotes: u32, // FITUR BARU: Counter dukungan/reputasi
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Comment {
    pub id: u64,
    pub author: String,
    pub content: String,
}

#[contract]
pub struct KMSContract;

#[contractimpl]
impl KMSContract {
    
    // ==========================================
    // FITUR A: MANAJEMEN POST (CRUD)
    // ==========================================
    pub fn get_posts(env: Env) -> Vec<Post> {
        return env.storage().instance().get(&DataKey::Posts).unwrap_or(Vec::new(&env));
    }

    pub fn create_post(env: Env, author: String, title: String, category: String, content: String) -> String {
        let mut posts: Vec<Post> = env.storage().instance().get(&DataKey::Posts).unwrap_or(Vec::new(&env));
        let new_post = Post {
            id: env.prng().gen::<u64>(),
            author,
            title,
            category,
            content,
            upvotes: 0, // Nilai default saat post dibuat
        };
        posts.push_back(new_post);
        env.storage().instance().set(&DataKey::Posts, &posts);
        return String::from_str(&env, "Berhasil! Pengetahuan baru telah dipublikasikan.");
    }

    pub fn update_post(env: Env, id: u64, new_title: String, new_content: String) -> String {
        let mut posts: Vec<Post> = env.storage().instance().get(&DataKey::Posts).unwrap_or(Vec::new(&env));
        for i in 0..posts.len() {
            let mut current_post = posts.get(i).unwrap();
            if current_post.id == id {
                current_post.title = new_title;
                current_post.content = new_content;
                posts.set(i, current_post);
                env.storage().instance().set(&DataKey::Posts, &posts);
                return String::from_str(&env, "Berhasil! Konten berhasil direvisi.");
            }
        }
        return String::from_str(&env, "Gagal! Post tidak ditemukan.");
    }

    pub fn delete_post(env: Env, id: u64) -> String {
        let mut posts: Vec<Post> = env.storage().instance().get(&DataKey::Posts).unwrap_or(Vec::new(&env));
        for i in 0..posts.len() {
            if posts.get(i).unwrap().id == id {
                posts.remove(i);
                env.storage().instance().set(&DataKey::Posts, &posts);
                return String::from_str(&env, "Berhasil! Post telah dihapus.");
            }
        }
        return String::from_str(&env, "Gagal! Post tidak ditemukan.");
    }

    // ==========================================
    // FITUR B: SISTEM UPVOTE (GAMIFIKASI)
    // ==========================================
    pub fn upvote_post(env: Env, id: u64) -> String {
        let mut posts: Vec<Post> = env.storage().instance().get(&DataKey::Posts).unwrap_or(Vec::new(&env));
        for i in 0..posts.len() {
            let mut current_post = posts.get(i).unwrap();
            if current_post.id == id {
                current_post.upvotes += 1; // Tambah 1 poin reputasi
                posts.set(i, current_post);
                env.storage().instance().set(&DataKey::Posts, &posts);
                return String::from_str(&env, "Terima kasih! Upvote berhasil diberikan.");
            }
        }
        return String::from_str(&env, "Gagal! Post tidak ditemukan.");
    }

    // ==========================================
    // FITUR C: SISTEM KOMENTAR (RELASIONAL)
    // ==========================================
    pub fn add_comment(env: Env, post_id: u64, author: String, content: String) -> String {
        // Kita menggunakan post_id untuk membuat "folder" penyimpanan khusus untuk komentar di post tersebut
        let key = DataKey::Comments(post_id);
        let mut comments: Vec<Comment> = env.storage().instance().get(&key).unwrap_or(Vec::new(&env));
        
        let new_comment = Comment {
            id: env.prng().gen::<u64>(),
            author,
            content,
        };
        
        comments.push_back(new_comment);
        env.storage().instance().set(&key, &comments); // Simpan ke storage dinamis
        
        return String::from_str(&env, "Diskusi ditambahkan! Komentar berhasil diunggah.");
    }

    pub fn get_comments(env: Env, post_id: u64) -> Vec<Comment> {
        let key = DataKey::Comments(post_id);
        // Mengembalikan daftar komentar hanya untuk post_id yang diminta
        return env.storage().instance().get(&key).unwrap_or(Vec::new(&env));
    }
}