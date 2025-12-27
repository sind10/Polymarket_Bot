# 🤖 Polymarket-Kalshi Arbitrage Bot

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.75+-orange?logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/License-MIT%2FApache--2.0-blue" alt="License">
  <img src="https://img.shields.io/badge/Status-Production%20Ready-green" alt="Status">
</p>

> **Bot d'arbitrage haute performance** pour les marchés de prédiction Kalshi et Polymarket.  
> Détection et exécution automatique d'opportunités d'arbitrage sans risque en temps réel.

---

## 📋 Table des matières

- [✨ Fonctionnalités](#-fonctionnalités)
- [🎯 Comment ça marche](#-comment-ça-marche)
- [🚀 Démarrage rapide](#-démarrage-rapide)
- [⚙️ Configuration](#️-configuration)
- [📖 Utilisation](#-utilisation)
- [🏗️ Architecture](#️-architecture)
- [📊 Statut du projet](#-statut-du-projet)
- [🤝 Contribution](#-contribution)
- [📄 Licence](#-licence)

---

## ✨ Fonctionnalités

| Type d'arbitrage | Description |
|------------------|-------------|
| **Kalshi ↔ Polymarket** | Arbitrage cross-plateforme |
| **Polymarket ↔ Polymarket** | Arbitrage intra-plateforme |
| **Kalshi ↔ Kalshi** | Arbitrage intra-plateforme |

### Points forts

- 🔒 **Cache orderbook lock-free** - Opérations atomiques sans copie
- ⚡ **Détection SIMD** - Latence sub-milliseconde
- 🔄 **Exécution concurrente** - Ordres simultanés sur les deux plateformes
- 🛡️ **Circuit breaker** - Gestion des risques configurable
- 🗺️ **Découverte intelligente** - Mapping automatique des marchés

---

## 🎯 Comment ça marche

### Principe de l'arbitrage

Dans les marchés de prédiction : **YES + NO = 1.00$** (garanti).

Le bot détecte quand :
```
Prix YES (Plateforme A) + Prix NO (Plateforme B) < 1.00$
```

### Exemple concret

| Élément | Valeur |
|---------|--------|
| YES sur Kalshi | 0.42$ |
| NO sur Polymarket | 0.56$ |
| **Coût total** | **0.98$** |
| **Gain garanti** | **0.02$ (2.04%)** |

### Types d'opportunités

| Type | Stratégie | Fréquence |
|------|-----------|-----------|
| `poly_yes_kalshi_no` | Acheter YES Polymarket + NO Kalshi | Commun |
| `kalshi_yes_poly_no` | Acheter YES Kalshi + NO Polymarket | Commun |
| `poly_only` | Acheter YES + NO sur Polymarket | Rare |
| `kalshi_only` | Acheter YES + NO sur Kalshi | Rare |

---

## 🚀 Démarrage rapide

### 1. Prérequis

```bash
# Installer Rust 1.75+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Installation

```bash
# Cloner le projet
git clone https://github.com/sind10/Polymarket_Bot.git
cd Polymarket_Bot

# Compiler
cargo build --release
```

### 3. Configuration

Créer un fichier `.env` :

```env
# ═══════════════════════════════════════
# KALSHI
# ═══════════════════════════════════════
KALSHI_API_KEY_ID=your_api_key_id
KALSHI_PRIVATE_KEY_PATH=/path/to/private_key.pem

# ═══════════════════════════════════════
# POLYMARKET
# ═══════════════════════════════════════
POLY_PRIVATE_KEY=0xYOUR_PRIVATE_KEY
POLY_FUNDER=0xYOUR_WALLET_ADDRESS

# ═══════════════════════════════════════
# SYSTÈME
# ═══════════════════════════════════════
DRY_RUN=1
RUST_LOG=info
```

### 4. Lancement

```bash
# Mode test (paper trading)
dotenvx run -- cargo run --release

# Mode production
DRY_RUN=0 dotenvx run -- cargo run --release
```

---

## ⚙️ Configuration

### Variables requises

| Variable | Description |
|----------|-------------|
| `KALSHI_API_KEY_ID` | ID de clé API Kalshi |
| `KALSHI_PRIVATE_KEY_PATH` | Chemin vers la clé privée RSA (PEM) |
| `POLY_PRIVATE_KEY` | Clé privée Ethereum (préfixe 0x) |
| `POLY_FUNDER` | Adresse wallet Polymarket (préfixe 0x) |

### Variables système

| Variable | Défaut | Description |
|----------|--------|-------------|
| `DRY_RUN` | `1` | `1` = simulation, `0` = trading réel |
| `RUST_LOG` | `info` | Niveau de log (`error`, `warn`, `info`, `debug`, `trace`) |
| `FORCE_DISCOVERY` | `0` | `1` = reconstruire le cache des marchés |
| `PRICE_LOGGING` | `0` | `1` = logs détaillés des prix |

### Circuit Breaker

| Variable | Défaut | Description |
|----------|--------|-------------|
| `CB_ENABLED` | `true` | Activer/désactiver |
| `CB_MAX_POSITION_PER_MARKET` | `100` | Max contrats par marché |
| `CB_MAX_TOTAL_POSITION` | `500` | Max contrats total |
| `CB_MAX_DAILY_LOSS` | `5000` | Perte max journalière (centimes) |
| `CB_MAX_CONSECUTIVE_ERRORS` | `5` | Erreurs consécutives avant arrêt |
| `CB_COOLDOWN_SECS` | `60` | Délai de récupération |

### Mode test

| Variable | Défaut | Description |
|----------|--------|-------------|
| `TEST_ARB` | `0` | `1` = injecter une opportunité synthétique |
| `TEST_ARB_TYPE` | `poly_yes_kalshi_no` | Type d'arbitrage à simuler |

---

## 📖 Utilisation

### Exemples de commandes

```bash
# 🧪 Paper trading avec logs détaillés
RUST_LOG=debug DRY_RUN=1 dotenvx run -- cargo run --release

# 🔬 Tester l'exécution avec opportunité synthétique
TEST_ARB=1 DRY_RUN=0 dotenvx run -- cargo run --release

# 🚀 Production avec circuit breaker personnalisé
DRY_RUN=0 CB_MAX_DAILY_LOSS=10000 dotenvx run -- cargo run --release

# 🔄 Forcer la redécouverte des marchés
FORCE_DISCOVERY=1 dotenvx run -- cargo run --release
```

---

## 🔑 Obtenir les credentials

### Kalshi

1. Connectez-vous sur [Kalshi](https://kalshi.com)
2. Allez dans **Settings → API Keys**
3. Créez une clé avec permissions de trading
4. Téléchargez la clé privée (fichier PEM)
5. Notez l'API Key ID

### Polymarket

1. Créez/importez un wallet Ethereum (MetaMask, etc.)
2. Exportez la clé privée (avec préfixe `0x`)
3. Approvisionnez en USDC sur le réseau Polygon
4. L'adresse du wallet = `POLY_FUNDER`

---

## 🏗️ Architecture

```
src/
├── main.rs              # Point d'entrée et orchestration WebSocket
├── types.rs             # Types et gestion d'état des marchés
├── execution.rs         # Moteur d'exécution concurrent
├── position_tracker.rs  # Suivi des positions et P&L
├── circuit_breaker.rs   # Gestion des risques
├── discovery.rs         # Découverte et matching des marchés
├── cache.rs             # Cache de mapping des équipes
├── kalshi.rs            # Client API Kalshi (REST + WebSocket)
├── polymarket.rs        # Client WebSocket Polymarket
├── polymarket_clob.rs   # Client CLOB Polymarket
└── config.rs            # Configuration des ligues et seuils
```

### Frais

| Plateforme | Frais |
|------------|-------|
| **Kalshi** | `ceil(0.07 × contrats × prix × (1-prix))` |
| **Polymarket** | Gratuit |

---

## 📊 Statut du projet

### ✅ Fonctionnalités complètes

- [x] Client API Kalshi (REST + WebSocket)
- [x] Client API Polymarket (REST + WebSocket)
- [x] Cache orderbook atomique lock-free
- [x] Détection d'arbitrage accélérée SIMD
- [x] Exécution concurrente multi-jambes
- [x] Suivi des positions et P&L en temps réel
- [x] Circuit breaker configurable
- [x] Découverte intelligente avec cache
- [x] Gestion automatique de l'exposition

### 🚧 Améliorations futures

- [ ] Interface web de configuration des risques
- [ ] Support multi-comptes
- [ ] Stratégies de routage avancées
- [ ] Dashboard d'analytics historiques

---

## 🛠️ Développement

```bash
# Lancer les tests
cargo test

# Build avec profiling
cargo build --release --features profiling

# Benchmarks
cargo bench
```

---

## 🤝 Contribution

Les contributions sont les bienvenues ! N'hésitez pas à ouvrir une issue ou une pull request.

---

## 📄 Licence

Ce projet est sous double licence :

- **Apache License 2.0** - [LICENSE-APACHE](LICENSE-APACHE)
- **MIT License** - [LICENSE-MIT](LICENSE-MIT)

---

<p align="center">
  <strong>⭐ Star ce repo si vous le trouvez utile !</strong>
</p>
