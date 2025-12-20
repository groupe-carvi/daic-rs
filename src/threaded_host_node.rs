use std::ffi::{c_void, CString};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;
use std::sync::{Arc, Mutex};

use autocxx::c_int;
use depthai_sys::{depthai, DaiNode};

use crate::error::{clear_error_flag, last_error, Result};
use crate::output::{Input, Output};
use crate::pipeline::{Node, Pipeline, PipelineInner};

pub trait ThreadedHostNodeImpl: Send + 'static {
    fn run(&mut self, ctx: &ThreadedHostNodeContext);
    fn on_start(&mut self) {}
    fn on_stop(&mut self) {}
}

#[derive(Clone)]
pub struct ThreadedHostNode {
    node: Node,
}

impl ThreadedHostNode {
    pub(crate) fn from_handle(pipeline: Arc<PipelineInner>, handle: DaiNode) -> Self {
        Self {
            node: Node::from_handle(pipeline, handle),
        }
    }

    pub fn as_node(&self) -> &Node {
        &self.node
    }

    pub fn create_input(&self, name: Option<&str>) -> Result<Input> {
        self.create_input_with(name, None, None)
    }

    pub fn create_input_with(
        &self,
        name: Option<&str>,
        group: Option<&str>,
        queue_size: Option<i32>,
    ) -> Result<Input> {
        clear_error_flag();
        let name_c = name
            .map(|s| CString::new(s).map_err(|_| last_error("invalid input name")))
            .transpose()?;
        let group_c = group
            .map(|s| CString::new(s).map_err(|_| last_error("invalid input group")))
            .transpose()?;
        let queue_size = queue_size.unwrap_or(-1);
        let handle = unsafe {
            depthai::dai_threaded_hostnode_create_input(
                self.node.handle(),
                name_c.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null()),
                group_c.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null()),
                true,
                c_int(queue_size),
                false,
            )
        };
        if handle.is_null() {
            Err(last_error("failed to create threaded host input"))
        } else {
            Ok(Input::from_handle(Arc::clone(&self.node.pipeline), handle))
        }
    }

    pub fn create_output(&self, name: Option<&str>) -> Result<Output> {
        self.create_output_with(name, None)
    }

    pub fn create_output_with(&self, name: Option<&str>, group: Option<&str>) -> Result<Output> {
        clear_error_flag();
        let name_c = name
            .map(|s| CString::new(s).map_err(|_| last_error("invalid output name")))
            .transpose()?;
        let group_c = group
            .map(|s| CString::new(s).map_err(|_| last_error("invalid output group")))
            .transpose()?;
        let handle = unsafe {
            depthai::dai_threaded_hostnode_create_output(
                self.node.handle(),
                name_c.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null()),
                group_c.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null()),
            )
        };
        if handle.is_null() {
            Err(last_error("failed to create threaded host output"))
        } else {
            Ok(Output::from_handle(Arc::clone(&self.node.pipeline), handle))
        }
    }
}

pub struct ThreadedHostNodeContext {
    node: DaiNode,
}

impl ThreadedHostNodeContext {
    pub(crate) fn new(node: DaiNode) -> Self {
        Self { node }
    }

    pub fn is_running(&self) -> bool {
        unsafe { depthai::dai_threaded_node_is_running(self.node) }
    }
}

pub(crate) fn create_threaded_host_node<T, F>(pipeline: &Pipeline, init: F) -> Result<ThreadedHostNode>
where
    T: ThreadedHostNodeImpl,
    F: FnOnce(&ThreadedHostNode) -> Result<T>,
{
    clear_error_flag();
    let state = Box::new(ThreadedHostNodeState::<T> {
        inner: Mutex::new(None),
        node: Mutex::new(ptr::null_mut()),
    });
    let ctx = Box::into_raw(state) as *mut c_void;
    let handle = unsafe {
        depthai::dai_pipeline_create_threaded_host_node(
            pipeline.handle(),
            ctx as *mut _,
            Some(threaded_hostnode_run::<T>),
            Some(threaded_hostnode_on_start::<T>),
            Some(threaded_hostnode_on_stop::<T>),
            Some(threaded_hostnode_drop::<T>),
        )
    };
    if handle.is_null() {
        unsafe { drop(Box::from_raw(ctx as *mut ThreadedHostNodeState<T>)) };
        return Err(last_error("failed to create threaded host node"));
    }

    let node = ThreadedHostNode::from_handle(pipeline.inner_arc(), handle);
    {
        let state = unsafe { &*(ctx as *mut ThreadedHostNodeState<T>) };
        let mut guard = state.node.lock().unwrap_or_else(|e| e.into_inner());
        *guard = handle;
    }

    let impl_node = init(&node)?;
    {
        let state = unsafe { &*(ctx as *mut ThreadedHostNodeState<T>) };
        let mut guard = state.inner.lock().unwrap_or_else(|e| e.into_inner());
        *guard = Some(impl_node);
    }

    Ok(node)
}

struct ThreadedHostNodeState<T: ThreadedHostNodeImpl> {
    inner: Mutex<Option<T>>,
    node: Mutex<DaiNode>,
}

unsafe extern "C" fn threaded_hostnode_run<T: ThreadedHostNodeImpl>(ctx: *mut c_void) {
    if ctx.is_null() {
        return;
    }
    let state = unsafe { &*(ctx as *mut ThreadedHostNodeState<T>) };
    let node = {
        let guard = state.node.lock().unwrap_or_else(|e| e.into_inner());
        *guard
    };
    let mut guard = match state.inner.lock() {
        Ok(g) => g,
        Err(e) => e.into_inner(),
    };
    let Some(inner) = guard.as_mut() else {
        return;
    };
    let ctx = ThreadedHostNodeContext::new(node);
    let _ = catch_unwind(AssertUnwindSafe(|| inner.run(&ctx)));
}

unsafe extern "C" fn threaded_hostnode_on_start<T: ThreadedHostNodeImpl>(ctx: *mut c_void) {
    if ctx.is_null() {
        return;
    }
    let state = unsafe { &*(ctx as *mut ThreadedHostNodeState<T>) };
    let mut guard = match state.inner.lock() {
        Ok(g) => g,
        Err(e) => e.into_inner(),
    };
    let Some(inner) = guard.as_mut() else {
        return;
    };
    let _ = catch_unwind(AssertUnwindSafe(|| inner.on_start()));
}

unsafe extern "C" fn threaded_hostnode_on_stop<T: ThreadedHostNodeImpl>(ctx: *mut c_void) {
    if ctx.is_null() {
        return;
    }
    let state = unsafe { &*(ctx as *mut ThreadedHostNodeState<T>) };
    let mut guard = match state.inner.lock() {
        Ok(g) => g,
        Err(e) => e.into_inner(),
    };
    let Some(inner) = guard.as_mut() else {
        return;
    };
    let _ = catch_unwind(AssertUnwindSafe(|| inner.on_stop()));
}

unsafe extern "C" fn threaded_hostnode_drop<T: ThreadedHostNodeImpl>(ctx: *mut c_void) {
    if ctx.is_null() {
        return;
    }
    unsafe { drop(Box::from_raw(ctx as *mut ThreadedHostNodeState<T>)) };
}
