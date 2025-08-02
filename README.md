# Huffit

A fast and lightweight command-line tool for compressing and decompressing text files using the Huffman encoding algorithm.

⚠️ **Project Status**: In Development  
This project is currently a work in progress. Features are being actively implemented and tested.

## ✨ Goals

- Implement a full Huffman encoder and decoder from scratch.
- Explore mechanical sympathy and CPU-level optimizations.
- Learn and apply efficient data structures (binary trees, bit-level operations).
- Compare compression ratios with standard tools (e.g. `gzip`).

## 🚀 Planned Features

- [x] Read text files and compute character frequencies.
- [x] Build and serialize the Huffman tree.
- [x] Encode text using a prefix-free Huffman code table.
- [x] Write compressed data along with necessary metadata.
- [x] Decode compressed files back into original text.
- [x] Support CLI flags: `--compress`, `--decompress`.
- [ ] Fix bugs

## 📦 Usage (Coming Soon)

Once implemented, usage will look like:

```bash
# Compress a file
huff compress test.txt

# Decompress it
huff decompress compressed.huff

🛠️ Development
Clone the repo and build:
git clone https://github.com/TG100/Huffit.git
cd Huffit
cargo build

Run tests:
cargo test

📚 License
This project is licensed under the MIT License.

