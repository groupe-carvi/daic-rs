use crate::common::ImageFrameType;
use crate::error::{DepthaiError, Result};
use crate::output::Input;
use crate::threaded_host_node::{ThreadedHostNode, ThreadedHostNodeContext};
use crate::{depthai_threaded_host_node, CreateInPipelineWith, Pipeline};

use rerun as rr;

use std::time::{Duration, Instant};

pub struct RerunWebConfig {
    pub bind_ip: String,
    /// Port for hosting the Web Viewer (HTTP).
    ///
    /// Note: Rerun's `WebViewerServerPort::AUTO` picks a random port, which is inconvenient for
    /// examples and remote dev. We default to 9090 (same as the `rerun` CLI).
    pub web_port: u16,
    pub open_browser: bool,
    pub connect_url: Option<String>,
}

impl Default for RerunWebConfig {
    fn default() -> Self {
        Self {
            bind_ip: "0.0.0.0".to_string(),
            web_port: 9090,
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
    // The Rerun gRPC server + web-viewer server require a Tokio runtime.
    // Keep it alive for the whole lifetime of the node.
    #[cfg(feature = "rerun")]
    _tokio_rt: Option<tokio::runtime::Runtime>,
    entity_path: String,
    frame_index: i64,
    received_frames: u64,
    logged_frames: u64,
    skipped_frames: u64,
    last_stats: Instant,
    last_skip_note: Instant,
}

impl RerunHostNodeImpl {
    pub fn new(input: Input, config: RerunHostNodeConfig) -> Result<Self> {
        match config.viewer {
            RerunViewer::Web(web) => {
                // Rerun's serving utilities rely on a Tokio runtime existing in the current context.
                // We create one dedicated runtime for this node and keep it alive.
                let rt = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .map_err(|e| DepthaiError::new(format!("failed to create tokio runtime: {e}")))?;

                // Temporarily enter the runtime so rerun can spawn background tasks.
                let _guard = rt.enter();

                let rec = rr::RecordingStreamBuilder::new(config.app_id.clone())
                    .serve_grpc()
                    .map_err(rerun_err)?;

                // The URL the browser will use to fetch data (via the /proxy endpoint).
                // Default is 127.0.0.1 which is perfect when the browser is on the same machine,
                // but if you're using a remote dev environment you may need to override this
                // (or port-forward the gRPC proxy port).
                let connect_to = web.connect_url.unwrap_or_else(|| {
                    if web.bind_ip != "0.0.0.0" {
                        format!(
                            "rerun+http://{}:{}/proxy",
                            web.bind_ip,
                            rr::DEFAULT_SERVER_PORT
                        )
                    } else {
                        rr::DEFAULT_CONNECT_URL.to_string()
                    }
                });

                let web_server = rr::serve_web_viewer(rr::web_viewer::WebViewerConfig {
                    bind_ip: web.bind_ip.clone(),
                    web_port: re_web_viewer_server::WebViewerServerPort(web.web_port),
                    open_browser: web.open_browser,
                    connect_to: vec![connect_to.clone()],
                    ..Default::default()
                })
                .map_err(rerun_err)?;

                eprintln!("rerun: gRPC /proxy connect URL: {connect_to}");
                eprintln!("rerun: web viewer served at: {}", web_server.server_url());
                web_server.detach();

                eprintln!(
                    "rerun: host node starting (viewer=web, entity_path='{}')",
                    config.entity_path
                );

                Ok(Self {
                    input,
                    rec,
                    _tokio_rt: Some(rt),
                    entity_path: config.entity_path,
                    frame_index: 0,
                    received_frames: 0,
                    logged_frames: 0,
                    skipped_frames: 0,
                    last_stats: Instant::now(),
                    last_skip_note: Instant::now() - Duration::from_secs(60),
                })
            }
            RerunViewer::Native => {
                let rec = rr::RecordingStreamBuilder::new(config.app_id.clone())
                    .spawn()
                    .map_err(rerun_err)?;

                eprintln!(
                    "rerun: host node starting (viewer=native, entity_path='{}')",
                    config.entity_path
                );

                Ok(Self {
                    input,
                    rec,
                    _tokio_rt: None,
                    entity_path: config.entity_path,
                    frame_index: 0,
                    received_frames: 0,
                    logged_frames: 0,
                    skipped_frames: 0,
                    last_stats: Instant::now(),
                    last_skip_note: Instant::now() - Duration::from_secs(60),
                })
            }
        }
    }

    pub fn run(&mut self, ctx: &ThreadedHostNodeContext) {
        while ctx.is_running() {
            match self.input.get_frame() {
                Ok(frame) => {
                    self.received_frames += 1;

                    // Print periodic stats so we can tell whether we are receiving frames at all.
                    if self.last_stats.elapsed() >= Duration::from_secs(2) {
                        let (w, h) = (frame.width(), frame.height());
                        eprintln!(
                            "rerun: stats: received={} logged={} skipped={} last_frame={}x{} format={:?}",
                            self.received_frames,
                            self.logged_frames,
                            self.skipped_frames,
                            w,
                            h,
                            frame.format()
                        );
                        self.last_stats = Instant::now();
                    }

                    if let Err(e) = self.log_frame(&frame) {
                        // Previously we silently ignored errors which makes debugging painful.
                        eprintln!("rerun: failed to process frame: {e}");
                    }
                }
                Err(e) => {
                    eprintln!("rerun: input.get_frame() failed; stopping host node: {e}");
                    break;
                }
            }
        }

        eprintln!(
            "rerun: host node stopping (received={} logged={} skipped={})",
            self.received_frames, self.logged_frames, self.skipped_frames
        );
    }

    fn log_frame(&mut self, frame: &crate::camera::ImageFrame) -> Result<()> {
        let w = frame.width();
        let h = frame.height();
        let format = frame.format();
        let mut bytes = frame.bytes();

        // Helpful sanity check: if the byte length doesn't match the expected format,
        // log a note (rate-limited) and skip to avoid confusing downstream.
        let expected_len = match format {
            Some(ImageFrameType::RGB888i) | Some(ImageFrameType::BGR888i) => {
                Some((w as usize).saturating_mul(h as usize).saturating_mul(3))
            }
            Some(ImageFrameType::GRAY8) => Some((w as usize).saturating_mul(h as usize)),
            _ => None,
        };

        if let Some(expected) = expected_len {
            if bytes.len() < expected {
                self.skipped_frames += 1;
                if self.last_skip_note.elapsed() >= Duration::from_secs(2) {
                    eprintln!(
                        "rerun: skipping frame due to short buffer: {}x{} format={:?} bytes_len={} expected_len={}",
                        w,
                        h,
                        format,
                        bytes.len(),
                        expected
                    );
                    self.last_skip_note = Instant::now();
                }
                return Ok(());
            }

            if bytes.len() > expected {
                // Some backends may include padding or metadata. Log a warning and truncate so we
                // still try to produce an image instead of silently skipping everything.
                if self.last_skip_note.elapsed() >= Duration::from_secs(2) {
                    eprintln!(
                        "rerun: note: buffer larger than expected; truncating: {}x{} format={:?} bytes_len={} expected_len={}",
                        w,
                        h,
                        format,
                        bytes.len(),
                        expected
                    );
                    self.last_skip_note = Instant::now();
                }
                bytes.truncate(expected);
            }
        }

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
                self.skipped_frames += 1;
                if self.last_skip_note.elapsed() >= Duration::from_secs(2) {
                    eprintln!(
                        "rerun: skipping frame with unsupported/unknown format: {}x{} format={:?} bytes_len={}",
                        w,
                        h,
                        format,
                        bytes.len()
                    );
                    eprintln!(
                        "rerun: supported formats for logging are: RGB888i, BGR888i, GRAY8 (hint: set CameraOutputConfig.frame_type=Some(ImageFrameType::RGB888i))"
                    );
                    self.last_skip_note = Instant::now();
                }
                return Ok(());
            }
        };

        self.rec.set_time_sequence("frame", self.frame_index);
        self.frame_index += 1;
        self.rec
            .log(self.entity_path.as_str(), &image)
            .map_err(rerun_err)?;

        self.logged_frames += 1;
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
