# Tutorial 2 - Broadcast Chat

## Experiment 2.1: Original code, and how it run

Project ini berisi aplikasi broadcast chat asynchronous berbasis WebSocket dari Comprehensive Rust.
Server menerima koneksi client pada `127.0.0.1:2000`.
Setiap client membaca input dari terminal, mengirim pesan tersebut ke server, lalu server melakukan broadcast pesan ke semua client yang sedang terhubung.

Cara menjalankan server:

```powershell
cargo run --bin server
```

Cara menjalankan client:

```powershell
cargo run --bin client
```

Untuk eksperimen, jalankan satu terminal server dan tiga terminal client.
Setelah semua client terhubung, ketik pesan pada salah satu client.
Pesan tersebut akan dikirim ke server melalui WebSocket, lalu server membroadcast pesan itu ke semua client.

### Server

![Server running broadcast chat](images/server.png)

Server berhasil berjalan pada port `2000`.
Pada screenshot, server menerima tiga koneksi client dari alamat lokal `127.0.0.1` dengan port yang berbeda-beda.
Server juga menerima tiga pesan, yaitu `hello from client 1`, `hello from client 2`, dan `hello from client 3`.

### Client 1

![Client 1 receiving broadcast messages](images/client1.png)

### Client 2

![Client 2 receiving broadcast messages](images/client2.png)

### Client 3

![Client 3 receiving broadcast messages](images/client3.png)

Setiap client menerima pesan dari client lain melalui broadcast server.
Pada terminal client, pesan yang diketik oleh client dapat terlihat dua kali: pertama sebagai input yang diketik di terminal, dan kedua sebagai pesan broadcast yang dikirim kembali oleh server.
Hal tersebut menunjukkan bahwa server juga mengirimkan pesan kembali ke pengirim, bukan hanya ke client lain.

Aplikasi ini menggunakan `tokio::select!` agar client dapat membaca input pengguna dan menerima pesan dari server secara concurrent.
Pada sisi server, `tokio::select!` juga digunakan untuk menangani dua pekerjaan sekaligus, yaitu menerima pesan dari satu client dan mengirim pesan broadcast ke client tersebut.
Ketika sebuah client mengirim pesan, server menerima pesan tersebut melalui WebSocket dan meneruskannya ke broadcast channel.
Setiap koneksi client memiliki receiver broadcast sendiri, sehingga semua client yang subscribe ke channel tersebut dapat menerima pesan yang sama.
Karena itu, ketika satu client mengetik pesan, pesan tersebut muncul pada semua client yang sedang terhubung.

## Experiment 2.2: Modifying port

Pada eksperimen ini, port WebSocket diubah dari `2000` menjadi `8080`.
Perubahan harus dilakukan pada dua sisi koneksi, yaitu server dan client.
Pada sisi server, alamat bind diubah menjadi:

```rust
TcpListener::bind("127.0.0.1:8080")
```

Pada sisi client, URI WebSocket diubah menjadi:

```rust
Uri::from_static("ws://127.0.0.1:8080")
```

Server menggunakan `TcpListener` untuk membuka koneksi TCP pada alamat `127.0.0.1:8080`.
Client menggunakan protokol WebSocket melalui URI `ws://127.0.0.1:8080`.
Prefix `ws://` menunjukkan bahwa koneksi yang digunakan adalah WebSocket, bukan HTTP biasa.
Port harus sama di kedua sisi karena client perlu melakukan koneksi ke alamat dan port yang sedang didengarkan oleh server.
Jika server sudah berjalan pada port `8080`, tetapi client masih mencoba terhubung ke port `2000`, koneksi akan gagal karena tidak ada server yang menerima koneksi pada port tersebut.

Setelah perubahan ini, server dapat dijalankan dengan perintah yang sama:

```powershell
cargo run --bin server
```

Server akan menampilkan:

```text
listening on port 8080
```

Client juga tetap dijalankan dengan perintah yang sama:

```powershell
cargo run --bin client
```

Setelah diuji kembali, aplikasi tetap berjalan dengan benar.
Client dapat terhubung ke server pada port `8080`, mengirim pesan, dan menerima broadcast dari client lain seperti pada eksperimen sebelumnya.

Hasil uji server setelah port diubah:

```text
listening on port 8080
New connection from 127.0.0.1:65106
New connection from 127.0.0.1:65108
New connection from 127.0.0.1:65109
From 127.0.0.1:65106: port 8080 from client 1
From 127.0.0.1:65108: port 8080 from client 2
From 127.0.0.1:65109: port 8080 from client 3
```

Hasil uji pada setiap client:

```text
port 8080 from client 1
port 8080 from client 2
port 8080 from client 3
```
