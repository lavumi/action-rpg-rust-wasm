use std::fs;
use std::sync::Arc;

use log::info;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FrameSize {
    x: Option<u16>,
    y: Option<u16>,
    w: u16,
    h: u16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FrameData {
    // filename : String,
    // rotated : bool,
    // trimmed : bool,
    frame: FrameSize,
    sprite_source_size: FrameSize,
    source_size: FrameSize,
    duration: i16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FrameTag {
    name: String,
    from: usize,
    to: usize,
    direction: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MetaData {
    size: FrameSize,
    frame_tags: Vec<FrameTag>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AnimationJsonData {
    frames: Vec<FrameData>,
    meta: MetaData,
}


pub struct AnimationData {
    pub uv: Vec<[f32; 4]>,
    pub dt: Vec<f32>,
}

pub struct AnimationDataHandler {
    // pub character_animations_hashmap: HashMap<String, AnimationData>,
    pub character_animations: Vec<Arc<AnimationData>>,
}

impl Default for AnimationDataHandler {
    fn default() -> Self {
        AnimationDataHandler {
            // character_animations_hashmap: Default::default(),
            character_animations: Default::default()
        }
    }
}


impl AnimationDataHandler {
    pub fn init(&mut self) {
        let str = fs::read_to_string("./assets/character/06.json").expect("Unable to read file");
        let data: AnimationJsonData = serde_json::from_str(&str).expect("JSON was not well-formatted");

        let atlas_size = [data.meta.size.w as f32, data.meta.size.h as f32];
        for frame_tag in data.meta.frame_tags {
            let mut animation_data = AnimationData {
                uv: vec![],
                dt: vec![],
            };

            for i in frame_tag.from..frame_tag.to {
                let start_x = data.frames[i].frame.x.unwrap() as f32 / atlas_size[0];
                let start_y = data.frames[i].frame.y.unwrap() as f32 / atlas_size[1];
                let end_x = (data.frames[i].frame.x.unwrap() + data.frames[i].frame.w) as f32 / atlas_size[0];
                let end_y = (data.frames[i].frame.y.unwrap() + data.frames[i].frame.h) as f32 / atlas_size[1];
                animation_data.uv.push([
                    start_x, end_x, start_y, end_y
                ]);
                animation_data.dt.push(data.frames[i].duration as f32 / 1000.0);
            }
            self.character_animations.push(Arc::from(animation_data));
        }
        info!("load animation data success");
    }
}