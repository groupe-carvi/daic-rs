use crate::common::ImageFrameType;
use crate::error::{DepthaiError, Result};
use crate::output::Input;
use crate::threaded_host_node::{ThreadedHostNode, ThreadedHostNodeContext};
use crate::{depthai_threaded_host_node, CreateInPipelineWith, Pipeline};

use rerun as rr;

pub struct RerunWebConfig {
    pub bind_ip: String,
    pub web_port: u16,
    pub open_browser: bool,
    pub connect_url: Option<String>,
}

impl Default for RerunWebConfig {
    fn default() -> Self {
        Self {
            bind_ip: "0.0.0.0".to_string(),
            web_port: 0,
            open_browser: true,
            connect_url: None,
        }
    }
}

pub enum RerunViewer {
    Web(RerunWebConfig),
    Native,
}

pub struct RerunHostNodeConfig {
    pub app_id: String,
    pub entity_path: String,
    pub viewer: RerunViewer,
    pub input_name: String,
}

impl Default for RerunHostNodeConfig {
    fn default() -> Self {
        Self {
            app_id: "depthai_rerun".to_string(),
            entity_path: "camera".to_string(),
            viewer: RerunViewer::Web(RerunWebConfig::default()),
            input_name: "in".to_string(),
        }
    }
}

#[depthai_threaded_host_node]
struct RerunHostNodeImpl {
    input: Input,
    rec: rr::RecordingStream,
    _web_server: Option<Box<dyn std::any::Any + Send>>,
    entity_path: String,
    frame_index: i64,
}

impl RerunHostNodeImpl {
    pub fn new(input: Input, config: RerunHostNodeConfig) -> Result<Self> {
        let (rec, server) = match config.viewer {
            RerunViewer::Web(web) => {
                let rec = rr::RecordingStreamBuilder::new(config.app_id.clone())
                    .serve_grpc()
                    .map_err(rerun_err)?;
                let mut viewer = rr::web_viewer::WebViewerConfig::default();
                viewer.bind_ip = web.bind_ip;
                viewer.web_port = re_web_viewer_server::WebViewerServerPort(web.web_port);
                viewer.open_browser = web.open_browser;
                viewer.connect_to = vec![
                    web.connect_url
                        .unwrap_or_else(|| rr::DEFAULT_CONNECT_URL.to_string()),
                ];
                let server = rr::serve_web_viewer(viewer).map_err(rerun_err)?;
                (rec, Some(Box::new(server) as Box<dyn std::any::Any + Send>))
            }
            RerunViewer::Native => {
                let rec = rr::RecordingStreamBuilder::new(config.app_id.clone())
                    .spawn()
                    .map_err(rerun_err)?;
                (rec, None)
            }
        };

        Ok(Self {
            input,
            rec,
            _web_server: server,
            entity_path: config.entity_path,
            frame_index: 0,
        })
    }

    pub fn run(&mut self, ctx: &ThreadedHostNodeContext) {
        while ctx.is_running() {
            match self.input.get_frame() {
                Ok(frame) => {
                    let _ = self.log_frame(&frame);
                }
                Err(_) => break,
            }
        }
    }

    fn log_frame(&mut self, frame: &crate::camera::ImageFrame) -> Result<()> {
        let w = frame.width();
        let h = frame.height();
        let format = frame.format();
        let bytes = frame.bytes();

        let image = match format {
            Some(ImageFrameType::RGB888i) => {
                rr::Image::from_rgb24(bytes, [w, h])
            }
            Some(ImageFrameType::BGR888i) => {
                let mut rgb = bytes;
                for chunk in rgb.chunks_exact_mut(3) {
                    chunk.swap(0, 2);
                }
                rr::Image::from_rgb24(rgb, [w, h])
            }
            Some(ImageFrameType::GRAY8) => {
                rr::Image::from_l8(bytes, [w, h])
            }
            _ => {
                return Ok(());
            }
        };

        self.rec.set_time_sequence("frame", self.frame_index);
        self.frame_index += 1;
        self.rec
            .log(self.entity_path.as_str(), &image)
            .map_err(rerun_err)?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct RerunHostNode {
    node: ThreadedHostNode,
}

impl RerunHostNode {
    pub fn as_node(&self) -> &crate::pipeline::Node {
        self.node.as_node()
    }

    pub fn input(&self, name: &str) -> Result<Input> {
        self.as_node().input(name)
    }
}

impl CreateInPipelineWith<RerunHostNodeConfig> for RerunHostNode {
    fn create_with(pipeline: &Pipeline, config: RerunHostNodeConfig) -> Result<Self> {
        let input_name = config.input_name.clone();
        let node = pipeline.create_threaded_host_node(|node| {
            let input = node.create_input(Some(&input_name))?;
            RerunHostNodeImpl::new(input, config)
        })?;
        Ok(Self { node })
    }
}

pub fn create_rerun_host_node(
    pipeline: &Pipeline,
    input_name: &str,
    config: RerunHostNodeConfig,
) -> Result<RerunHostNode> {
    let mut config = config;
    config.input_name = input_name.to_string();
    RerunHostNode::create_with(pipeline, config)
}

fn rerun_err(err: impl std::fmt::Display) -> DepthaiError {
    DepthaiError::new(format!("rerun error: {}", err))
}
