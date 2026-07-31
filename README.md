# Password-Strength-Checker 🔑

A cybersecurity project to learn cryptographic hashing and working with REST APIs.

> A password strength analyzer written in Rust. Analyzes password characteristics, assigns a score, and checks for breaches.

## 🚚 Features
- Password strength scoring
- Uppercase and lowercase letter detection
- Digit and special character detection
- Pasword strength classification
  - Weak
  - Fair
  - Strong
  - Very Strong
- [Have I Been Pwned](https://haveibeenpwned.com/) password breach check 
- Colored terminal ouput

## 👟 Running
Clone the repository
```powershell
# Clone the repository
git clone [https://github.com/TereshDK/Password-Strength-Checker.git](https://github.com/TereshDK/Password-Strength-Checker.git)
cd ferro
```
Run
```powershell
cargo run
```
Build a release version
```powershell
# Build in release mode
cargo build --release
```

## ❓ How HIBP Works
1. Password is hashed locally using SHA-1
2. First five characters of hash sent to HIBP
3. The API returns all matching hash suffixes
4. The remaining hash is compared locally
5. If match is found, total known breaches are reported

## 📄 License
This project is licensed under a personal license. Click to [View License](LICENSE)
