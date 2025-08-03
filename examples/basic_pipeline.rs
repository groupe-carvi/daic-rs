// Exemple basique utilisant seulement les bindings générés
use daic_sys::{
    root::{DeviceHandle, PipelineHandle},
    device_create, device_destroy, device_is_connected,
    pipeline_create, pipeline_destroy, pipeline_start, pipeline_stop, pipeline_is_running,
    dai_get_last_error, dai_clear_last_error
};
use std::ffi::CStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Création d'un device DepthAI...");
    
    // Créer le device
    let device = unsafe { device_create() };
    if device.is_null() {
        let error = unsafe { 
            let err_ptr = dai_get_last_error();
            if !err_ptr.is_null() {
                CStr::from_ptr(err_ptr).to_string_lossy().to_string()
            } else {
                "Erreur inconnue lors de la création du device".to_string()
            }
        };
        return Err(format!("Impossible de créer le device: {}", error).into());
    }
    
    println!("Device créé avec succès!");
    
    // Vérifier la connexion
    let is_connected = unsafe { device_is_connected(device) };
    println!("Device connecté: {}", is_connected);
    
    // Créer un pipeline
    let pipeline = unsafe { pipeline_create() };
    if pipeline.is_null() {
        unsafe { device_destroy(device) };
        return Err("Impossible de créer le pipeline".into());
    }
    
    println!("Pipeline créé avec succès!");
    
    // Démarrer le pipeline (probablement échouera sans device physique)
    let pipeline_started = unsafe { pipeline_start(pipeline, device) };
    println!("Pipeline démarré: {}", pipeline_started);
    
    if pipeline_started {
        let is_running = unsafe { pipeline_is_running(pipeline) };
        println!("Pipeline en cours d'exécution: {}", is_running);
        
        // Arrêter le pipeline
        let stopped = unsafe { pipeline_stop(pipeline) };
        println!("Pipeline arrêté: {}", stopped);
    }
    
    // Nettoyage
    unsafe { 
        pipeline_destroy(pipeline);
        device_destroy(device);
        dai_clear_last_error();
    }
    
    println!("Nettoyage terminé avec succès!");
    Ok(())
}
