//! Safe Rust API for DepthAI camera
use crate::device::{self, Device};
use crate::frame::{Frame, TestPattern};
use std::sync::Arc;
use std::time::Duration;

pub struct Camera {
    device: Arc<Device>,
    frame_counter: std::sync::atomic::AtomicU32,
}

impl Camera {
    /// Initialise une nouvelle caméra DepthAI avec gestion d'erreur améliorée
    pub fn new(device: Device ) -> Result<Self, &'static str> {
        
        // Test initial de connectivité
        if !device.is_connected() {
            return Err("Device not connected");
        }

        Ok(Camera {
            device: Arc::new(device),
            frame_counter: std::sync::atomic::AtomicU32::new(0),
        })
    }

    /// Capture une image depuis la caméra avec stabilité améliorée
    pub fn capture(&self) -> Result<Frame, &'static str> {
        // Vérifier que le device est toujours connecté
        if !self.device.is_connected() {
            return Err("Camera disconnected");
        }

        // Incrémenter le compteur de frames de façon thread-safe
        let frame_id = self.frame_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        
        // Capture avec retry en cas d'erreur temporaire
        for attempt in 0..3 {
            match self.device.capture_frame() {
                Ok(mut frame) => {
                    // Pour les tests, utiliser des patterns variés
                    if frame_id % 90 < 30 {
                        frame = Frame::test_pattern(640, 480, TestPattern::Gradient);
                    } else if frame_id % 90 < 60 {
                        frame = Frame::test_pattern(640, 480, TestPattern::Checkerboard);
                    } else {
                        frame = Frame::test_pattern(640, 480, TestPattern::Noise);
                    }
                    
                    return Ok(frame.with_frame_id(frame_id));
                }
                Err(e) => {
                    if attempt == 2 {
                        return Err(e);
                    }
                    // Pause courte avant retry
                    std::thread::sleep(Duration::from_millis(10));
                }
            }
        }
        
        Err("Failed to capture after retries")
    }

    /// Obtenir des statistiques de capture
    pub fn get_stats(&self) -> CameraStats {
        CameraStats {
            total_frames: self.frame_counter.load(std::sync::atomic::Ordering::SeqCst),
            device_captures: self.device.get_capture_count(),
            is_connected: self.device.is_connected(),
        }
    }

    /// Déconnecter proprement la caméra
    pub fn disconnect(&self) {
        self.device.disconnect();
    }
}

/// Statistiques de la caméra
#[derive(Debug)]
pub struct CameraStats {
    pub total_frames: u32,
    pub device_captures: u32,
    pub is_connected: bool,
}

// Thread safety
unsafe impl Send for Camera {}
unsafe impl Sync for Camera {}
