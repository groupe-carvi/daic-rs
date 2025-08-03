# DepthAI Pipeline API avec Bindings C++ Intégrés

Ce document décrit l'implémentation complète de l'API DepthAI en Rust avec les bindings C++ réels intégrés.

## Architecture Modulaire

### Structure des Modules

```
src/
├── pipeline/
│   ├── mod.rs           # Module principal avec exports
│   ├── core.rs          # Pipeline et Device avec bindings C réels
│   ├── nodes/           # Implémentations de nœuds spécialisés
│   │   ├── mod.rs       # Exports des nœuds
│   │   ├── camera.rs    # Nœud caméra avec API C intégrée
│   │   ├── mono_camera.rs
│   │   ├── neural_network.rs
│   │   ├── stereo_depth.rs
│   │   ├── image_manip.rs
│   │   ├── xlink_out.rs
│   │   └── xlink_in.rs
│   ├── data.rs          # Types de données (ImgFrame, NNData, IMUData)
│   └── builder.rs       # Pattern Builder pour création fluide
├── device.rs            # Device avec bindings C FFI intégrés
└── error.rs             # Gestion d'erreurs étendue
```

### Intégration des Bindings C++

#### daic-sys/src/
- `core_bindings.rs` - Fonctions C principales (device, pipeline)
- `camera_bindings.rs` - API caméra C spécialisée
- `lib.rs` - Exports et re-exports des bindings

## Fonctionnalités Implémentées

### ✅ Complètement Intégré

1. **Device Management**
   - `device_create()`, `device_destroy()`, `device_is_connected()`
   - Gestion sécurisée des handles avec Drop trait
   - Gestion d'erreurs FFI avec `dai_get_last_error()`

2. **Pipeline Core**
   - `pipeline_create()`, `pipeline_destroy()`
   - `pipeline_start()`, `pipeline_stop()`, `pipeline_is_running()`
   - Gestion du cycle de vie complète

3. **Camera Node**
   - `camera_create()`, `camera_destroy()`
   - `camera_set_board_socket()`, `camera_set_resolution()`, `camera_set_fps()`
   - `camera_request_preview()`, `camera_request_video()`, `camera_request_still()`
   - Configuration complète avec validation d'erreurs

### ⚠️ En Attente d'Implémentation

4. **Autres Nodes** (structure prête, bindings C à ajouter)
   - MonoCamera, NeuralNetwork, StereoDepth, ImageManip
   - XLinkOut, XLinkIn pour communication

5. **Data Flow** (structure prête, bindings C à ajouter)
   - ImgFrame, NNData, IMUData wrappers
   - Système de connexion entre nœuds

## Utilisation

### Exemple Basic

```rust
use daic_rs::{
    device::Device,
    pipeline::{
        core::{Pipeline, PipelineConfig},
        nodes::camera::{Camera, BoardSocket, ResolutionPreset},
    },
};

fn main() -> DaiResult<()> {
    // Créer un device DepthAI
    let device = Device::new()?;
    
    // Créer un pipeline
    let mut pipeline = Pipeline::new()?;
    
    // Créer et configurer une caméra
    let mut camera = Camera::new("main_camera")?;
    camera.set_board_socket(BoardSocket::CamA)?;
    camera.set_resolution(ResolutionPreset::The1080P)?;
    camera.set_fps(30.0)?;
    
    // Demander les sorties
    camera.request_preview_output("preview")?;
    camera.request_video_output("video")?;
    
    // Ajouter au pipeline et démarrer
    pipeline.add_node(camera)?;
    pipeline.start(&device)?;
    
    // Arrêter proprement
    pipeline.stop()?;
    
    Ok(())
}
```

### Pattern Builder (Implémenté)

```rust
use daic_rs::pipeline::builder::PipelineBuilder;

let pipeline = PipelineBuilder::new()
    .add_camera("main_cam")
        .board_socket(BoardSocket::CamA)
        .resolution(ResolutionPreset::The1080P)
        .fps(30.0)
        .request_preview("preview")
        .request_video("video")
    .add_neural_network("detection")
        .blob_path("./models/yolo.blob")
        .input_size(416, 416)
    .connect("main_cam.preview", "detection.input")
    .build()?;
```

## Sécurité et Gestion d'Erreurs

### Handles Sécurisés

Tous les handles C++ sont encapsulés dans des structures Rust sécurisées avec :
- **Drop trait** automatique pour libération des ressources
- **Validation de nullité** des pointeurs
- **Gestion d'erreurs FFI** avec messages détaillés

### Gestion d'Erreurs

```rust
pub enum DaiError {
    DeviceError(String),
    PipelineStartFailed(String),
    ConfigurationError(String),
    FfiError(String),
    // ... autres variantes
}
```

### Thread Safety

- `Device` et `Pipeline` implémentent `Send` mais pas `Sync`
- Utilisation d'`Arc<Mutex<>>` pour partage entre threads quand nécessaire
- Gestion des locks avec timeouts appropriés

## Tests et Validation

### Tests Unitaires

Chaque module inclut des tests qui :
- Testent la création sans device physique (graceful failure)
- Valident la configuration des paramètres
- Vérifient la gestion des erreurs

### Tests d'Intégration

- `examples/pipeline_with_real_bindings.rs` - Exemple complet
- Tests conditionnels avec `#[cfg(feature = "hardware-tests")]`

## Prochaines Étapes

### Phase 2 : Completion des Nodes

1. **MonoCamera** - API caméra monochrome
2. **NeuralNetwork** - Inférence AI avec modèles .blob
3. **StereoDepth** - Calcul de profondeur stéréo
4. **ImageManip** - Manipulation d'images (crop, resize, rotate)

### Phase 3 : Data Flow

1. **Système de Connexion** - Liaison entre nœuds
2. **Stream Processing** - Gestion des flux de données
3. **Synchronisation** - Synchronisation multi-caméras

### Phase 4 : Fonctionnalités Avancées

1. **Holistic Recording/Replay** - Enregistrement complet
2. **IMU Integration** - Données inertiales
3. **Pipeline Optimization** - Optimisations performance

## Notes Techniques

### Compilation

Les bindings C nécessitent :
- DepthAI C++ library liée
- Headers DepthAI disponibles
- `bindgen` pour génération automatique

### Performance

- Zéro-cost abstractions Rust
- Appels FFI directs sans overhead
- Gestion mémoire optimisée avec RAII

### Compatibilité

- Compatible avec DepthAI API 2.x
- Support multi-plateforme (Windows, Linux, macOS)
- Intégration cmake via `build.rs`

Cette implémentation fournit une base solide pour l'utilisation complète de DepthAI depuis Rust avec la sécurité et l'ergonomie du langage Rust combinées à la puissance de l'API DepthAI C++.
