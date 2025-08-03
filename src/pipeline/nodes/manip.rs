use crate::error::DaiResult;
use crate::pipeline::PipelineNode;
use daic_sys::root::dai::node::ImageManip as DaiImageManip;

// Types locaux pour ImageManip
#[derive(Debug, Clone)]
pub enum ResizeMode {
    Crop,
    Letterbox,
    Stretch,
}

#[derive(Debug, Clone)]
pub struct ResizeConfig {
    pub width: u32,
    pub height: u32,
    pub mode: ResizeMode,
    pub keep_aspect_ratio: bool,
}

#[derive(Debug, Clone)]
pub struct CropConfig {
    pub x_min: f32,
    pub y_min: f32,
    pub x_max: f32,
    pub y_max: f32,
}

#[derive(Debug, Clone)]
pub struct ImageManipConfig {
    pub resize: Option<ResizeConfig>,
    pub crop: Option<CropConfig>,
    pub keep_aspect_ratio: Option<bool>,
    pub interpolation: Option<u32>,
}

impl Default for ImageManipConfig {
    fn default() -> Self {
        Self {
            resize: None,
            crop: None,
            keep_aspect_ratio: Some(true),
            interpolation: None,
        }
    }
}

pub struct ImageManip {
    inner: Option<*mut DaiImageManip>,
    config: ImageManipConfig,
}

impl ImageManip {
    pub fn new() -> Self {
        Self {
            inner: None,
            config: ImageManipConfig::default(),
        }
    }

    pub fn with_config(config: ImageManipConfig) -> Self {
        Self {
            inner: None,
            config,
        }
    }

    pub fn set_resize(&mut self, width: u32, height: u32, mode: ResizeMode) -> &mut Self {
        self.config.resize = Some(ResizeConfig {
            width,
            height,
            mode,
            keep_aspect_ratio: true,
        });
        
        if let Some(ref mut manip) = self.inner {
            unsafe {
                // TODO: Appeler les fonctions C++ pour configurer le resize
            }
        }
        self
    }

    pub fn set_crop(&mut self, x_min: f32, y_min: f32, x_max: f32, y_max: f32) -> &mut Self {
        self.config.crop = Some(CropConfig {
            x_min,
            y_min,
            x_max,
            y_max,
        });
        
        if let Some(ref mut manip) = self.inner {
            unsafe {
                // TODO: Appeler les fonctions C++ pour configurer le crop
            }
        }
        self
    }

    pub fn set_keep_aspect_ratio(&mut self, keep: bool) -> &mut Self {
        self.config.keep_aspect_ratio = Some(keep);
        
        if let Some(ref mut manip) = self.inner {
            unsafe {
                // TODO: Appeler les fonctions C++ pour configurer l'aspect ratio
            }
        }
        self
    }

    pub fn set_interpolation(&mut self, interpolation: u32) -> &mut Self {
        self.config.interpolation = Some(interpolation);
        
        if let Some(ref mut manip) = self.inner {
            unsafe {
                // TODO: Appeler les fonctions C++ pour configurer l'interpolation
            }
        }
        self
    }

    pub fn get_resize_config(&self) -> Option<&ResizeConfig> {
        self.config.resize.as_ref()
    }

    pub fn get_crop_config(&self) -> Option<&CropConfig> {
        self.config.crop.as_ref()
    }
}

impl PipelineNode for ImageManip {
    fn id(&self) -> String {
        if let Some(_manip) = self.inner {
            unsafe {
                // TODO: Récupérer l'ID du nœud C++
                "image_manip".to_string()
            }
        } else {
            "image_manip".to_string()
        }
    }

    fn node_type(&self) -> String {
        "ImageManip".to_string()
    }

    fn configure(&mut self) -> DaiResult<()> {
        if self.inner.is_none() {
            unsafe {
                let manip = Box::new(std::mem::zeroed::<DaiImageManip>());
                self.inner = Some(Box::into_raw(manip));
                
                // TODO: Initialiser le nœud ImageManip C++
                
                // Appliquer la configuration
                if let Some(ref resize) = self.config.resize {
                    // TODO: Configurer le resize
                }
                
                if let Some(ref crop) = self.config.crop {
                    // TODO: Configurer le crop
                }
                
                if let Some(keep_aspect) = self.config.keep_aspect_ratio {
                    // TODO: Configurer l'aspect ratio
                }
                
                if let Some(interpolation) = self.config.interpolation {
                    // TODO: Configurer l'interpolation
                }
            }
        }
        Ok(())
    }
}

impl Drop for ImageManip {
    fn drop(&mut self) {
        if let Some(manip) = self.inner.take() {
            unsafe {
                let _ = Box::from_raw(manip);
                // TODO: Nettoyer les ressources C++
            }
        }
    }
}

impl Default for ImageManip {
    fn default() -> Self {
        Self::new()
    }
}
