// use wgpu::{util::DeviceExt, CompositeAlphaMode};
// use winit::{
//     dpi::LogicalSize,
//     event::*,
//     event_loop::{ControlFlow, EventLoop},
//     platform::macos::WindowBuilderExtMacOS,
//     window::{WindowBuilder, Fullscreen},
// };

// #[tokio::main]
// async fn main() {
//     let event_loop = EventLoop::new();
//     let window = WindowBuilder::new()
//         .with_title("Overlay Window")
//         .with_transparent(true)
//         .with_decorations(false)
//         // .with_always_on_top(true)
//         .with_fullscreen(Some(Fullscreen::Borderless(None)))
//         .build(&event_loop)
//         .unwrap();
    
//     let backend = wgpu::Backends::PRIMARY;
//     let instance = wgpu::Instance::new(backend);
//     let surface = unsafe { instance.create_surface(&window) };
    
//     let adapter = instance
//         .request_adapter(&wgpu::RequestAdapterOptions {
//             power_preference: wgpu::PowerPreference::HighPerformance,
//             compatible_surface: Some(&surface),
//             force_fallback_adapter: false,
//         })
//         .await
//         .unwrap();
    
//     let (device, queue) = adapter
//         .request_device(
//             &wgpu::DeviceDescriptor {
//                 label: None,
//                 features: wgpu::Features::empty(),
//                 limits: wgpu::Limits::default(),
//             },
//             None,
//         )
//         .await
//         .unwrap();
    
//     let surface_format = surface.get_supported_formats(&adapter)[0];
//     let config = wgpu::SurfaceConfiguration {
//         usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//         alpha_mode: CompositeAlphaMode::Auto,
//         format: surface_format,
//         width: window.inner_size().width,
//         height: window.inner_size().height,
//         present_mode: wgpu::PresentMode::Fifo,
//     };
//     surface.configure(&device, &config);
    
//     let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
//         label: Some("Shader"),
//         source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
//     });
    
//     let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//         label: Some("Pipeline Layout"),
//         bind_group_layouts: &[],
//         push_constant_ranges: &[],
//     });
    
//     let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//         label: Some("Render Pipeline"),
//         layout: Some(&pipeline_layout),
//         vertex: wgpu::VertexState {
//             module: &shader,
//             entry_point: "vs_main",
//             buffers: &[],
//         },
//         fragment: Some(wgpu::FragmentState {
//             module: &shader,
//             entry_point: "fs_main",
//             targets: &[Some(wgpu::ColorTargetState {
//                 format: config.format,
//                 blend: Some(wgpu::BlendState::REPLACE),
//                 write_mask: wgpu::ColorWrites::ALL,
//             })],
//         }),
//         primitive: wgpu::PrimitiveState {
//             topology: wgpu::PrimitiveTopology::TriangleList,
//             strip_index_format: None,
//             front_face: wgpu::FrontFace::Ccw,
//             cull_mode: Some(wgpu::Face::Back),
//             ..Default::default()
//         },
//         depth_stencil: None,
//         multisample: wgpu::MultisampleState {
//             count: 1,
//             mask: !0,
//             alpha_to_coverage_enabled: false,
//         },
//         multiview: None,
//     });
    
//     let mut config = config;
//     event_loop.run(move |event, _, control_flow| {
//         match event {
//             Event::WindowEvent {
//                 event: WindowEvent::CloseRequested,
//                 ..
//             } => *control_flow = ControlFlow::Exit,
//             Event::RedrawRequested(_) => {
//                 let frame = surface
//                     .get_current_texture()
//                     .expect("Failed to acquire next surface texture");
//                 let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
    
//                 let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
//                     label: Some("Render Encoder"),
//                 });
    
//                 {
//                     let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//                         label: Some("Render Pass"),
//                         color_attachments: &[Some(wgpu::RenderPassColorAttachment {
//                             view: &view,
//                             resolve_target: None,
//                             ops: wgpu::Operations {
//                                 load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
//                                 store: true,
//                             },
//                         })],
//                         depth_stencil_attachment: None,
//                     });
    
//                     render_pass.set_pipeline(&pipeline);
//                     render_pass.draw(0..3, 0..1); // 삼각형을 그립니다. 여기서 도형을 그리는 코드를 수정할 수 있습니다.
//                 }
    
//                 queue.submit(Some(encoder.finish()));
//                 frame.present();
//             }
//             Event::MainEventsCleared => {
//                 window.request_redraw();
//             }
//             _ => {}
//         }
//     });
// }
extern crate cocoa;
extern crate objc;
extern crate core_graphics;

use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicy, NSWindow, NSWindowStyleMask, NSBackingStoreType, NSScreen, NSColor};
use cocoa::base::{id, nil, YES, NO};
use cocoa::foundation::{NSAutoreleasePool, NSRect};
use objc::runtime::{Class, Object, Sel};
use objc::declare::ClassDecl;
use objc::{msg_send, sel};
use objc::sel_impl;
use core_graphics::geometry::{CGRect, CGPoint, CGSize};
use core_graphics::context::{CGContext, CGContextRef};
use core_graphics::color::CGColor;


fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular);

        let screen = NSScreen::mainScreen(nil);
        let screen_frame = NSScreen::frame(screen);


        let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
            screen_frame,
            NSWindowStyleMask::NSBorderlessWindowMask,
            NSBackingStoreType::NSBackingStoreBuffered,
            NO,
        );

        window.setOpaque_(NO);
        // window.setBackgroundColor_(NSColor::clearColor(nil));
        // window.setBackgroundColor_(NSColor::blueComponent());
        window.setLevel_(0);
        window.setIgnoresMouseEvents_(YES);
        window.setCollectionBehavior_(cocoa::appkit::NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces);
        window.makeKeyAndOrderFront_(nil);

        let overlay_view = create_overlay_view(screen_frame);
        window.setContentView_(overlay_view);

        println!("is running");
        app.run();
    }
}

fn create_overlay_view(frame: NSRect) -> id {
    let superclass = Class::get("NSView").unwrap();
    let mut decl = ClassDecl::new("OverlayView", superclass).unwrap();

    unsafe {
        decl.add_method(sel!(drawRect:), draw_rect as extern "C" fn(&Object, Sel, NSRect));
    }

    let class = decl.register();
    let overlay_view: id = unsafe { msg_send![class, alloc] };
    let overlay_view: id = unsafe { msg_send![overlay_view, initWithFrame:frame] };
    overlay_view
}

extern "C" fn draw_rect(_self: &Object, _cmd: Sel, rect: NSRect) {
    unsafe {
        let context: id = msg_send![Class::get("NSGraphicsContext").unwrap(), currentContext];
        let cg_context: CGContextRef = msg_send![context, CGContext];

        let color: id = msg_send![Class::get("NSColor").unwrap(), redColor];
        let cg_color: *const core_graphics::color::CGColor = msg_send![color, CGColor];
        CGContextSetStrokeColorWithColor(&cg_context, cg_color);
        CGContextSetLineWidth(&cg_context, 5.0);
        CGContextStrokeRect(&cg_context, CGRect::new(
            &CGPoint::new(2.5, 2.5),
            &CGSize::new(rect.size.width - 5.0, rect.size.height - 5.0)
        ));
    }
}

extern "C" {
    fn CGContextSetStrokeColorWithColor(context: &CGContextRef, color: *const core_graphics::color::CGColor);
    fn CGContextSetLineWidth(context: &CGContextRef, width: f64);
    fn CGContextStrokeRect(context: &CGContextRef, rect: CGRect);
}