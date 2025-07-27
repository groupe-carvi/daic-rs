# Exemples DepthAI-RS avec Kornia et Rerun

Ce répertoire contient plusieurs exemples démontrant l'utilisation de DepthAI avec différentes bibliothèques de traitement d'image et de visualisation.

## Exemples disponibles

### 1. `camera_camera_all.rs` - Pipeline de base
**Description :** Exemple de base montrant la capture d'image et le traitement avec imageproc.

**Fonctionnalités :**
- ✅ Initialisation de la caméra DepthAI
- ✅ Capture d'image en niveaux de gris
- ✅ Conversion en ndarray
- ✅ Flou gaussien avec imageproc
- ✅ Flou simple avec convolution manuelle
- ✅ Statistiques d'image

**Exécution :**
```bash
cargo run --example camera_camera_all
```

### 2. `camera_rerun_viz.rs` - Visualisation avec Rerun
**Description :** Démonstration de la visualisation en temps réel avec Rerun.

**Fonctionnalités :**
- ✅ Capture d'image DepthAI
- ✅ Enregistrement Rerun dans un fichier .rrd
- ✅ Affichage de l'image originale
- ✅ Flou gaussien (σ=3.0)
- ✅ Détection de contours (gradient)
- ✅ Seuillage binaire (seuil=128)

**Exécution :**
```bash
cargo run --example camera_rerun_viz
rerun camera_session.rrd  # Ouverture du visualiseur
```

### 3. `camera_kornia_processing.rs` - Traitement avancé
**Description :** Implémentation manuelle d'opérations de traitement d'image style Kornia.

**Fonctionnalités :**
- ✅ Normalisation d'image [0,1]
- ✅ Convolution gaussienne manuelle
- ✅ Détection de contours Sobel
- ✅ Analyse statistique
- ✅ Normalisation de plage dynamique

**Exécution :**
```bash
cargo run --example camera_kornia_processing
```

## Dépendances

Les exemples utilisent les bibliothèques suivantes :

```toml
[dependencies]
image = "0.25.6"
ndarray = "0.16.1"
imageproc = "0.25.0"

[dev-dependencies]
kornia = "0.1.9"      # Pour le traitement d'image (API en développement)
rerun = "0.24.0"      # Pour la visualisation
```

## Architecture

```
src/
├── lib.rs                    # API principale
├── device.rs                 # Wrapper Device DepthAI
└── ...

daic-sys/
├── src/
│   ├── camera.rs            # API Camera safe
│   ├── frame.rs             # Structure Frame
│   ├── device.rs            # Device bas niveau
│   └── ...
└── examples/
    ├── camera_camera_all.rs
    ├── camera_rerun_viz.rs
    └── camera_kornia_processing.rs
```

## Formats d'image supportés

- **Entrée :** Images DepthAI (Vec<u8> + dimensions)
- **Traitement :** ndarray::Array2<u8> / Array2<f32>
- **ImageProc :** ImageBuffer<Luma<u8>>
- **Rerun :** Tenseurs et Images natives

## Pipeline de traitement type

1. **Capture** : `Camera::new()` → `camera.capture()`
2. **Conversion** : `Vec<u8>` → `ndarray::Array2`
3. **Traitement** : Filtres, détection, analyse
4. **Visualisation** : Rerun, sauvegarde fichier
5. **Statistiques** : Moyennes, min/max, histogrammes

## Notes de développement

- Les API Kornia et Rerun sont en évolution rapide
- Les exemples incluent des implémentations manuelles pour la robustesse
- Le support de Kornia intégré sera ajouté lorsque l'API sera stable
- Rerun utilise un format .rrd pour la persistance des sessions

## Prochaines étapes

- [ ] Intégration Kornia native (selon disponibilité API)
- [ ] Support des images couleur RGB
- [ ] Pipeline temps réel avec Rerun streaming
- [ ] Exemples avec différents modèles de caméra DepthAI
- [ ] Support de la profondeur et des données IMU

## Résolution de problèmes

Si vous rencontrez des erreurs de compilation :

1. Vérifiez que les dépendances DepthAI sont installées
2. Assurez-vous que les DLL sont dans le PATH
3. Utilisez `cargo clean` puis `cargo build` en cas de problème

Pour la visualisation Rerun :
- Installez `rerun-cli` : `pip install rerun-sdk`
- Ou utilisez le binaire : `cargo install rerun-cli`
