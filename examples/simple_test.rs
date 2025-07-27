use daic_rs::{device::Device, camera::Camera, frame::Frame};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Test de base de l'API DepthAI...");
    
    // Test 1: Création du device
    println!("📱 Test 1: Création du device...");
    let device = Device::new()?;
    println!("✓ Device créé");
    
    // Test 2: Création de la caméra
    println!("📷 Test 2: Création de la caméra...");
    let camera = Camera::new(&device)?;
    println!("✓ Caméra créée");
    
    // Test 3: Capture de quelques frames avec délai
    println!("🎬 Test 3: Capture de 5 frames avec délai...");
    for i in 1..=5 {
        println!("  Capture frame {}...", i);
        match camera.capture() {
            Ok(frame) => {
                let stats = camera.get_stats();
                println!("  ✓ Frame {} capturée - Total: {}, Success: {}, Errors: {}", 
                         i, stats.total_frames, stats.successful_captures, stats.errors);
            }
            Err(e) => {
                println!("  ✗ Erreur frame {}: {}", i, e);
                break;
            }
        }
        
        // Attendre 100ms entre captures pour éviter saturation
        thread::sleep(Duration::from_millis(100));
    }
    
    // Test 4: Statistiques finales
    let final_stats = camera.get_stats();
    println!("📊 Statistiques finales:");
    println!("  Total frames: {}", final_stats.total_frames);
    println!("  Captures réussies: {}", final_stats.successful_captures);
    println!("  Erreurs: {}", final_stats.errors);
    println!("  Taux de succès: {:.1}%", 
             (final_stats.successful_captures as f64 / final_stats.total_frames as f64) * 100.0);
    
    println!("✅ Test terminé avec succès");
    Ok(())
}
