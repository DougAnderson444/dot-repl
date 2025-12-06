//! Graphviz SVG → Dioxus renderer with optional "rough" hand-drawn styling.
//!
//! When `rough` feature is enabled, shapes and paths can be converted to hand-drawn versions using `roughr`.
use super::fonts::{ARCHITECTS_DAUGHTER_CSS, ARCHITECTS_DAUGHTER_FAMILY};
use dioxus::logger::tracing;
use dioxus::prelude::*;
use dioxus_router::Navigator;
use roxmltree::{Document, Node};
use std::borrow::Cow;

#[cfg(feature = "rough")]
use std::fmt::Display;

#[cfg(feature = "rough")]
use roughr::{
    core::{FillStyle, OpSetType, Options, OptionsBuilder},
    generator::Generator,
};

#[derive(Debug, Clone, PartialEq)]
pub enum LinkKind {
    Internal(String),
    External(String),
    Fragment(String),
    None,
}

const XLINK_NS: &str = "http://www.w3.org/1999/xlink";
const XML_NS: &str = "http://www.w3.org/XML/1998/namespace";

// Default opacity for hachure strokes when source color has no alpha
const HATCH_DEFAULT_OPACITY: f32 = 0.95;

// ------------------------- Config -------------------------

#[derive(Clone)]
pub struct SvgBuildConfig {
    pub classify_link: fn(&str) -> LinkKind,
    pub map_internal_route: Option<fn(&str) -> Option<String>>,
    pub on_fragment_click: Option<fn(&str)>,
    pub on_title: Option<fn(&str)>,
    pub strip_doctype: bool,
    pub rough_style: bool,
    pub scale_to_fit: bool,
    pub rough_options: RoughOptions,
    pub rough_use_custom_font: bool,
    pub rough_embed_font_data: Option<&'static str>,
}

#[derive(Clone, PartialEq)]
pub struct RoughOptions {
    pub roughness: f32,
    pub bowing: f32,
    pub fill_style: RoughFillStyle,
}

#[derive(Clone, PartialEq)]
pub enum RoughFillStyle {
    Solid,
    Hachure,
    ZigZag,
    CrossHatch,
    Dots,
    Dashed,
    ZigZagLine,
}

impl Default for RoughOptions {
    fn default() -> Self {
        Self {
            roughness: 1.0,
            bowing: 1.0,
            fill_style: RoughFillStyle::Hachure,
        }
    }
}

impl PartialEq for SvgBuildConfig {
    fn eq(&self, other: &Self) -> bool {
        // Compare the fields that should trigger a re-render
        self.rough_style == other.rough_style
            && self.strip_doctype == other.strip_doctype
            && self.scale_to_fit == other.scale_to_fit
            && self.rough_options == other.rough_options
            && self.rough_use_custom_font == other.rough_use_custom_font
    }
}

impl Default for SvgBuildConfig {
    fn default() -> Self {
        SvgBuildConfig {
            classify_link: |href: &str| {
                if let Some(rest) = href.strip_prefix('#') {
                    LinkKind::Fragment(rest.to_string())
                } else if href.starts_with("http://") || href.starts_with("https://") {
                    LinkKind::External(href.to_string())
                } else
                // include dioxus://index.html/graphviz/ZGlncmFwaC...
                if href.starts_with('/') {
                    LinkKind::Internal(href.to_string())
                } else {
                    LinkKind::None
                }
            },
            map_internal_route: None,
            on_fragment_click: None,
            on_title: None,
            strip_doctype: true,
            rough_style: true,
            scale_to_fit: false,
            rough_options: RoughOptions::default(),
            rough_use_custom_font: true,
            rough_embed_font_data: None,
        }
    }
}

// ------------------------- Attribute collection -------------------------

#[derive(Default, Clone)]
struct SvgAttrs {
    id: Option<String>,
    class: Option<String>,
    style: Option<String>,
    transform: Option<String>,
    fill: Option<String>,
    stroke: Option<String>,
    stroke_width: Option<String>,
    stroke_dasharray: Option<String>,
    font_size: Option<String>,
    font_family: Option<String>,
    font_weight: Option<String>,
    text_anchor: Option<String>,
    xml_space: Option<String>,

    x: Option<String>,
    y: Option<String>,
    dx: Option<String>,
    dy: Option<String>,
    cx: Option<String>,
    cy: Option<String>,
    rx: Option<String>,
    ry: Option<String>,
    r: Option<String>,
    width: Option<String>,
    height: Option<String>,
    d: Option<String>,
    points: Option<String>,
    view_box: Option<String>,

    href: Option<String>,
    xlink_href: Option<String>,
    xlink_title: Option<String>,
    target: Option<String>,
    rel: Option<String>,

    extra: Vec<(String, String)>,
}

fn collect_attrs(node: Node) -> SvgAttrs {
    let mut sa = SvgAttrs::default();
    for a in node.attributes() {
        let ns = a.namespace();
        let local = a.name();
        let value = a.value().to_string();
        match (ns, local) {
            (Some(XLINK_NS), "href") => sa.xlink_href = Some(value),
            (Some(XLINK_NS), "title") => sa.xlink_title = Some(value),
            (Some(XML_NS), "space") => sa.xml_space = Some(value),

            (None, "id") => sa.id = Some(value),
            (None, "class") => sa.class = Some(value),
            (None, "style") => sa.style = Some(value),
            (None, "transform") => sa.transform = Some(value),
            (None, "fill") => sa.fill = Some(value),
            (None, "stroke") => sa.stroke = Some(value),
            (None, "stroke-width") => sa.stroke_width = Some(value),
            (None, "stroke-dasharray") => sa.stroke_dasharray = Some(value),
            (None, "font-size") => sa.font_size = Some(value),
            (None, "font-family") => sa.font_family = Some(value),
            (None, "font-weight") => sa.font_weight = Some(value),
            (None, "text-anchor") => sa.text_anchor = Some(value),

            (None, "x") => sa.x = Some(value),
            (None, "y") => sa.y = Some(value),
            (None, "dx") => sa.dx = Some(value),
            (None, "dy") => sa.dy = Some(value),
            (None, "cx") => sa.cx = Some(value),
            (None, "cy") => sa.cy = Some(value),
            (None, "rx") => sa.rx = Some(value),
            (None, "ry") => sa.ry = Some(value),
            (None, "r") => sa.r = Some(value),
            (None, "width") => sa.width = Some(value),
            (None, "height") => sa.height = Some(value),
            (None, "d") => sa.d = Some(value),
            (None, "points") => sa.points = Some(value),
            (None, "viewBox") => sa.view_box = Some(value),

            (None, "href") => sa.href = Some(value),
            (None, "target") => sa.target = Some(value),
            (None, "rel") => sa.rel = Some(value),

            _ => {
                let key = match ns {
                    Some(ns_uri) => format!("{ns_uri}:{local}"),
                    None => local.to_string(),
                };
                sa.extra.push((key, value));
            }
        }
    }
    sa
}

// ------------------------- Color helpers for opacity -------------------------

fn format_opacity(v: f32) -> String {
    // Trim to at most 3 decimals and strip trailing zeros/dot.
    let s = format!("{:.3}", v.clamp(0.0, 1.0));
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s.is_empty() {
        "0".into()
    } else {
        s.into()
    }
}

/// Normalize a color into (#RRGGBB, Some(opacity)) if it is #RRGGBBAA; pass through otherwise.
/// Returns (normalized_color, optional_opacity_string).
fn normalize_hex_color_with_opacity(input: &Option<String>) -> (Option<String>, Option<String>) {
    let Some(raw) = input.as_ref() else {
        return (None, None);
    };
    let s = raw.trim();
    if !s.starts_with('#') {
        return (Some(raw.clone()), None);
    }
    let hex = &s[1..];
    match hex.len() {
        6 => (Some(raw.clone()), None),
        8 => {
            if let (Ok(a), Ok(_)) = (
                u8::from_str_radix(&hex[6..8], 16),
                u32::from_str_radix(&hex[0..6], 16),
            ) {
                let rgb = format!("#{}", &hex[0..6]);
                let op = (a as f32) / 255.0;
                (Some(rgb), Some(format_opacity(op)))
            } else {
                (Some(raw.clone()), None)
            }
        }
        _ => (Some(raw.clone()), None),
    }
}

// ------------------------- Rough conversion core -------------------------

#[cfg(feature = "rough")]
fn map_fill_style(fs: &RoughFillStyle) -> FillStyle {
    match fs {
        RoughFillStyle::Solid => FillStyle::Solid,
        RoughFillStyle::Hachure => FillStyle::Hachure,
        RoughFillStyle::ZigZag => FillStyle::ZigZag,
        RoughFillStyle::CrossHatch => FillStyle::CrossHatch,
        RoughFillStyle::Dots => FillStyle::Dots,
        RoughFillStyle::Dashed => FillStyle::Dashed,
        RoughFillStyle::ZigZagLine => FillStyle::ZigZagLine,
    }
}

#[cfg(feature = "rough")]
fn parse_color_to_srgba(s: &str) -> Option<roughr::Srgba> {
    // Accept #RRGGBB or #RRGGBBAA
    let hex = s.trim();
    let hex = hex.strip_prefix('#')?;
    let (r, g, b, a) = match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            (r, g, b, 255)
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            (r, g, b, a)
        }
        _ => return None,
    };
    Some(roughr::Srgba::from_components((
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    )))
}

/// Attenuate the alpha channel of a color by a factor (0.0 to 1.0)
#[cfg(feature = "rough")]
fn attenuate_alpha(c: roughr::Srgba, factor: f32) -> roughr::Srgba {
    let (r, g, b, a) = c.into_components();
    roughr::Srgba::from_components((r, g, b, a * factor))
}

/// Build roughr::Options from SVG attributes and config
#[cfg(feature = "rough")]
fn build_rough_options_from_attrs(attrs: &SvgAttrs, cfg: &SvgBuildConfig) -> Option<Options> {
    let mut builder = OptionsBuilder::default();
    builder.roughness(cfg.rough_options.roughness);
    builder.bowing(cfg.rough_options.bowing);
    builder.fill_style(map_fill_style(&cfg.rough_options.fill_style));

    if let Some(stroke) = attrs.stroke.as_ref().and_then(|s| parse_color_to_srgba(s)) {
        builder.stroke(stroke);
    }
    if let Some(mut fill) = attrs.fill.as_ref().and_then(|s| parse_color_to_srgba(s)) {
        if cfg.rough_style {
            // Slight attenuation to reduce visual density under text
            fill = attenuate_alpha(fill, 1.0);
        }
        builder.fill(fill);
    }
    if let Some(sw) = attrs
        .stroke_width
        .as_ref()
        .and_then(|s| s.parse::<f32>().ok())
    {
        builder.stroke_width(sw);
    }

    builder.build().ok()
}

#[cfg(feature = "rough")]
fn drawable_to_path_elements<F: num_traits::Float + euclid::Trig + Display>(
    drawable: &roughr::core::Drawable<F>,
    original_attrs: &SvgAttrs,
) -> Vec<Element> {
    // Precompute normalized colors and opacities
    let (fill_color_norm, fill_opacity_attr) =
        normalize_hex_color_with_opacity(&original_attrs.fill);
    let (stroke_color_norm, stroke_opacity_attr) =
        normalize_hex_color_with_opacity(&original_attrs.stroke);

    // Choose hatch color: prefer fill color (represents fill pattern); fallback to stroke; then black
    let hatch_color = fill_color_norm
        .clone()
        .or_else(|| stroke_color_norm.clone())
        .unwrap_or_else(|| "#000".to_string());

    // Hatch opacity: prefer fill's alpha; fallback to default; allow stroke alpha if no fill alpha
    let hatch_opacity = fill_opacity_attr
        .clone()
        .or_else(|| stroke_opacity_attr.clone())
        .unwrap_or_else(|| format_opacity(HATCH_DEFAULT_OPACITY));

    let mut out = Vec::new();
    for set in &drawable.sets {
        // Build path 'd'
        let mut d_buf = String::new();
        for op in &set.ops {
            use roughr::core::OpType;
            match op.op {
                OpType::Move => {
                    if op.data.len() >= 2 {
                        d_buf.push_str(&format!("M{},{} ", op.data[0], op.data[1]));
                    }
                }
                OpType::LineTo => {
                    if op.data.len() >= 2 {
                        d_buf.push_str(&format!("L{},{} ", op.data[0], op.data[1]));
                    }
                }
                OpType::BCurveTo => {
                    if op.data.len() >= 6 {
                        d_buf.push_str(&format!(
                            "C{},{} {},{} {},{} ",
                            op.data[0], op.data[1], op.data[2], op.data[3], op.data[4], op.data[5]
                        ));
                    }
                }
            }
        }
        let d_str = d_buf.trim().to_string();

        match set.op_set_type {
            OpSetType::Path => {
                // Outline stroke path
                out.push(rsx! {
                    path {
                        id: original_attrs.id.clone(),
                        class: original_attrs.class.clone(),
                        d: d_str,
                        stroke: stroke_color_norm.clone(),
                        "stroke-width": original_attrs.stroke_width.clone(),
                        fill: "none",
                        // apply stroke-opacity if stroke color had alpha
                        "stroke-opacity": stroke_opacity_attr.clone(),
                        style: original_attrs.style.clone(),
                        "data-rough-segment": "stroke"
                    }
                });
            }
            OpSetType::FillPath => {
                // Base fill area
                out.push(rsx! {
                    path {
                        id: original_attrs.id.clone(),
                        class: original_attrs.class.clone(),
                        d: d_str,
                        fill: fill_color_norm.clone().or_else(|| Some("none".into())),
                        stroke: "none",
                        // apply fill-opacity if fill color had alpha
                        "fill-opacity": fill_opacity_attr.clone(),
                        style: original_attrs.style.clone(),
                        "data-rough-segment": "fill"
                    }
                });
            }
            OpSetType::FillSketch => {
                // Hatching strokes — use stroke-opacity for legibility
                out.push(rsx! {
                    path {
                        id: original_attrs.id.clone(),
                        class: original_attrs.class.clone(),
                        d: d_str,
                        stroke: hatch_color.clone(),
                        "stroke-width": original_attrs.stroke_width.clone(),
                        "stroke-opacity": hatch_opacity.clone(),
                        fill: "none",
                        style: original_attrs.style.clone(),
                        "data-rough-segment": "hatch"
                    }
                });
            }
        }
    }
    out
}

#[cfg(feature = "rough")]
fn rough_path(attrs: &SvgAttrs, cfg: &SvgBuildConfig) -> Option<Vec<Element>> {
    let d = attrs.d.as_ref()?;
    let options = build_rough_options_from_attrs(attrs, cfg)?;
    let gen = Generator::default();
    let drawable = gen.path::<f32>(d.clone(), &Some(options));
    Some(drawable_to_path_elements(&drawable, attrs))
}

#[cfg(feature = "rough")]
fn rough_rect(attrs: &SvgAttrs, cfg: &SvgBuildConfig) -> Option<Vec<Element>> {
    let x = attrs.x.as_ref()?.parse::<f32>().ok()?;
    let y = attrs.y.as_ref()?.parse::<f32>().ok()?;
    let w = attrs.width.as_ref()?.parse::<f32>().ok()?;
    let h = attrs.height.as_ref()?.parse::<f32>().ok()?;
    let options = build_rough_options_from_attrs(attrs, cfg)?;
    let gen = Generator::default();
    let drawable = gen.rectangle::<f32>(x, y, w, h, &Some(options));
    Some(drawable_to_path_elements(&drawable, attrs))
}

#[cfg(feature = "rough")]
fn rough_circle(attrs: &SvgAttrs, cfg: &SvgBuildConfig) -> Option<Vec<Element>> {
    let cx = attrs.cx.as_ref()?.parse::<f32>().ok()?;
    let cy = attrs.cy.as_ref()?.parse::<f32>().ok()?;
    let r = attrs.r.as_ref()?.parse::<f32>().ok()?;
    let diameter = r * 2.0;
    let options = build_rough_options_from_attrs(attrs, cfg)?;
    let gen = Generator::default();
    let drawable = gen.circle::<f32>(cx, cy, diameter, &Some(options));
    Some(drawable_to_path_elements(&drawable, attrs))
}

#[cfg(feature = "rough")]
fn rough_ellipse(attrs: &SvgAttrs, cfg: &SvgBuildConfig) -> Option<Vec<Element>> {
    let cx = attrs.cx.as_ref()?.parse::<f32>().ok()?;
    let cy = attrs.cy.as_ref()?.parse::<f32>().ok()?;
    let rx = attrs.rx.as_ref()?.parse::<f32>().ok()?;
    let ry = attrs.ry.as_ref()?.parse::<f32>().ok()?;
    let options = build_rough_options_from_attrs(attrs, cfg)?;
    let gen = Generator::default();
    let drawable = gen.ellipse::<f32>(cx, cy, rx * 2.0, ry * 2.0, &Some(options));
    Some(drawable_to_path_elements(&drawable, attrs))
}

#[cfg(feature = "rough")]
fn rough_polygon(attrs: &SvgAttrs, cfg: &SvgBuildConfig) -> Option<Vec<Element>> {
    let pts_str = attrs.points.as_ref()?;
    let mut points = Vec::new();
    for pair in pts_str.split_whitespace() {
        let (x, y) = pair.split_once(',')?;
        let px = x.parse::<f32>().ok()?;
        let py = y.parse::<f32>().ok()?;
        points.push(roughr::Point2D::new(px, py));
    }
    if points.len() < 3 {
        return None;
    }
    let options = build_rough_options_from_attrs(attrs, cfg)?;
    let gen = Generator::default();
    let drawable = gen.polygon::<f32>(&points, &Some(options));
    Some(drawable_to_path_elements(&drawable, attrs))
}

// ------------------------- Non-rough fallback for path processing -------------------------

#[cfg(not(feature = "rough"))]
fn rough_path(_attrs: &SvgAttrs, _cfg: &SvgBuildConfig) -> Option<Vec<Element>> {
    None
}
#[cfg(not(feature = "rough"))]
fn rough_rect(_attrs: &SvgAttrs, _cfg: &SvgBuildConfig) -> Option<Vec<Element>> {
    None
}
#[cfg(not(feature = "rough"))]
fn rough_circle(_attrs: &SvgAttrs, _cfg: &SvgBuildConfig) -> Option<Vec<Element>> {
    None
}
#[cfg(not(feature = "rough"))]
fn rough_ellipse(_attrs: &SvgAttrs, _cfg: &SvgBuildConfig) -> Option<Vec<Element>> {
    None
}
#[cfg(not(feature = "rough"))]
fn rough_polygon(_attrs: &SvgAttrs, _cfg: &SvgBuildConfig) -> Option<Vec<Element>> {
    None
}

// ------------------------- DTD strip -------------------------

fn strip_doctype(raw: &str) -> Cow<'_, str> {
    if !raw.contains("<!DOCTYPE") {
        return Cow::Borrowed(raw);
    }
    let mut out = String::with_capacity(raw.len());
    let mut i = 0;
    let b = raw.as_bytes();
    while i < b.len() {
        if b[i] == b'<' && raw[i..].starts_with("<!DOCTYPE") {
            i += "<!DOCTYPE".len();
            while i < b.len() && b[i] != b'>' {
                i += 1;
            }
            if i < b.len() {
                i += 1;
            }
            while i < b.len() && matches!(b[i], b'\n' | b'\r') {
                i += 1;
            }
        } else {
            out.push(b[i] as char);
            i += 1;
        }
    }
    Cow::Owned(out)
}

// ------------------------- Component -------------------------

#[component]
pub fn GraphvizSvg(svg_text: String, config: SvgBuildConfig) -> Element {
    let navigator = use_navigator();

    let mut cow: Cow<'_, str> = if config.strip_doctype {
        strip_doctype(&svg_text)
    } else {
        Cow::Borrowed(svg_text.as_str())
    };

    let doc = loop {
        match Document::parse(&cow) {
            Ok(d) => break d,
            Err(e) => {
                let did_strip = !matches!(cow, Cow::Borrowed(_));
                if !did_strip && svg_text.contains("<!DOCTYPE") {
                    cow = strip_doctype(&svg_text);
                    continue;
                } else {
                    return render_parse_error(e, did_strip || config.strip_doctype);
                }
            }
        }
    };

    let Some(root) = doc.descendants().find(|n| n.has_tag_name("svg")) else {
        return rsx! { svg { class: "graphviz-svg error", "No <svg> root found." } };
    };

    build_node(root, &config, navigator).unwrap_or(rsx! {})
}

fn render_parse_error(err: roxmltree::Error, did_strip: bool) -> Element {
    rsx! {
        svg {
            class: "graphviz-svg error",
            style: "padding:8px;font-family:monospace;font-size:12px;fill:#900;",
            "SVG parse error (strip_doctype={did_strip}): {err}"
        }
    }
}

// ------------------------- Recursive build -------------------------

fn build_node(node: Node, cfg: &SvgBuildConfig, navigator: Navigator) -> Option<Element> {
    if node.is_text() {
        let t = node.text().unwrap_or_default();
        if t.trim().is_empty() {
            return None;
        }
        return Some(rsx! { "{t}" });
    }
    if !node.is_element() {
        return None;
    }

    let tag = node.tag_name().name();
    let attrs = collect_attrs(node);
    let children: Vec<Element> = node
        .children()
        .filter_map(|c| build_node(c, cfg, navigator))
        .collect();

    let arch_daughter = r#"@import url('https://fonts.googleapis.com/css2?family=Architects+Daughter&display=swap');
@import url('https://fonts.googleapis.com/css2?family=Noto+Sans+Symbols+2&display=swap');
svg, text, tspan {
  font-family: 'Architects Daughter','Noto Sans Symbols 2','Noto Sans',sans-serif;
}"#;

    let custom_style = if cfg.rough_style && cfg.rough_use_custom_font {
        if let Some(css) = cfg.rough_embed_font_data {
            format!("{css}\nsvg, text, tspan {{ font-family: {ARCHITECTS_DAUGHTER_FAMILY}; }}")
        } else {
            format!(
                "{ARCHITECTS_DAUGHTER_CSS}\nsvg, text, tspan {{ font-family: {ARCHITECTS_DAUGHTER_FAMILY}; }}"
            )
        }
    } else {
        String::new()
    };

    let el = match tag {
        "svg" => {
            let (width, height) = if cfg.scale_to_fit {
                (Some("100%".to_string()), Some("100%".to_string()))
            } else {
                (attrs.width, attrs.height)
            };

            rsx! {
                svg {
                    id: attrs.id,
                    class: attrs.class,
                    width: width,
                    height: height,
                    view_box: attrs.view_box,
                    style: attrs.style,
                    "xmlns": "http://www.w3.org/2000/svg",
                    "xmlns:xlink": XLINK_NS,
                    style { {custom_style} }
                    for child in children { {child} }
                }
            }
        }
        "g" => rsx! {
            g {
                id: attrs.id,
                class: attrs.class,
                transform: attrs.transform,
                style: attrs.style,
                for child in children { {child} }
            }
        },
        "text" => rsx! {
            text {
                id: attrs.id,
                class: attrs.class,
                x: attrs.x,
                y: attrs.y,
                dx: attrs.dx,
                dy: attrs.dy,
                fill: attrs.fill,
                "font-size": attrs.font_size,
                "font-family": attrs.font_family,
                "font-weight": attrs.font_weight,
                "text-anchor": attrs.text_anchor,
                "xml:space": attrs.xml_space,
                style: attrs.style,
                for child in children { {child} }
            }
        },
        "title" => {
            if let Some(t) = node.text() {
                if let Some(cb) = cfg.on_title {
                    cb(t);
                }
                rsx! { title { "{t}" } }
            } else {
                rsx! { title { for child in children { {child} } } }
            }
        }
        "path" => {
            if cfg.rough_style {
                if let Some(segments) = rough_path(&attrs, cfg) {
                    rsx! { g { for seg in segments { {seg} } } }
                } else {
                    default_path(&attrs)
                }
            } else {
                default_path(&attrs)
            }
        }
        "rect" => {
            if cfg.rough_style {
                if let Some(segments) = rough_rect(&attrs, cfg) {
                    rsx! { g { for seg in segments { {seg} } for child in children { {child} } } }
                } else {
                    default_rect(&attrs, &children)
                }
            } else {
                default_rect(&attrs, &children)
            }
        }
        "circle" => {
            if cfg.rough_style {
                if let Some(segments) = rough_circle(&attrs, cfg) {
                    rsx! { g { for seg in segments { {seg} } } }
                } else {
                    default_circle(&attrs)
                }
            } else {
                default_circle(&attrs)
            }
        }
        "ellipse" => {
            if cfg.rough_style {
                if let Some(segments) = rough_ellipse(&attrs, cfg) {
                    rsx! { g { for seg in segments { {seg} } } }
                } else {
                    default_ellipse(&attrs)
                }
            } else {
                default_ellipse(&attrs)
            }
        }
        "polygon" => {
            if cfg.rough_style {
                if let Some(segments) = rough_polygon(&attrs, cfg) {
                    rsx! { g { for seg in segments { {seg} } } }
                } else {
                    default_polygon(&attrs)
                }
            } else {
                default_polygon(&attrs)
            }
        }
        "polyline" => rsx! {
            polyline {
                id: attrs.id,
                class: attrs.class,
                points: attrs.points,
                fill: attrs.fill,
                stroke: attrs.stroke,
                "stroke-width": attrs.stroke_width,
                "stroke-dasharray": attrs.stroke_dasharray,
                style: attrs.style,
            }
        },
        "a" => build_anchor(attrs, children, cfg, navigator),
        _ => {
            rsx! {
                g {
                    id: attrs.id,
                    class: attrs.class,
                    style: attrs.style,
                    "data-unknown-tag": tag,
                    for child in children { {child} }
                }
            }
        }
    };

    Some(el)
}

// ------------------------- Fallback element builders -------------------------

fn default_path(attrs: &SvgAttrs) -> Element {
    rsx! {
        path {
            id: attrs.id.clone(),
            class: attrs.class.clone(),
            d: attrs.d.clone(),
            fill: attrs.fill.clone(),
            stroke: attrs.stroke.clone(),
            "stroke-width": attrs.stroke_width.clone(),
            "stroke-dasharray": attrs.stroke_dasharray.clone(),
            style: attrs.style.clone(),
        }
    }
}

fn default_rect(attrs: &SvgAttrs, children: &[Element]) -> Element {
    rsx! {
        rect {
            id: attrs.id.clone(),
            class: attrs.class.clone(),
            x: attrs.x.clone(),
            y: attrs.y.clone(),
            width: attrs.width.clone(),
            height: attrs.height.clone(),
            rx: attrs.rx.clone(),
            ry: attrs.ry.clone(),
            fill: attrs.fill.clone(),
            stroke: attrs.stroke.clone(),
            "stroke-width": attrs.stroke_width.clone(),
            "stroke-dasharray": attrs.stroke_dasharray.clone(),
            style: attrs.style.clone(),
            for child in children { {child.clone()} }
        }
    }
}

fn default_circle(attrs: &SvgAttrs) -> Element {
    rsx! {
        circle {
            id: attrs.id.clone(),
            class: attrs.class.clone(),
            cx: attrs.cx.clone(),
            cy: attrs.cy.clone(),
            r: attrs.r.clone(),
            fill: attrs.fill.clone(),
            stroke: attrs.stroke.clone(),
            "stroke-width": attrs.stroke_width.clone(),
            "stroke-dasharray": attrs.stroke_dasharray.clone(),
            style: attrs.style.clone(),
        }
    }
}

fn default_ellipse(attrs: &SvgAttrs) -> Element {
    rsx! {
        ellipse {
            id: attrs.id.clone(),
            class: attrs.class.clone(),
            cx: attrs.cx.clone(),
            cy: attrs.cy.clone(),
            rx: attrs.rx.clone(),
            ry: attrs.ry.clone(),
            fill: attrs.fill.clone(),
            stroke: attrs.stroke.clone(),
            "stroke-width": attrs.stroke_width.clone(),
            "stroke-dasharray": attrs.stroke_dasharray.clone(),
            style: attrs.style.clone(),
        }
    }
}

fn default_polygon(attrs: &SvgAttrs) -> Element {
    rsx! {
        polygon {
            id: attrs.id.clone(),
            class: attrs.class.clone(),
            points: attrs.points.clone(),
            fill: attrs.fill.clone(),
            stroke: attrs.stroke.clone(),
            "stroke-width": attrs.stroke_width.clone(),
            "stroke-dasharray": attrs.stroke_dasharray.clone(),
            style: attrs.style.clone(),
        }
    }
}

// ------------------------- Anchor -------------------------

fn build_anchor(
    a: SvgAttrs,
    children: Vec<Element>,
    cfg: &SvgBuildConfig,
    navigator: Navigator,
) -> Element {
    let mut effective_href = a.href.clone().or(a.xlink_href.clone());

    if let Some(mapper) = cfg.map_internal_route.as_ref() {
        if let Some(href) = &effective_href {
            if let Some(mapped) = mapper(href) {
                effective_href = Some(mapped);
            }
        }
    }

    let tooltip_node = a.xlink_title.as_ref().map(|t| rsx! { title { "{t}" } });

    match effective_href {
        Some(href) => {
            let kind = (cfg.classify_link)(&href);
            match kind {
                LinkKind::External(url) => {
                    let url_owned = url.clone();
                    rsx! {
                        g {
                            id: a.id,
                            class: a.class,
                            style: a.style,
                            "data-link-type": "external",
                            "data-href": "{url}",
                            cursor: "pointer",
                            onclick: move |evt| {
                                evt.prevent_default();
                                tracing::info!("External link clicked, navigating to {}", url_owned);
                                #[cfg(feature = "desktop")]
                                {
                                    if let Err(e) = dioxus::desktop::use_window().webview.load_url(&url_owned) {
                                        tracing::error!("Failed to navigate to {}: {}", url_owned, e);
                                    }
                                }
                                #[cfg(target_arch = "wasm32")]
                                {
                                    let url = url_owned.clone();
                                    let _ = web_sys::window()
                                        .and_then(|w| w.open_with_url_and_target(&url_owned, "_blank").ok())
                                        .flatten();
                                }
                            },
                            { tooltip_node }
                            for child in children { {child} }
                        }
                    }
                }
                LinkKind::Internal(route) => {
                    let route_owned = route.clone();
                    rsx! {
                        g {
                            id: a.id,
                            class: a.class,
                            style: a.style,
                            "data-link-type": "internal",
                            "data-href": "{route}",
                            cursor: "pointer",
                            onclick: {
                                move |evt| {
                                    tracing::info!("Internal link clicked, navigating to {}", route_owned);
                                    evt.prevent_default();
                                    navigator.push(route_owned.as_str());
                                }
                            },
                            { tooltip_node }
                            for child in children { {child} }
                        }
                    }
                }
                LinkKind::Fragment(id) => {
                    let id_owned = id.clone();
                    let cb = cfg.on_fragment_click;
                    rsx! {
                        g {
                            id: a.id,
                            class: a.class,
                            style: a.style,
                            "data-link-type": "fragment",
                            "data-href": "#{id}",
                            cursor: "pointer",
                            onclick: move |evt| {
                                evt.prevent_default();
                                tracing::info!("Fragment link clicked: #{}", id_owned);
                                if let Some(f) = cb {
                                    f(&id_owned);
                                }
                            },
                            { tooltip_node }
                            for child in children { {child} }
                        }
                    }
                }
                LinkKind::None => {
                    tracing::warn!("LinkKind::None for href: {}", href);
                    rsx! {
                        g {
                            id: a.id,
                            class: a.class,
                            style: a.style,
                            { tooltip_node }
                            for child in children { {child} }
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                g {
                    id: a.id,
                    class: a.class,
                    style: a.style,
                    { tooltip_node }
                    for child in children { {child} }
                }
            }
        }
    }
}
