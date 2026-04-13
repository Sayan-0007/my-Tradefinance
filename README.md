# my-Tradefinance
# 🚢 Trade Finance Smart Contract (Soroban | Stellar)

## 📌 Project Description

This project is a decentralized **Trade Finance Smart Contract** built using Soroban on the Stellar blockchain. It simulates a secure escrow-based workflow between a buyer and a seller, ensuring that funds are only released when predefined trade conditions are met.

![WhatsApp Image 2026-04-13 at 2 23 14 PM](https://github.com/user-attachments/assets/eb567609-91f4-4f90-a430-590f130ffaf8)



The goal is to reduce reliance on intermediaries like banks while increasing transparency, security, and efficiency in trade transactions.

---

## ⚙️ What It Does

The smart contract manages a trade lifecycle through clearly defined steps:

1. **Trade Initialization**
   Buyer sets up a trade agreement with seller and defines payment amount

2. **Deposit (Escrow Funding)**
   Buyer locks funds into the contract

3. **Shipment Confirmation**
   Seller confirms that goods have been shipped

4. **Delivery Confirmation**
   Buyer confirms receipt of goods

5. **Completion**
   Trade is marked complete and funds are released

---

## ✨ Features

* 🔐 **Authentication Required**
  Buyer and seller must authorize actions using their wallet

* 🔄 **State-Based Workflow**
  Trade progresses through:

  * INIT
  * FUNDED
  * SHIPPED
  * COMPLETED

* 💰 **Escrow Mechanism (Extendable)**
  Designed for integration with Soroban token contracts

* ⚡ **Fast & Low-Cost Transactions**
  Powered by Stellar network

* 🔍 **Transparent State Tracking**
  Trade details can be queried at any time

* 🧱 **Modular & Extensible Design**
  Easily extendable with advanced features

---

## 🔗 Deployed Smart Contract

**Contract Address:**

```
CCOT7XUEGV2EASJ72DMWVKZPU4Q3GETM362B2QORV6UZ73QMJN25D4KS
```

---

## 🛠 Tech Stack

* Soroban Smart Contracts
* Rust
* Stellar Blockchain

---

## 🚀 How to Build & Deploy

### Build Contract

```bash
stellar contract build
```

### Deploy Contract

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/contract.wasm \
  --network testnet
```

---

## 🧪 Example Workflow

* Buyer initializes trade
* Buyer deposits funds
* Seller confirms shipment
* Buyer confirms delivery
* Trade completes

---

## 🔮 Future Improvements

* 💸 Real token escrow integration
* ⏱ Timeout & automatic refund system
* ⚖️ Dispute resolution mechanism
* 📦 Oracle-based shipment verification
* 📊 Multi-trade support

---

## 📄 License

MIT License

---

## 🤝 Contributing

Contributions are welcome! Feel free to fork the repository and submit pull requests to improve functionality or add features.
