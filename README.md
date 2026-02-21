# PDF2DXF Converter

PDF2DXF est une application de bureau performante (développée avec **Tauri v2**, **React** et **Rust**) permettant de convertir des plans d'architectes et autres fichiers vectoriels du format PDF vers le format CAO DXF (AutoCAD R12). L'application garantit une très haute fidélité d'extraction vectorielle pour le traitement géométrique direct.

## Fonctionnalités
- Processus complet d'extraction et de conversion des données vectorielles d'un PDF.
- Prise en charge de la transformation d'échelle (`1:X`).
- Interface moderne, épurée et ergonomique respectant les codes de design minimaux.
- Visualisation directe et ouverture transparente des fichiers DXF fraîchement générés avec votre logiciel installé par défaut.

## Architecture
- **Frontend** : [React](https://react.dev/), [Vite](https://vitejs.dev/), [Framer Motion](https://www.framer.com/motion/) et icônes SVG via [Lucide](https://lucide.dev/).
- **Backend** : [Rust](https://www.rust-lang.org/) fonctionnant sous [Tauri v2](https://v2.tauri.app/). Analyse PDF (`lopdf`), algorithmes matriciels, et génération structurée (`dxf`, `image`).

## Installation & Développement

### Prérequis
- `Node.js`
- `Rust` / `Cargo`
- Dépendances de votre système d'exploitation requises par Tauri.

### Commandes

```bash
# 1. Installer les paquets node
npm install

# 2. Lancer la version de test avec le rechargement à chaud (Vite + Fenêtre Rust)
npm run tauri dev

# 3. Générer le binaire pour votre système
npm run tauri build
```

## 👨‍💻 Auteur
Créé par [Hugo Burnet](https://www.linkedin.com/in/hugo-burnet-a11323309/)  
- 📜 **Portfolio** : [cv-online](https://hugo-burnet.github.io/cv-online/)  
- 📐 **Logiciel CalipiCAD** : [CalipiCAD](https://hugo-burnet.github.io/CalipiCAD/)  
- 🐈‍⬛ **Toutes mes sources** : [hugo-burnet sur Github](https://github.com/hugo-burnet)
