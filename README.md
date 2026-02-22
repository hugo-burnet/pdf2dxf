# 📐 PDF2DXF (v1.0.0) - Convertisseur Vectoriel Haute Fidélité

**PDF2DXF** est votre nouvel outil de bureau conçu pour transformer sans perte les plans d'architectes (ou n'importe quel dessin vectoriel) du format PDF vers le format CAO universel DXF.

---

## 🛠️ Comment utiliser le logiciel ?

### 1. Importation d'un plan
Ouvrez l'application et glissez-déposez simplement votre fichier PDF dans la grande zone centrale (vous pouvez également cliquer sur cette zone pour parcourir vos fichiers classiques).

### 2. Réglage de l'échelle (⚠️ Très important)
Par défaut, le logiciel extrait les lignes à la taille du papier du PDF (échelle 1/1). 
**Si votre cartouche PDF indique que le plan est à l'échelle `1/20e`**, voici comment retrouver vos mesures réelles en mètres/millimètres dans AutoCAD :
* Cliquez sur le bloc **"Échelle Globale"** (en bas à gauche de la fenêtre).
* Rentrez un nouveau ratio inverse. Pour rétablir un plan au 1/20e pour qu'il soit grandeur nature, tapez : **`20 / 1`**
* *Autre exemple : Si le plan imprimé est au 1/50e, tapez `50 / 1` pour que le logiciel multiplie toute la géométrie par 50 lors de la conversion.*

### 3. Lancement de la Conversion
Une fois le plan et l'échelle choisis, cliquez sur le gros bouton bleu **"Start Conversion"** en bas à droite.

### 4. Ouvrir vos fichiers en un clic 🖱️
Fini de chercher où les fichiers sont partis s'enregistrer ! 
Sur la **barre latérale gauche**, vous retrouverez l'historique complet de toutes vos conversions. 
**Cliquez directement sur un élément de cet historique** : le fichier DXF généré s'ouvrira illico avec votre logiciel installé par défaut (comme AutoCAD, DraftSight ou un viewer CAO).

---

## 📦 Installation (Windows)

1. Rendez-vous dans la section **Releases** et téléchargez le fichier d'installation (`pdf2dxf_1.0.0_x64-setup.exe`).
2. Lancez l'installation et cliquez sur suivant. Le programme se logera automatiquement dans votre menu Démarrer.

---

## 💻 Pour les Geeks (Détails techniques)

* **Backend (Rust) :** Le moteur lourd. Décodage natif du flux PDF via `lopdf`, décomposition mathématique des matrices de transformation (CTM) et conversion des courbes de Bézier en segments de droites exploitables.
* **Génération CAO :** Écriture native au format **DXF R12**. C'est la version la plus stable pour garantir une ouverture sans corruption d'en-tête sur n'importe quel logiciel du marché.
* **Frontend (React / Tauri v2) :** Interface minimaliste, communication inter-processus (IPC) ultra-rapide. Résultat : l'application consomme moins de RAM qu'un simple onglet de navigateur.

---

### 👨‍💻 Créé par *Hugo Burnet*
* 🌐 [Portfolio - CV Online](https://hugo-burnet.github.io/cv-online/)
* 📐 [Plateforme CalipiCAD](https://hugo-burnet.github.io/CalipiCAD/)

**Bonnes conversions et bonnes conceptions sur vos DXF générés !**